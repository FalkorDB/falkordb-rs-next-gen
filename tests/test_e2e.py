import os
import platform
import subprocess

import itertools
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
                            "target/debug/libfalkordb.dylib" if platform.system() == "Darwin" else "target/debug/libfalkordb.so")
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


def query(query: str, params=None, write: bool = False):
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

    for f in map(lambda x: x / 10.0, range(-100, 100, 1)):
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
            res = query(f"RETURN {a} + {b}")
            assert res.result_set == [[a + b]]

            res = query(f"RETURN {a} * {b}")
            assert res.result_set == [[a * b]]

            if a != 0:
                res = query(f"RETURN {a} ^ {b}")
                assert res.result_set == [[float("{:.15g}".format(pow(a, b)))]]

            if a >= 0 and b > 0:
                res = query(f"RETURN {a} % {b}")
                assert res.result_set == [[a % b]]

            res = query(f"RETURN {a} + {b} * ({a} + {b})")
            assert res.result_set == [[a + b * (a + b)]]

    for op1 in ['+', '-', '*', '/']:
        for op2 in ['+', '-', '*', '/']:
            for op3 in ['+', '-', '*', '/']:
                for op4 in ['+', '-', '*', '/']:
                    res = query(f"RETURN 1 {op1} 2 {op2} 3 {op3} 4 {op4} 5")
                    pyop1 = op1.replace("/", "//")
                    pyop2 = op2.replace("/", "//")
                    pyop3 = op3.replace("/", "//")
                    pyop4 = op4.replace("/", "//")
                    assert res.result_set == [[eval(f"1 {pyop1} 2 {pyop2} 3 {pyop3} 4 {pyop4} 5")]]

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
    assert len(res.result_set) == 9
    for record in [[Node(0, labels="N"), Node(0, labels="N")], [Node(0, labels="N"), Node(1, labels="N")],
                   [Node(0, labels="N"), Node(2, labels="N")], [Node(1, labels="N"), Node(0, labels="N")],
                   [Node(1, labels="N"), Node(1, labels="N")], [Node(1, labels="N"), Node(2, labels="N")],
                   [Node(2, labels="N"), Node(0, labels="N")], [Node(2, labels="N"), Node(1, labels="N")],
                   [Node(2, labels="N"), Node(2, labels="N")]]:
        assert record in res.result_set

    g.delete()

    res = query("UNWIND range(0, 2) AS x CREATE (n:N {v: x})-[r:R {v: x}]->(m:M {v: x}) RETURN n, r, m", write=True)
    assert len(res.result_set) == 3
    for record in [[Node(0, labels="N", properties={"v": 0}), Edge(0, "R", 1, 0, properties={"v": 0}),
                    Node(1, labels="M", properties={"v": 0})],
                   [Node(2, labels="N", properties={"v": 1}), Edge(2, "R", 3, 1, properties={"v": 1}),
                    Node(3, labels="M", properties={"v": 1})],
                   [Node(4, labels="N", properties={"v": 2}), Edge(4, "R", 5, 2, properties={"v": 2}),
                    Node(5, labels="M", properties={"v": 2})]]:
        assert record in res.result_set

    res = query("MATCH (n:N) RETURN n.v")
    assert res.result_set == [[0], [1], [2]]


def test_large_graph():
    query("UNWIND range(0, 100000) AS x CREATE (n:N {v: x})-[r:R {v: x}]->(m:M {v: x})", write=True)


def test_toInteger():
    for v in [None, '']:
        res = query("RETURN toInteger($p)", params={"p": v})
        assert res.result_set == [[None]]

    for v in [True, False, 1, 1.0, 1.1, '1', '1.0', '1.1']:
        res = query("RETURN toInteger($p)", params={"p": v})
        assert res.result_set == [[int(float(v))]]

    for v in [[], [1], {}, {"a": 2}]:
        try:
            query("RETURN toInteger($p)", params={"p": v})
            assert False, "Expected an error"
        except ResponseError as e:
            assert f"Type mismatch: expected String, Boolean, Integer, Float, or Null but was " in str(e)


def test_list_range():
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


def test_list_concat():
    res = query("RETURN [1, 10, 100] + [4, 5] AS foo")
    assert res.result_set == [[[1, 10, 100, 4, 5]]]

    res = query("RETURN [false, true] + false AS foo")
    assert res.result_set == [[[False, True, False]]]


def test_in_list():
    # test that the error is correct on all cases
    for value, name in [(False, 'Boolean'), (1, 'Integer'), (1.0, 'Float'), ('"Avi"', 'String'), ({}, 'Map')]:
        try:
            query(f"RETURN 0 IN {value} AS r")
            assert False, "Expected an error"
        except ResponseError as e:
            assert f"Type mismatch" in str(e)

    # test for simple values
    for value in [True, False, 1, -1, 0.1, 'Avi', [1]]:
        res = query("RETURN $p IN [$p]", params={"p": value})
        assert res.result_set == [[True]]

    # TCK
    res = query("WITH [[1, 2, 3]] AS list RETURN 3 IN list[0] AS r")
    assert res.result_set == [[True]]

    res = query("RETURN 1 IN null AS r")
    assert res.result_set == [[None]]

    res = query("RETURN 3 IN [[1, 2, 3]][0] AS r")
    assert res.result_set == [[True]]

    res = query("WITH [1, 2, 3] AS list RETURN 3 IN list[0..1] AS r")
    assert res.result_set == [[False]]

    res = query("RETURN 1 IN ['1', 2] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN [1, 2] IN [1, [1, '2']] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN [1] IN [1, 2] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN [1, 2] IN [1, 2] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN [1] IN [1, 2, [1]] AS res")
    assert res.result_set == [[True]]

    res = query("RETURN [1, 2] IN [1, [1, 2]] AS res")
    assert res.result_set == [[True]]

    res = query("RETURN [1, 2] IN [1, [2, 1]] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN [1, 2] IN [1, [1, 2, 3]] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN [1, 2] IN [1, [[1, 2]]] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN [[1, 2], [3, 4]] IN [5, [[1, 2], [3, 4]]] AS res")
    assert res.result_set == [[True]]

    res = query("RETURN [[1, 2], 3] IN [1, [[1, 2], 3]] AS res")
    assert res.result_set == [[True]]

    res = query("RETURN [[1]] IN [2, [[1]]] AS res")
    assert res.result_set == [[True]]

    res = query("RETURN [[1, 3]] IN [2, [[1, 3]]] AS res")
    assert res.result_set == [[True]]

    res = query("RETURN [[1]] IN [2, [1]] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN [[1, 3]] IN [2, [1, 3]] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN null IN [null] AS res")
    assert res.result_set == [[None]]

    res = query("RETURN [null] IN [[null]] AS res")
    assert res.result_set == [[None]]

    res = query("RETURN [null] IN [null] AS res")
    assert res.result_set == [[None]]

    res = query("RETURN [1] IN [[1, null]] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN 3 IN [1, null, 3] AS res")
    assert res.result_set == [[True]]

    res = query("RETURN 4 IN [1, null, 3] AS res")
    assert res.result_set == [[None]]

    res = query("RETURN [1, 2] IN [[null, 'foo'], [1, 2]] AS res")
    assert res.result_set == [[True]]

    res = query("RETURN [1, 2] IN [1, [1, 2], null] AS res")
    assert res.result_set == [[True]]

    res = query("RETURN [1, 2] IN [[null, 'foo']] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN [1, 2] IN [[null, 2]] AS res")
    assert res.result_set == [[None]]

    res = query("RETURN [1, 2] IN [1, [1, 2, null]] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN [1, 2, null] IN [1, [1, 2, null]] AS res")
    assert res.result_set == [[None]]

    res = query("RETURN [1, 2] IN [[null, 2], [1, 2]] AS res")
    assert res.result_set == [[True]]

    res = query("RETURN [[1, 2], [3, 4]] IN [5, [[1, 2], [3, 4], null]] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN [1, 2] IN [[null, 2], [1, 3]] AS res")
    assert res.result_set == [[None]]

    res = query("RETURN [] IN [[]] AS res")
    assert res.result_set == [[True]]

    res = query("RETURN [] IN [] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN [] IN [1, []] AS res")
    assert res.result_set == [[True]]

    res = query("RETURN [] IN [1, 2] AS res")
    assert res.result_set == [[False]]

    res = query("RETURN [[]] IN [1, [[]]] AS res")
    assert res.result_set == [[True]]

    res = query("RETURN [] IN [1, 2, null] AS res")
    assert res.result_set == [[None]]

    res = query("RETURN [[], []] IN [1, [[], []]] AS res")
    assert res.result_set == [[True]]


def test_is_equal():
    for v in [1, 1.0, 1.1, '1', '1.0', '1.1', True, False, None, "Avi", [], {}, [1], {"a": 2}]:
        res = query("RETURN $a = null AS res", params={"a": v})
        assert res.result_set == [[None]]

        res = query("RETURN null = $a AS res", params={"a": v})
        assert res.result_set == [[None]]

        res = query("RETURN $a = $a = null = 1.8 AS res", params={"a": v})
        assert res.result_set == [[None]]

    for v in [1, 1.0, 1.1, '1', '1.0', '1.1', True, False, "Avi", [], {}, [1], {"a": 2}]:
        res = query("RETURN $a = $a AS res", params={"a": v})
        assert res.result_set == [[True]]

        # res = query("RETURN $a = $a = $a AS res", params={"a": v})
        # assert res.result_set == [[True]]

        res = query("RETURN $a = $a = $a = $b AS res", params={"a": v, "b": "foo"})
        assert res.result_set == [[False]]

        # res = query("RETURN $a = $a = $a = null AS res", params={"a": v})
        # assert res.result_set == [[None]]

        res = query("RETURN $a = $a = $a = $b AS res", params={"a": v, "b": "foo"})
        assert res.result_set == [[False]]

        # res = query("RETURN $a = $a = 1.8 = null AS res", params={"a": v})
        # assert res.result_set == [[False]]

    res = query("RETURN $a = $a AS res", params={"a": None})
    assert res.result_set == [[None]]
    res = query("RETURN [null] = [null] AS res")
    assert res.result_set == [[None]]

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


def test_list_size():
    res = query("RETURN size([1, 2, 3]) AS res")
    assert res.result_set == [[3]]

    res = query("RETURN size([]) AS res")
    assert res.result_set == [[0]]

    res = query("RETURN size(null) AS res")
    assert res.result_set == [[None]]

    res = query("RETURN size('Avi') AS res")
    assert res.result_set == [[3]]

    for value, name in [(False, 'Boolean'), (True, 'Boolean'), (1, 'Integer'), (1.0, 'Float'), ({}, 'Map')]:
        try:
            query(f"RETURN size({value}) AS r")
            assert False, "Expected an error"
        except ResponseError as e:
            assert f"Type mismatch: expected List, String, or Null but was {name}" in str(e)

    res = query("RETURN size([[], []] + [[]]) AS l")
    assert res.result_set == [[3]]
    res = query("WITH null AS l RETURN size(l), size(null)")
    assert res.result_set == [[None, None]]


def test_list_head():
    res = query("RETURN head([1, 2, 3]) AS res")
    assert res.result_set == [[1]]

    res = query("RETURN head([]) AS res")
    assert res.result_set == [[None]]

    res = query("RETURN head(null) AS res")
    assert res.result_set == [[None]]

    for value, name in [(False, 'Boolean'), (True, 'Boolean'), (1, 'Integer'), (1.0, 'Float'), ({}, 'Map')]:
        try:
            query(f"RETURN head({value}) AS r")
            assert False, "Expected an error"
        except ResponseError as e:
            assert f"Type mismatch: expected List or Null but was {name}" in str(e)

        res = query(f"RETURN head([{value}, 1]) AS res")
        assert res.result_set == [[value]]


def test_list_last():
    res = query("RETURN last([1, 2, 3]) AS res")
    assert res.result_set == [[3]]

    res = query("RETURN last([]) AS res")
    assert res.result_set == [[None]]

    res = query("RETURN last(null) AS res")
    assert res.result_set == [[None]]

    for value, name in [(False, 'Boolean'), (True, 'Boolean'), (1, 'Integer'), (1.0, 'Float'), ({}, 'Map')]:
        try:
            query(f"RETURN last({value}) AS r")
            assert False, "Expected an error"
        except ResponseError as e:
            assert f"Type mismatch: expected List or Null but was {name}" in str(e)

        res = query(f"RETURN last([1, {value}]) AS res")
        assert res.result_set == [[value]]


def test_list_tail():
    res = query("RETURN tail([1, 2, 3]) AS res")
    assert res.result_set == [[[2, 3]]]

    res = query("RETURN tail([]) AS res")
    assert res.result_set == [[[]]]

    res = query("RETURN tail(null) AS res")
    assert res.result_set == [[None]]

    for value, name in [(False, 'Boolean'), (True, 'Boolean'), (1, 'Integer'), (1.0, 'Float'), ({}, 'Map')]:
        try:
            query(f"RETURN tail({value}) AS r")
            assert False, "Expected an error"
        except ResponseError as e:
            assert f"Type mismatch: expected List or Null but was {name}" in str(e)


def test_list_reverse():
    res = query("RETURN reverse([1, 2, 3]) AS res")
    assert res.result_set == [[[3, 2, 1]]]

    res = query("RETURN reverse(['a', 'b', 'c']) AS res")
    assert res.result_set == [[['c', 'b', 'a']]]

    res = query("RETURN reverse([True, False]) AS res")
    assert res.result_set == [[[False, True]]]

    res = query("RETURN reverse([null, False]) AS res")
    assert res.result_set == [[[False, None]]]

    res = query("RETURN reverse([]) AS res")
    assert res.result_set == [[[]]]

    res = query("RETURN reverse(null) AS res")
    assert res.result_set == [[None]]

    for value, name in [(False, 'Boolean'), (True, 'Boolean'), (1, 'Integer'), (1.0, 'Float'), ({}, 'Map')]:
        try:
            query(f"RETURN reverse({value}) AS r")
            assert False, "Expected an error"
        except ResponseError as e:
            assert f"Type mismatch" in str(e)


def cypher_xor(a, b, c):
    """
    This function simulates the XOR operation for three boolean values.
    It returns True if an odd number of inputs are True, otherwise it returns False.
    """
    if a == "null" or b == "null" or c == "null":
        return None
    else:
        return a ^ b ^ c


def test_xor():
    # Define the possible values
    values = [True, False, "null"]

    # Generate all possible triples
    triples = list(itertools.product(values, repeat=3))

    for (a, b, c) in triples:
        res = query(f"RETURN {a} XOR {b} XOR {c} AS r")
        expected = cypher_xor(a, b, c)
        assert res.result_set == [[expected]]


def test_literals():
    for i in range(-100, 101):
        hex_representation = hex(i)
        res = query(f"RETURN {hex_representation} AS literal")
        assert res.result_set == [[i]]

        # octal representation with leading zero, old format
        res = query("RETURN 02613152366 AS literal")
        assert res.result_set == [[372036854]]

        res = query("RETURN .2 AS literal")
        assert res.result_set == [[0.2]]

        res = query("RETURN -.2 AS literal")
        assert res.result_set == [[-0.2]]


def test_split():
    res = query("RETURN split('Learning Cypher!', ' ')")
    assert res.result_set == [[["Learning", "Cypher!"]]]

    res = query("RETURN split('We are learning Cypher!', ' ')")
    assert res.result_set == [[["We", "are", "learning", "Cypher!"]]]

    res = query("RETURN split('Hakuna-Matata', ' ')")
    assert res.result_set == [[["Hakuna-Matata"]]]

    res = query("RETURN split('Hakuna-Matata', '-')")
    assert res.result_set == [[["Hakuna", "Matata"]]]

    res = query("RETURN split('We are learning Cypher', 'e ')")
    assert res.result_set == [[["W", "ar", "learning Cypher"]]]

    res = query("RETURN split('We are learning Cypher', null)")
    assert res.result_set == [[None]]

    res = query("RETURN split(null, ' ')")
    assert res.result_set == [[None]]

    res = query("RETURN split('we are learning cypher', '')")
    assert res.result_set == [
        [["w", "e", " ", "a", "r", "e", " ", "l", "e", "a", "r", "n", "i", "n", "g", " ", "c", "y", "p", "h", "e",
          "r"]]]

    for value in [False, True, 1, 1.0, {}, [], ["foo"]]:
        try:
            query(f"RETURN split({value}, 'a') AS r")
            assert False, "Expected an error"
        except ResponseError as e:
            assert f"Type mismatch" in str(e)

        try:
            query(f"RETURN split('a', {value}) AS r")
            assert False, "Expected an error"
        except ResponseError as e:
            assert f"Type mismatch" in str(e)


def test_letter_casing():
    res = query("RETURN toUpper('Avi') AS name")
    assert res.result_set == [["AVI"]]

    res = query("RETURN toLower('Avi') AS name")
    assert res.result_set == [["avi"]]

    res = query("RETURN toLower(null) AS name")
    assert res.result_set == [[None]]

    res = query("RETURN toUpper(null) AS name")
    assert res.result_set == [[None]]

    res = query("RETURN toLower('') AS name")
    assert res.result_set == [[""]]

    res = query("RETURN toUpper('') AS name")
    assert res.result_set == [[""]]

    for value in [False, True, 1, 1.0, {}, [], ["foo"]]:
        try:
            query(f"RETURN toLower({value}) AS r")
            assert False, "Expected an error"
        except ResponseError as e:
            assert f"Type mismatch" in str(e)

        try:
            query(f"RETURN toUpper({value}) AS r")
            assert False, "Expected an error"
        except ResponseError as e:
            assert f"Type mismatch" in str(e)


def test_add():
    res = query("RETURN null + 1 AS name")
    assert res.result_set == [[None]]

    res = query("RETURN 1 + null AS name")
    assert res.result_set == [[None]]

    res = query("RETURN 9223372036854775807 + 2 AS name")
    assert res.result_set == [[-9223372036854775807]]

    res = query("RETURN 1 + 1 AS name")
    assert res.result_set == [[2]]

    res = query("RETURN 1.0 + 1.0 AS name")
    assert res.result_set == [[2.0]]

    res = query("RETURN 1.1 + 1 AS name")
    assert res.result_set == [[2.1]]

    res = query("RETURN 1 + 1.1 AS name")
    assert res.result_set == [[2.1]]

    res = query("RETURN [1] + [1] AS name")
    assert res.result_set == [[[1, 1]]]

    res = query("RETURN [1] + 1 AS name")
    assert res.result_set == [[[1, 1]]]

    res = query("RETURN [] + 1 AS name")
    assert res.result_set == [[[1]]]

    res = query("RETURN 'a' + [1, 2 ,3] AS name")
    assert res.result_set == [[['a', 1, 2, 3]]]

    res = query("RETURN 'a' + 'b' + 'c' AS name")
    assert res.result_set == [["abc"]]

    res = query("RETURN 'a' + 'b' + 1 AS name")
    assert res.result_set == [["ab1"]]

    res = query("RETURN 'a' + 'b' + 0.100000 AS name")
    assert res.result_set == [["ab0.100000"]] or res.result_set == [["ab0.1"]]

    res = query("RETURN 'a' + True AS name")
    assert res.result_set == [["atrue"]]

    try:
        query("RETURN {} + 1 AS name")
    except ResponseError as e:
        pass


def test_starts_with():
    res = query("RETURN null STARTS WITH 'a' AS name")
    assert res.result_set == [[None]]

    res = query("RETURN 'ab' STARTS WITH null AS name")
    assert res.result_set == [[None]]

    res = query("RETURN 'ab' STARTS WITH 'a' AS name")
    assert res.result_set == [[True]]

    res = query("RETURN 'ab' STARTS WITH 'b' AS name")
    assert res.result_set == [[False]]

    res = query("RETURN '' STARTS WITH 'b' AS name")
    assert res.result_set == [[False]]

    try:
        query("RETURN [1, 2] STARTS WITH 'a' AS name")
        assert False, "Expected an error"
    except ResponseError as e:
        assert "Type mismatch: expected String or Null but was" in str(e)


def test_ends_with():
    res = query("RETURN null ENDS WITH 'a' AS name")
    assert res.result_set == [[None]]

    res = query("RETURN 'ab' ENDS WITH null AS name")
    assert res.result_set == [[None]]

    res = query("RETURN 'ab' ENDS WITH 'b' AS name")
    assert res.result_set == [[True]]

    res = query("RETURN 'ab' ENDS WITH 'a' AS name")
    assert res.result_set == [[False]]

    res = query("RETURN '' ENDS WITH 'b' AS name")
    assert res.result_set == [[False]]

    try:
        query("RETURN [1, 2] ENDS WITH 'a' AS name")
        assert False, "Expected an error"
    except ResponseError as e:
        assert "Type mismatch: expected String or Null but was" in str(e)


def test_contains():
    res = query("RETURN null CONTAINS 'a' AS name")
    assert res.result_set == [[None]]

    res = query("RETURN 'ab' CONTAINS null AS name")
    assert res.result_set == [[None]]

    res = query("RETURN 'ab' CONTAINS 'b' AS name")
    assert res.result_set == [[True]]

    res = query("RETURN 'ab' CONTAINS 'a' AS name")
    assert res.result_set == [[True]]

    res = query("RETURN 'ab' CONTAINS 'c' AS name")
    assert res.result_set == [[False]]

    res = query("RETURN '' CONTAINS 'b' AS name")
    assert res.result_set == [[False]]

    try:
        query("RETURN [1, 2] CONTAINS 'a' AS name")
        assert False, "Expected an error"
    except ResponseError as e:
        assert "Type mismatch: expected String or Null but was" in str(e)


def test_replace():
    # Null handling
    res = query("RETURN replace(null, 'a', 'b') AS result")
    assert res.result_set == [[None]]

    res = query("RETURN replace('abc', null, 'b') AS result")
    assert res.result_set == [[None]]

    res = query("RETURN replace('abc', 'a', null) AS result")
    assert res.result_set == [[None]]

    # Basic replacements
    res = query("RETURN replace('abc', 'a', 'x') AS result")
    assert res.result_set == [["xbc"]]

    res = query("RETURN replace('abcabc', 'a', 'x') AS result")
    assert res.result_set == [["xbcxbc"]]

    res = query("RETURN replace('abc', 'd', 'x') AS result")
    assert res.result_set == [["abc"]]  # No match, no replacement

    # Empty strings
    res = query("RETURN replace('abc', '', 'x') AS result")
    assert res.result_set == [["xaxbxcx"]]

    res = query("RETURN replace('', 'a', 'x') AS result")
    assert res.result_set == [[""]]  # Empty input string remains empty

    res = query("RETURN replace('abc', 'a', '') AS result")
    assert res.result_set == [["bc"]]  # Replacement with empty string removes matches

    # Type mismatch
    for value, name in [(1, 'Integer'), (1.0, 'Float'), (True, 'Boolean'), ({}, 'Map'), ([], 'List')]:
        try:
            query(f"RETURN replace({value}, 'a', 'b') AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert "Type mismatch" in str(e)

        try:
            query(f"RETURN replace('abc', {value}, 'b') AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert "Type mismatch" in str(e)

        try:
            query(f"RETURN replace('abc', 'a', {value}) AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert "Type mismatch" in str(e)


@pytest.mark.extra
def test_regex_matches():
    res = query("RETURN 'abc' =~ 'a.*' AS result")
    assert res.result_set == [[True]]

    res = query("RETURN 'abc' =~ 'd.*' AS result")
    assert res.result_set == [[False]]

    res = query("RETURN 'abc' =~ 'a.*c' AS result")
    assert res.result_set == [[True]]

    res = query("RETURN 'abc' =~ 'a.*d' AS result")
    assert res.result_set == [[False]]

    res = query("RETURN 'abc' =~ '^a.*c$' AS result")
    assert res.result_set == [[True]]

    res = query("RETURN 'abc' =~ '^d.*c$' AS result")
    assert res.result_set == [[False]]

    # Null handling
    res = query("RETURN null =~ 'a.*' AS result")
    assert res.result_set == [[None]]

    res = query("RETURN 'abc' =~ null AS result")
    assert res.result_set == [[None]]

    # Type mismatch
    for value, name in [(1, 'Integer'), (1.0, 'Float'), (True, 'Boolean'), ({}, 'Map'), ([], 'List')]:
        try:
            query(f"RETURN {value} =~ 'a.*' AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert "Type mismatch" in str(e)

        try:
            query(f"RETURN 'abc' =~ {value} AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert "Type mismatch" in str(e)


def test_left():
    # Null handling
    res = query("RETURN left(null, 3) AS result")
    assert res.result_set == [[None]]

    # Basic functionality
    res = query("RETURN left('abc', 2) AS result")
    assert res.result_set == [["ab"]]

    res = query("RETURN left('abc', 0) AS result")
    assert res.result_set == [[""]]

    res = query("RETURN left('abc', 5) AS result")
    assert res.result_set == [["abc"]]  # n > length of string

    # Negative values for n
    try:
        query("RETURN left('abc', -1) AS result")
        assert False, "Expected an error"
    except ResponseError as e:
        assert "length must be a non-negative integer" in str(e)

    try:
        query("RETURN left('abc', null) AS result")
        assert False, "Expected an error"
    except ResponseError as e:
        assert "length must be a non-negative integer" in str(e)

    # Type mismatch
    for value, name in [(1.0, 'Float'), (True, 'Boolean'), ({}, 'Map'), ([], 'List')]:
        try:
            query(f"RETURN left({value}, 2) AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert "Type mismatch" in str(e)

        try:
            query(f"RETURN left('abc', {value}) AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert "Type mismatch" in str(e)


def test_ltrim():
    # Null handling
    res = query("RETURN ltrim(null) AS result")
    assert res.result_set == [[None]]

    # Basic functionality
    res = query("RETURN ltrim('   abc') AS result")
    assert res.result_set == [["abc"]]

    res = query("RETURN ltrim('abc   ') AS result")
    assert res.result_set == [["abc   "]]

    res = query("RETURN ltrim('   abc   ') AS result")
    assert res.result_set == [["abc   "]]

    res = query("RETURN ltrim('abc') AS result")
    assert res.result_set == [["abc"]]

    # Type mismatch
    for value, name in [(1.0, 'Float'), (True, 'Boolean'), ({}, 'Map'), ([], 'List')]:
        try:
            query(f"RETURN ltrim({value}) AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert "Type mismatch" in str(e)


def test_right():
    # Null handling
    res = query("RETURN right(null, 3) AS result")
    assert res.result_set == [[None]]

    # Basic functionality
    res = query("RETURN right('abc', 2) AS result")
    assert res.result_set == [["bc"]]

    res = query("RETURN right('abc', 0) AS result")
    assert res.result_set == [[""]]

    res = query("RETURN right('abc', 5) AS result")
    assert res.result_set == [["abc"]]  # n > length of string

    # Negative values for n
    try:
        query("RETURN right('abc', -1) AS result")
        assert False, "Expected an error"
    except ResponseError as e:
        assert "length must be a non-negative integer" in str(e)
    try:
        query("RETURN right('abc', null) AS result")
        assert False, "Expected an error"
    except ResponseError as e:
        assert "length must be a non-negative integer" in str(e)

    # Type mismatch
    for value, name in [(1.0, 'Float'), (True, 'Boolean'), ({}, 'Map'), ([], 'List')]:
        try:
            query(f"RETURN right({value}, 2) AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert "Type mismatch" in str(e)

        try:
            query(f"RETURN right('abc', {value}) AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert "Type mismatch" in str(e)


def test_substring():
    # Null handling
    res = query("RETURN substring(null, 0, 2) AS result")
    assert res.result_set == [[None]]

    # Basic functionality
    res = query("RETURN substring('abc', 0, 2) AS result")
    assert res.result_set == [["ab"]]

    res = query("RETURN substring('abc', 1, 2) AS result")
    assert res.result_set == [["bc"]]

    res = query("RETURN substring('abc', 0, 3) AS result")
    assert res.result_set == [["abc"]]  # n > length of string

    # Negative values for start and length
    try:
        query("RETURN substring('abc', -1, 2) AS result")
        assert False, "Expected an error"
    except ResponseError as e:
        assert "start must be a non-negative integer" in str(e)

    try:
        query("RETURN substring('abc', 0, -1) AS result")
        assert False, "Expected an error"
    except ResponseError as e:
        assert "length must be a non-negative integer" in str(e)

    # Type mismatch
    for value, name in [(1.0, 'Float'), (True, 'Boolean'), ({}, 'Map'), ([], 'List')]:
        try:
            query(f"RETURN substring({value}, 0, 2) AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert "Type mismatch" in str(e)

        try:
            query(f"RETURN substring('abc', {value}, 2) AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert "Type mismatch" in str(e)

        try:
            query(f"RETURN substring('abc', 0, {value}) AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert "Type mismatch" in str(e)

    try:
        query(f"RETURN substring('abc', null, 2) AS result")
        assert False, "Expected an error"
    except ResponseError as e:
        assert "Type mismatch" in str(e)

    try:
        query(f"RETURN substring('abc', 0, null) AS result")
        assert False, "Expected an error"
    except ResponseError as e:
        assert "Type mismatch" in str(e)


def test_graph_list():
    assert client is not None
    for i in range(1000):
        client.select_graph(f"g{i}").query("return 1")
        client.connection.set(f"ng{i}", "ng")
    graphs = client.list_graphs()

    assert len(graphs) == 1001
    for i in range(1000):
        assert f'g{i}' in graphs
    assert 'test' in graphs


def test_function_with_namespace():
    res = query("RETURN string.join(null, ',') AS result")
    assert res.result_set == [[None]]

    res = query("RETURN string.join([], 'foo') AS result")
    assert res.result_set == [[""]]

    res = query("RETURN string.join(['a', 'b'], ', ') AS result")
    assert res.result_set == [['a, b']]

    res = query("RETURN string.join(['a', 'b']) AS result")
    assert res.result_set == [['ab']]

    try:
        query(f"RETURN string.join(['a', 'b'], ', ', ', ') AS result")
        assert False, "Expected an error"
    except ResponseError as e:
        assert "Received 3 arguments to function 'string.join', expected at most 2" in str(e)

    try:
        query(f"RETURN string.join(1, 2) AS result")
        assert False, "Expected an error"
    except ResponseError as e:
        assert "Type mismatch: expected List or Null but was Integer" in str(e)

    for value, name in [(1.0, 'Float'), (True, 'Boolean'), ({}, 'Map'), ([], 'List'), ("null", 'Null')]:
        try:
            query(f"RETURN string.join(['a', {value}], ',') AS result")
            assert False, "Expected an error"
        except ResponseError as e:
            assert f"Type mismatch: expected String but was {name}" in str(e)
