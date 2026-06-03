import multiprocessing
import os
import platform
import shutil
import subprocess
import time

from falkordb import FalkorDB
from redis import Redis
from redis.retry import Retry
from redis.backoff import NoBackoff

redis_server: subprocess.Popen = None
client = None
g = None
shutdown = False


def fork_pool(processes=None):
    """Return a multiprocessing.Pool that always uses the 'fork' start method.

    macOS (and Windows) default to the 'spawn' start method, under which Pool
    workers boot a fresh interpreter and re-import the test module — that
    re-runs pytest collection inside every child and deadlocks. Linux already
    defaults to 'fork', so this is a no-op there while making the concurrency
    and MVCC suites work on macOS. The worker functions only open their own
    redis connections, so the usual fork-after-threads caveats don't apply.
    """
    return multiprocessing.get_context("fork").Pool(processes)


def start_redis(release=None, moduleEnvs=[]):
    global redis_server, client, g, shutdown
    host = os.environ.get("FALKORDB_HOST", "localhost")
    port = int(os.environ.get("FALKORDB_PORT", os.environ.get("PORT", "6379")))
    # In CI's services-container mode an external redis is already running with
    # the module loaded; spawning locally would race the port. Fail loudly instead.
    existing_env = os.environ.get("EXISTING_ENV", "").lower() == "1"
    if release is None:
        release = True if os.environ.get("RELEASE", "").lower() == "1" else False
    default_target = "target/debug/libfalkordb.so"
    if platform.system() == "Darwin":
        default_target = default_target.replace(".so", ".dylib")
    if release:
        default_target = default_target.replace("debug", "release")
    target = os.environ.get("TARGET", default_target)
    # Bounded connect timeout + no-retry policy: redis-py 7.4 defaults retry
    # ConnectionError indefinitely, which would hang the EXISTING_ENV probe
    # if the service container isn't reachable. Fail fast so the caller's
    # except branch fires within ~1s instead of after a long stall.
    r = Redis(host=host, port=port, socket_connect_timeout=1,
              retry=Retry(NoBackoff(), 0))
    try:
        r.ping()
        client = FalkorDB(host=host, port=port)
        g = client.select_graph("test")
        return
    except Exception as e:
        if existing_env:
            raise RuntimeError(
                f"EXISTING_ENV=1 but cannot reach redis at {host}:{port}: {e}"
            ) from e
        shutdown = True
        if os.path.exists("redis-test.log"):
            os.remove("redis-test.log")
        # Resolve redis-server from PATH so this works regardless of install
        # prefix: Linux/Intel Homebrew use /usr/local/bin, Apple-silicon
        # Homebrew uses /opt/homebrew/bin, and distro packages use /usr/bin.
        # REDIS_SERVER_PATH overrides for non-standard layouts.
        redis_server_bin = (
            os.environ.get("REDIS_SERVER_PATH")
            or shutil.which("redis-server")
            or "/usr/local/bin/redis-server"
        )
        redis_server = subprocess.Popen(
            [redis_server_bin,
             "--save", "", "--port", str(port), "--logfile", "redis-test.log",
             "--loadmodule", target] + moduleEnvs,
            stdout=subprocess.PIPE)
    # Bounded startup wait: if redis-server exits (e.g. the module fails to
    # load because `target` doesn't exist or has unresolved symbols) or never
    # becomes reachable, fail fast with diagnostics instead of spinning here
    # forever — an unbounded loop turns any startup error into an opaque hang.
    deadline = time.time() + 60
    while True:
        exited = redis_server.poll()
        if exited is not None:
            log = ""
            try:
                with open("redis-test.log") as f:
                    log = f.read()
            except OSError:
                pass
            raise RuntimeError(
                f"redis-server exited with code {exited} while loading module "
                f"{target!r} (exists={os.path.exists(target)}).\n"
                f"redis-test.log:\n{log}"
            )
        try:
            r.ping()
            client = FalkorDB(host=host, port=port)
            g = client.select_graph("test")
            return
        except Exception:
            if time.time() > deadline:
                raise RuntimeError(
                    f"redis-server did not become reachable on {host}:{port} "
                    f"within 60s (module {target!r}, "
                    f"exists={os.path.exists(target)})"
                )
            # Backoff so a slow redis startup doesn't peg a CPU core.
            time.sleep(0.05)

def falkordb():
    """Construct a FalkorDB client honoring FALKORDB_HOST / FALKORDB_PORT.

    Bare `FalkorDB()` defaults to localhost:6379, which silently bypasses
    the docker-services CI mode (where redis runs as a sibling container).
    Use this helper instead so EXISTING_ENV tests connect to the right place.
    """
    return FalkorDB(
        host=os.environ.get("FALKORDB_HOST", "localhost"),
        port=int(os.environ.get("FALKORDB_PORT", os.environ.get("PORT", "6379"))),
    )


def shutdown_redis():
    if shutdown:
        client.connection.shutdown(nosave=True)
        redis_server.wait()

def wait_for_indices_to_sync(graph):
    q = "CALL db.indexes() YIELD status WHERE status <> 'OPERATIONAL' RETURN count(1)"
    while True:
        result = graph.ro_query(q)
        if result.result_set[0][0] == 0:
            break
        time.sleep(0.5) # sleep 500ms