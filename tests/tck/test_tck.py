import os
import platform
import subprocess
from behave.__main__ import main as behave_main
import redis

redis_server = None
client = redis.Redis(protocol=3)

def query(query):
    return client.execute_command("GRAPH.QUERY", "x", query)

def setup_module(module):
    global redis_server
    target = os.environ.get("TARGET", "target/debug/libmatrixdb.dylib" if platform.system() == "Darwin" else "target/debug/libmatrixdb.so")
    try:
        client.ping()
        return
    except:
        if os.path.exists("redis-test.log"):
            os.remove("redis-test.log")
        redis_server = subprocess.Popen(executable="/usr/local/bin/redis-server", args=["--save", "", "--logfile", "redis-test.log", "--loadmodule", target], stdout=subprocess.PIPE)
    while True:
        try:
            client.ping()
            return
        except:
            pass


def teardown_module(module):
    client.shutdown(nosave=True)
    redis_server.wait()


def test_tck():
    cmd = ["./tests/tck/features/", '--tags=-crash', '--tags=-skip', "--no-capture", "--include", "tests/tck/features/clauses/create"]
    res = behave_main(cmd)
    res = 'pass' if res == 0 else 'fail'
    assert res == 'pass'
