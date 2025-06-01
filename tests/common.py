import os
import platform
import subprocess

from falkordb import FalkorDB
from redis import Redis

redis_server = None
client = None
g = None
shutdown = False


def start_redis():
    global redis_server, client, g, shutdown
    port = os.environ.get("PORT", "6379")
    target = os.environ.get("TARGET",
                            "target/debug/libfalkordb.dylib" if platform.system() == "Darwin" else "target/debug/libfalkordb.so")
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