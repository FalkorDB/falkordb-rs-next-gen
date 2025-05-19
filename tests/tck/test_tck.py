import os
import platform
import subprocess

import redis
from behave.__main__ import main as behave_main

redis_server = None
client = redis.Redis(protocol=3)
shutdown = False


def query(query):
    return client.execute_command("GRAPH.QUERY", "x", query)


def setup_module(module):
    global redis_server, shutdown
    target = os.environ.get("TARGET",
                            "target/debug/libfalkordb.dylib" if platform.system() == "Darwin" else "target/debug/libfalkordb.so")
    try:
        client.ping()
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
            client.ping()
            return
        except:
            pass


def teardown_module(module):
    if shutdown:
        client.shutdown(nosave=True)
        redis_server.wait()


def test_tck():
    tck_features = os.getenv("TCK_FEATURES", "./tests/tck/features/")
    cmd = [tck_features, '--tags=-crash', '--tags=-skip', "--no-capture"]
    tck_include = os.getenv("TCK_INCLUDE", "")
    if not tck_include:
        tck_done_file = os.getenv("TCK_DONE", "")
        if tck_done_file and os.path.exists(tck_done_file):
            with open(tck_done_file, "r") as file:
                tck_include = "|".join(line.strip() for line in file if line.strip())
    if tck_include:
        cmd = [tck_features, '--tags=-crash', '--tags=-skip', "--no-capture", "--stop", "--include", tck_include]
    res = behave_main(cmd)
    res = 'pass' if res == 0 else 'fail'
    assert res == 'pass'
