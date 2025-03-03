import os
import subprocess
import redis

redis_server = None
client = redis.Redis(protocol=3)

def query(query):
    return client.execute_command("GRAPH.QUERY", "x", query)

def setup_module(module):
    global redis_server
    target = os.environ.get("TARGET", "target/debug/libmatrixdb.so")
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

def test_return_values():
    res = query("RETURN null")
    assert res == [[None]]

    for b in [True, False]:
        res = query(f"RETURN {b}")
        assert res == [[1 if b else 0]]

    for i in range(-10, 10):
        res = query(f"RETURN {i}")
        assert res == [[i]]

    for f in map(lambda x: x/10.0, range(-100, 100, 1)):
        res = query(f"RETURN {f}")
        assert res == [[f]]

    res = query("RETURN 'Avi'")
    assert res == [[b"Avi"]]

    res = query("RETURN []")
    assert res == [[[]]]

    res = query("RETURN ['Avi', [1, 2]]")
    assert res == [[[b"Avi", [1, 2]]]]

    res = query("RETURN {}")
    assert res == [[{}]]

    res = query("RETURN {a: 'Avi', b: 42}")
    assert res == [[{b"a": b"Avi", b"b": 42}]]

    res = query("WITH 1 AS a, 'Avi' AS b RETURN b, a")
    assert res == [[b'Avi', 1]]

    res = query("WITH 1 AS a RETURN a")
    assert res == [[1]]

def test_operators():
    for a in [True, False]:
        for b in [True, False]:
            res = query(f"RETURN {a} AND {b}")
            assert res == [[1 if a and b else 0]]

    for a in [True, False]:
        for b in [True, False]:
            for c in [True, False]:
                res = query(f"RETURN {a} AND {b} AND {c}")
                assert res == [[1 if a and b and c else 0]]

    for a in [True, False]:
        for b in [True, False]:
            res = query(f"RETURN {a} OR {b}")
            assert res == [[1 if a or b else 0]]

    for a in [True, False]:
        for b in [True, False]:
            for c in [True, False]:
                res = query(f"RETURN {a} OR {b} OR {c}")
                assert res == [[1 if a or b or c else 0]]

    for a in [True, False]:
        for b in [True, False]:
            res = query(f"RETURN {a} = {b}")
            assert res == [[1 if a == b else 0]]

    for a in range(-10, 10):
        for b in range(-10, 10):
            res = query(f"RETURN {a} = {b}")
            assert res == [[1 if a == b else 0]]

    for a in range(-10, 10):
        for b in range(-10, 10):
            res = query(f"RETURN {a} + {b} * ({a} + {b})")
            assert res == [[a + b * (a + b)]]

    for a in range(5):
        res = query(f"RETURN [][{a}]")
        assert res == [[None]]

    for a in range(5):
        res = query(f"RETURN [0, 1, 2, 3, 4][{a}]")
        assert res == [[[0, 1, 2, 3, 4][a]]]

    res = query(f"UNWIND [NULL, true, false, 1, 'Avi'] AS x RETURN x IS NULL")
    assert res == [[True], [False], [False], [False], [False]]

def test_unwind():
    res = query("UNWIND [1, 2, 3] AS x RETURN x")
    assert res == [[1], [2], [3]]

    res = query("UNWIND range(1, 4) AS x RETURN x")
    assert res == [[1], [2], [3]]

    res = query("UNWIND range(1, 4, 2) AS x RETURN x")
    assert res == [[1], [3]]

    res = query("UNWIND range(1, 4) AS x UNWIND range(1, 4) AS y RETURN x, y")
    assert res == [[1, 1], [1, 2], [1, 3], [2, 1], [2, 2], [2, 3], [3, 1], [3, 2], [3, 3]]

    res = query("UNWIND range(1, 4) AS x UNWIND range(1, 4) AS y WHERE x = 2 RETURN x, y")
    assert res == [[2, 1], [2, 2], [2, 3]]
    
def test_create_delete_match():
    res = query("CREATE ()")
    assert res == []

    res = query("MATCH (n) RETURN n")
    assert res == [[{b"id": 0, b"labels": []}]]

    query("MATCH (n) DELETE n")

    res = query("MATCH (n) RETURN n")
    assert res == []

    res = query("UNWIND range(3) AS x CREATE (n:N) RETURN n")
    assert res == [[{b"id": 0, b"labels": [b"N"]}], [{b"id": 1, b"labels": [b"N"]}], [{b"id": 2, b"labels": [b"N"]}]]

    query("MATCH (n:N) DELETE n")

    res = query("MATCH (n:N) RETURN n")
    assert res == []

    res = query("UNWIND range(3) AS x CREATE (n:N {v: x})-[r:R {v: x}]->(m:M {v: x}) RETURN n, r, m")
    assert res == [[{b"id": 0, b"labels": [b"N"]}, {b"id": 0}, {b"id": 1, b"labels": [b"M"]}], [{b"id": 2, b"labels": [b"N"]}, {b"id": 1}, {b"id": 3, b"labels": [b"M"]}], [{b"id": 4, b"labels": [b"N"]}, {b"id": 2}, {b"id": 5, b"labels": [b"M"]}]]

    res = query("MATCH (n:N) RETURN n.v")
    assert res == [[0], [1], [2]]

def test_large_graph():
    query("UNWIND range(1024) AS x CREATE (n:N {v: x})-[r:R {v: x}]->(m:M {v: x})")
