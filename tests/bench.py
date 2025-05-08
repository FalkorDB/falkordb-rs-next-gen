import pytest
import os
import platform
import subprocess
import pytest
from falkordb import FalkorDB, Node, Edge
from redis import Redis, ResponseError

redis_server = None
client = None
g = None
shutdown = False


def setup_module(module):
    global redis_server, client, g, shutdown
    target = os.environ.get("TARGET",
                            "target/release/libfalkordb.dylib" if platform.system() == "Darwin" else "target/release/libfalkordb.so")
    r = Redis()
    try:
        r.ping()
        client = FalkorDB()
        g = client.select_graph("test")
        return
    except:
        shutdown = True
        if os.path.exists("redis-test.log"):
            os.remove("redis-test.log")
        redis_server = subprocess.Popen(executable="/usr/local/bin/redis-server",
                                        args=["--save", "", "--logfile", "redis-test.log", "--loadmodule", target],
                                        stdout=subprocess.PIPE)
    while True:
        try:
            r.ping()
            client = FalkorDB()
            g = client.select_graph("test")
            return
        except:
            pass


def teardown_module(module):
    if shutdown:
        client.connection.shutdown(nosave=True)
        redis_server.wait()


def setup_function(function):
    global g
    if g.name in client.list_graphs():
        g.delete()

def query(query: str, params=None):
    g.query(query, params)

def test_return(benchmark):
    benchmark(query, "RETURN 1")

def test_unwind(benchmark):
    benchmark(query, "UNWIND range(1, 1000000) AS x RETURN x")