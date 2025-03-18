import os
import platform
import subprocess
from falkordb import FalkorDB, Node
from redis import Redis

redis_server = None
client = None
g = None

def setup_module(module):
    global redis_server, client, g
    target = os.environ.get("TARGET", "target/debug/libmatrixdb.dylib" if platform.system() == "Darwin" else "target/debug/libmatrixdb.so")
    try:
        client.ping()
        return
    except:
        if os.path.exists("redis-test.log"):
            os.remove("redis-test.log")
        redis_server = subprocess.Popen(executable="/usr/local/bin/redis-server", args=["--save", "", "--logfile", "redis-test.log", "--loadmodule", target], stdout=subprocess.PIPE)
    r = Redis()
    while True:
        try:
            r.ping()
            client = FalkorDB()
            g = client.select_graph("test")
            return
        except:
            pass


def teardown_module(module):
    client.connection.shutdown(nosave=True)
    redis_server.wait()

def test_return_values():
    res = g.query("RETURN null")
    assert res.result_set == [[None]]

    for b in [True, False]:
        res = g.query(f"RETURN {b}")
        assert res.result_set == [[1 if b else 0]]

    for i in range(-10, 10):
        res = g.query(f"RETURN {i}")
        assert res.result_set == [[i]]

    for f in map(lambda x: x/10.0, range(-100, 100, 1)):
        res = g.query(f"RETURN {f}")
        assert res.result_set == [[f]]

    res = g.query("RETURN 'Avi'")
    assert res.result_set == [["Avi"]]

    res = g.query("RETURN []")
    assert res.result_set == [[[]]]

    res = g.query("RETURN ['Avi', [1, 2]]")
    assert res.result_set == [[["Avi", [1, 2]]]]

    res = g.query("RETURN {}")
    assert res.result_set == [[{}]]

    res = g.query("RETURN {a: 'Avi', b: 42}")
    assert res.result_set == [[{"a": "Avi", "b": 42}]]

    res = g.query("WITH 1 AS a, 'Avi' AS b RETURN b, a")
    assert res.result_set == [['Avi', 1]]

    res = g.query("WITH 1 AS a RETURN a")
    assert res.result_set == [[1]]

def test_operators():
    for a in [True, False]:
        for b in [True, False]:
            res = g.query(f"RETURN {a} AND {b}")
            assert res.result_set == [[1 if a and b else 0]]

    for a in [True, False]:
        for b in [True, False]:
            for c in [True, False]:
                res = g.query(f"RETURN {a} AND {b} AND {c}")
                assert res.result_set == [[1 if a and b and c else 0]]

    for a in [True, False]:
        for b in [True, False]:
            res = g.query(f"RETURN {a} OR {b}")
            assert res.result_set == [[1 if a or b else 0]]

    for a in [True, False]:
        for b in [True, False]:
            for c in [True, False]:
                res = g.query(f"RETURN {a} OR {b} OR {c}")
                assert res.result_set == [[1 if a or b or c else 0]]

    for a in [True, False]:
        for b in [True, False]:
            res = g.query(f"RETURN {a} = {b}")
            assert res.result_set == [[1 if a == b else 0]]

    for a in range(-10, 10):
        for b in range(-10, 10):
            res = g.query(f"RETURN {a} = {b}")
            assert res.result_set == [[1 if a == b else 0]]

    for a in range(-10, 10):
        for b in range(-10, 10):
            res = g.query(f"RETURN {a} + {b} * ({a} + {b})")
            assert res.result_set == [[a + b * (a + b)]]

    for a in range(5):
        res = g.query(f"RETURN [][{a}]")
        assert res.result_set == [[None]]

    for a in range(5):
        res = g.query(f"RETURN [0, 1, 2, 3, 4][{a}]")
        assert res.result_set == [[[0, 1, 2, 3, 4][a]]]

    res = g.query(f"UNWIND [NULL, true, false, 1, 'Avi'] AS x RETURN x IS NULL")
    assert res.result_set == [[True], [False], [False], [False], [False]]

def test_unwind():
    res = g.query("UNWIND [1, 2, 3] AS x RETURN x")
    assert res.result_set == [[1], [2], [3]]

    res = g.query("UNWIND range(1, 4) AS x RETURN x")
    assert res.result_set == [[1], [2], [3]]

    res = g.query("UNWIND range(1, 4, 2) AS x RETURN x")
    assert res.result_set == [[1], [3]]

    res = g.query("UNWIND range(1, 4) AS x UNWIND range(1, 4) AS y RETURN x, y")
    assert res.result_set == [[1, 1], [1, 2], [1, 3], [2, 1], [2, 2], [2, 3], [3, 1], [3, 2], [3, 3]]

    res = g.query("UNWIND range(1, 4) AS x UNWIND range(1, 4) AS y WHERE x = 2 RETURN x, y")
    assert res.result_set == [[2, 1], [2, 2], [2, 3]]
    
def test_create_delete_match():
    res = g.query("CREATE ()")
    assert res.result_set == []

    res = g.query("MATCH (n) RETURN n")
    assert res.result_set == [[Node(0)]]

    g.query("MATCH (n) DELETE n")

    res = g.query("MATCH (n) RETURN n")
    assert res.result_set == []

    # res = g.query("UNWIND range(3) AS x CREATE (n:N) RETURN n")
    # assert res.result_set == [[Node(0, labels="N")], [Node(1, labels="N")], [Node(2, labels="N")]]

#     g.query("MATCH (n:N) DELETE n")

#     res = g.query("MATCH (n:N) RETURN n")
#     assert res.result_set == []

#     res = g.query("UNWIND range(3) AS x CREATE (n:N {v: x})-[r:R {v: x}]->(m:M {v: x}) RETURN n, r, m")
#     assert res.result_set == [[{b"id": 0, b"labels": [b"N"]}, {b"id": 0}, {b"id": 1, b"labels": [b"M"]}], [{b"id": 2, b"labels": [b"N"]}, {b"id": 1}, {b"id": 3, b"labels": [b"M"]}], [{b"id": 4, b"labels": [b"N"]}, {b"id": 2}, {b"id": 5, b"labels": [b"M"]}]]

#     res = g.query("MATCH (n:N) RETURN n.v")
#     assert res.result_set == [[0], [1], [2]]

def test_large_graph():
    g.query("UNWIND range(1024) AS x CREATE (n:N {v: x})-[r:R {v: x}]->(m:M {v: x})")
