import os
import platform
import subprocess

from falkordb import FalkorDB
from redis import Redis

redis_server = None
client = None
g = None
shutdown = False


def start_redis(release=False):
    global redis_server, client, g, shutdown
    port = os.environ.get("PORT", "6379")
    default_target = "target/debug/libfalkordb.so"
    if platform.system() == "Darwin":
        default_target = default_target.replace(".so", ".dylib")
    if release:
        default_target = default_target.replace("debug", "release")
    target = os.environ.get("TARGET", default_target)
    r = Redis(port=port)
    try:
        r.ping()
        client = FalkorDB(port=port)
        g = client.select_graph("test")
        return
    except:
        shutdown = True
        if os.path.exists("redis-test.log"):
            os.remove("redis-test.log")
        redis_server = subprocess.Popen(executable="/usr/local/bin/redis-server",
                                        args=["--save", "", "--port", port, "--logfile", "redis-test.log",
                                              "--loadmodule", target],
                                        stdout=subprocess.PIPE)
    while True:
        try:
            r.ping()
            client = FalkorDB(port=port)
            g = client.select_graph("test")
            return
        except:
            pass

def shutdown_redis():
    if shutdown:
        client.connection.shutdown(nosave=True)
        redis_server.wait()