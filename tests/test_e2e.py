import os
import platform
import subprocess
from falkordb import FalkorDB, Node, Edge
from redis import Redis, ResponseError

redis_server = None
client = None
g = None

def setup_module(module):
    global redis_server, client, g
    target = os.environ.get("TARGET", "target/debug/libfalkordb.dylib" if platform.system() == "Darwin" else "target/debug/libfalkordb.so")
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

def query(query: str, params = None, write: bool = False):
    if write:
        return g.query(query, params)
    else:
        write_res = g.query(query, params)
        read_res = g.ro_query(query, params)
        assert write_res.result_set == read_res.result_set
        return write_res

def test_return_values():
    res = query("RETURN null")
    assert res.result_set == [[None]]

    for b in [True, False]:
        res = query(f"RETURN {b}")
        assert res.result_set == [[1 if b else 0]]

    for i in range(-10, 10):
        res = query(f"RETURN {i}")
        assert res.result_set == [[i]]

    for f in map(lambda x: x/10.0, range(-100, 100, 1)):
        res = query(f"RETURN {f}")
        assert res.result_set == [[f]]

    res = query("RETURN 'Avi'")
    assert res.result_set == [["Avi"]]

    res = query("RETURN []")
    assert res.result_set == [[[]]]

    res = query("RETURN ['Avi', [1, 2]]")
    assert res.result_set == [[["Avi", [1, 2]]]]

    res = query("RETURN {}")
    assert res.result_set == [[{}]]

    res = query("RETURN {a: 'Avi', b: 42}")
    assert res.result_set == [[{"a": "Avi", "b": 42}]]

    res = query("WITH 1 AS a, 'Avi' AS b RETURN b, a")
    assert res.result_set == [['Avi', 1]]

    res = query("WITH 1 AS a RETURN a")
    assert res.result_set == [[1]]

def test_parameters():
    for value in [None, True, False, 1, -1, 0.1, 'Avi', [1], {"a": 2}, {}]:
        res = query("RETURN $p", params={"p": value})
        assert res.result_set == [[value]]

def test_operators():
    for a in [True, False]:
        for b in [True, False]:
            res = query(f"RETURN {a} AND {b}")
            assert res.result_set == [[1 if a and b else 0]]

    for a in [True, False]:
        for b in [True, False]:
            for c in [True, False]:
                res = query(f"RETURN {a} AND {b} AND {c}")
                assert res.result_set == [[1 if a and b and c else 0]]

    for a in [True, False]:
        for b in [True, False]:
            res = query(f"RETURN {a} OR {b}")
            assert res.result_set == [[1 if a or b else 0]]

    for a in [True, False]:
        for b in [True, False]:
            for c in [True, False]:
                res = query(f"RETURN {a} OR {b} OR {c}")
                assert res.result_set == [[1 if a or b or c else 0]]

    for a in [True, False]:
        for b in [True, False]:
            res = query(f"RETURN {a} = {b}")
            assert res.result_set == [[1 if a == b else 0]]

    for a in range(-10, 10):
        for b in range(-10, 10):
            res = query(f"RETURN {a} = {b}")
            assert res.result_set == [[1 if a == b else 0]]

    for a in range(-10, 10):
        for b in range(-10, 10):
            res = query(f"RETURN {a} + {b} * ({a} + {b})")
            assert res.result_set == [[a + b * (a + b)]]

    for i, a in enumerate([True, 1, 'Avi', [1]]):
        res = query(f"RETURN {{a0: true, a1: 1, a2: 'Avi', a3: [1]}}.a{i}")
        assert res.result_set == [[a]]

    for i, a in enumerate([True, 1, 'Avi', [1]]):
        res = query(f"RETURN {{a: {{a0: true, a1: 1, a2: 'Avi', a3: [1]}}}}.a.a{i}")
        assert res.result_set == [[a]]

    for a in range(5):
        res = query(f"RETURN [][{a}]")
        assert res.result_set == [[None]]

    for a in range(5):
        res = query(f"RETURN [0, 1, 2, 3, 4][{a}]")
        assert res.result_set == [[[0, 1, 2, 3, 4][a]]]

    for a in range(5):
        res = query(f"RETURN [[0, 1, 2, 3, 4]][0][{a}]")
        assert res.result_set == [[[0, 1, 2, 3, 4][a]]]

    res = query(f"UNWIND [NULL, true, false, 1, 'Avi'] AS x RETURN x IS NULL")
    assert res.result_set == [[True], [False], [False], [False], [False]]

def test_unwind():
    res = query("UNWIND [1, 2, 3] AS x RETURN x")
    assert res.result_set == [[1], [2], [3]]

    res = query("UNWIND range(1, 3) AS x RETURN x")
    assert res.result_set == [[1], [2], [3]]

    res = query("UNWIND range(1, 4, 2) AS x RETURN x")
    assert res.result_set == [[1], [3]]

    res = query("UNWIND range(1, 3) AS x UNWIND range(1, 3) AS y RETURN x, y")
    assert res.result_set == [[1, 1], [1, 2], [1, 3], [2, 1], [2, 2], [2, 3], [3, 1], [3, 2], [3, 3]]

    res = query("UNWIND range(1, 3) AS x UNWIND range(1, 3) AS y WITH x, y WHERE x = 2 RETURN x, y")
    assert res.result_set == [[2, 1], [2, 2], [2, 3]]
    
def test_create_delete_match():
    res = query("CREATE ()", write=True)
    assert res.result_set == []
    assert res.nodes_created == 1

    res = query("MATCH (n) RETURN n")
    assert res.result_set == [[Node(0)]]

    res = query("MATCH (n) DELETE n", write=True)
    assert res.nodes_deleted == 1

    res = query("MATCH (n) RETURN n")
    assert res.result_set == []

    res = query("UNWIND range(1, 3) AS x CREATE (n:N) RETURN n", write=True)
    assert res.result_set == [[Node(0, labels="N")], [Node(1, labels="N")], [Node(2, labels="N")]]
    assert res.nodes_created == 3

    res = query("MATCH (n:N), (m:N) RETURN n, m")
    assert res.result_set == [[Node(0, labels="N"), Node(0, labels="N")], [Node(0, labels="N"), Node(1, labels="N")], [Node(0, labels="N"), Node(2, labels="N")], [Node(1, labels="N"), Node(0, labels="N")], [Node(1, labels="N"), Node(1, labels="N")], [Node(1, labels="N"), Node(2, labels="N")], [Node(2, labels="N"), Node(0, labels="N")], [Node(2, labels="N"), Node(1, labels="N")], [Node(2, labels="N"), Node(2, labels="N")]]

    res = query("MATCH (n:N) DELETE n", write=True)
    assert res.nodes_deleted == 3

    res = query("MATCH (n:N) RETURN n")
    assert res.result_set == []

    res = query("UNWIND range(3) AS x CREATE (n:N {v: x})-[r:R {v: x}]->(m:M {v: x}) RETURN n, r, m", write=True)
    assert res.result_set == [[Node(0, labels="N", properties={"v": 0}), Edge(0, "R", 1, 0, properties={"v": 0}), Node(1, labels="M", properties={"v": 0})], [Node(2, labels="N", properties={"v": 1}), Edge(2, "R", 3, 1, properties={"v": 1}), Node(3, labels="M", properties={"v": 1})], [Node(4, labels="N", properties={"v": 2}), Edge(4, "R", 5, 2, properties={"v": 2}), Node(5, labels="M", properties={"v": 2})]]

    res = query("MATCH (n)-[r:R]->(m) RETURN n, r, m")
    assert res.result_set == [[Node(0, labels="N", properties={"v": 0}), Edge(0, "R", 1, 0, properties={"v": 0}), Node(1, labels="M", properties={"v": 0})], [Node(2, labels="N", properties={"v": 1}), Edge(2, "R", 3, 1, properties={"v": 1}), Node(3, labels="M", properties={"v": 1})], [Node(4, labels="N", properties={"v": 2}), Edge(4, "R", 5, 2, properties={"v": 2}), Node(5, labels="M", properties={"v": 2})]]

    res = query("MATCH (n:N) RETURN n.v")
    assert res.result_set == [[0], [1], [2]]

def test_large_graph():
    query("UNWIND range(100000) AS x CREATE (n:N {v: x})-[r:R {v: x}]->(m:M {v: x})", write=True)

def test_toInteger():
    for v in [1, 1.0, 1.1, '1', '1.0', '1.1']:
        res = query("RETURN toInteger($p)", params={"p": v})
        assert res.result_set == [[int(float(v))]]

    for v in [None, True, False, '', 'Avi', [], [1], {}, {"a": 2}]:
        res = query("RETURN toInteger($p)", params={"p": v})
        assert res.result_set == [[None]]

def test_array_range():
    for a in range(-10, 10):
        for b in range(-10, 10):
            res = query(f"RETURN [1, 2, 3, 4, 5][{a}..{b}] AS r")
            assert res.result_set == [[[1, 2, 3, 4, 5][a:b]]]
            res = query("RETURN [1, 2, 3, 4, 5][$from..$to] AS r", params={"from": a, "to": b})
            assert res.result_set == [[[1, 2, 3, 4, 5][a:b]]]
    for a in range(-10, 10):
        res = query(f"RETURN [1, 2, 3, 4, 5][{a}..] AS r")
        assert res.result_set == [[[1, 2, 3, 4, 5][a:]]]
        res = query(f"RETURN [1, 2, 3, 4, 5][..{a}] AS r")
        assert res.result_set == [[[1, 2, 3, 4, 5][:a]]]

    res = query("RETURN [1, 2, 3][null..1] AS r")
    assert res.result_set == [[None]]
    res = query("RETURN [1, 2, 3][1..null] AS r")
    assert res.result_set == [[None]]
    res = query("RETURN [1, 2, 3][..] AS r")
    assert res.result_set == [[[1, 2, 3]]]

def test_array_equal():
    res = query("RETURN [1, 2] = 'foo' AS res")
    assert res.result_set == [[False]]
    res = query("RETURN [1] = [1, null] AS res")
    assert res.result_set == [[False]]
    res = query("RETURN [1, 2] = [null, 'foo'] AS res")
    assert res.result_set == [[False]]
    res = query("RETURN [1, 2] = [null, 2] AS res")
    assert res.result_set == [[None]]
    res = query("RETURN [[1]] = [[1], [null]] AS res")
    assert res.result_set == [[False]]
    res = query("RETURN [[1, 2], [1, 3]] = [[1, 2], [null, 'foo']] AS res")
    assert res.result_set == [[False]]
    res = query("RETURN [[1, 2], ['foo', 'bar']] = [[1, 2], [null, 'bar']] AS res")
    assert res.result_set == [[None]]

def test_array_concat():
    res = query("RETURN [1, 10, 100] + [4, 5] AS foo")
    assert res.result_set == [[[1, 10, 100, 4, 5]]]
    res = query("RETURN [false, true] + false AS foo")
    assert res.result_set == [[[False, True, False]]]

def test_aggregation():
    res = query("UNWIND range(1, 10) AS x RETURN collect(x)")
    assert res.result_set == [[[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]]

    res = query("UNWIND [true, 1, 1.0, 'Avi', [], {}] AS x RETURN collect(x)")
    assert res.result_set == [[[True, 1, 1.0, 'Avi', [], {}]]]

    res = query("UNWIND range(1, 10) AS x RETURN count(x)")
    assert res.result_set == [[10]]

    res = query("UNWIND range(1, 10) AS x RETURN sum(x)")
    assert res.result_set == [[55]]

    res = query("UNWIND range(1, 10) AS x RETURN min(x)")
    assert res.result_set == [[1]]

    res = query("UNWIND range(1, 10) AS x RETURN max(x)")
    assert res.result_set == [[10]]

    res = query("UNWIND range(1, 11) AS x RETURN x % 2, count(x)")
    assert res.result_set == [[1, 6], [0, 5]]