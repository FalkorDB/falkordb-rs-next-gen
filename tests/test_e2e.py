from decimal import Decimal
import sys
from typing import Counter
import common
from falkordb import Node, Edge
from hypothesis import given, strategies as st
import itertools
import math
import pytest
from redis import ResponseError

text_st = st.text().filter(lambda s: all(0x00 < ord(c) < 0x80 for c in s))
at_least_1_text_st = st.text("abcdefghijklmnopqrstuvwxyz", min_size=1)

def setup_module(module):
    common.start_redis()


def teardown_module(module):
    common.shutdown_redis()


def setup_function(function):
    if common.g.name in common.client.list_graphs():
        common.g.delete()


def query(query: str, params=None, write: bool = False, compare_results: bool = True):
    if write:
        try:
            common.g.query("RETURN 1")
            read_res = common.g.ro_query(query, params)
            assert False
        except ResponseError as e:
            assert "graph.RO_QUERY is to be executed only on read-only queries" == str(e)
        return common.g.query(query, params)
    else:
        write_res = common.g.query(query, params)
        read_res = common.g.ro_query(query, params)
        if compare_results:
            assert len(write_res.result_set) == len(read_res.result_set)
            for i in range(len(write_res.result_set)):
                assert len(write_res.result_set[i]) == len(read_res.result_set[i])
                for j in range(len(write_res.result_set[i])):
                    assert (write_res.result_set[i][j] == read_res.result_set[i][j]
                            or (math.isnan(write_res.result_set[i][j])
                                and math.isnan(read_res.result_set[i][j])))
        return write_res


def query_exception(query: str, message: str, params=None):
    try:
        common.g.query(query, params)
        assert False, "Expected an error"
    except ResponseError as e:
        assert message in str(e)


def assert_result_set_equal_no_order(res, expected):
    assert len(res.result_set) == len(expected)
    for record in expected:
        assert record in res.result_set


def test_return_values():
    res = query("RETURN null")
    assert res.result_set == [[None]]

    for b in [True, False]:
        res = query(f"RETURN {b}")
        assert res.result_set == [[1 if b else 0]]

    for i in range(0, 100):
        for sign in ['', '-', '- ', '+', '+ ']:
            res = query(f"RETURN {sign}{i}")
            assert res.result_set == [[eval(f"{sign}{i}")]]

            res = query(f"RETURN {sign}{i / 10.0}")
            assert res.result_set == [[eval(f"{sign}{i / 10.0}")]]

            # test number in hex format 0x...
            n = hex(i)
            res = query(f"RETURN {sign}{n}")
            assert res.result_set == [[eval(f"{sign}{n}")]]

            # Test engineering notation
            eng_notation = f"{sign}{i / 10.0:e}"
            res = query(f"RETURN {eng_notation} AS literal")
            assert res.result_set == [[eval(f"{eng_notation}")]]

    # Test specific cases
    res = query("RETURN .5 AS literal")
    assert res.result_set == [[0.5]]

    res = query("RETURN -.5 AS literal")
    assert res.result_set == [[-0.5]]

    res = query("RETURN 1e-3 AS literal")
    assert res.result_set == [[0.001]]

    res = query("RETURN -1e3 AS literal")
    assert res.result_set == [[-1000.0]]

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


@pytest.mark.extra
def test_numerical_bases():
    for i in range(0, 100):
        for sign in ['', '-', '- ', '+', '+ ']:
            n = oct(i)
            res = query(f"RETURN {sign}{n}")
            assert res.result_set == [[eval(f"{sign}{n}")]]

            n = bin(i)
            res = query(f"RETURN {sign}{n}")
            assert res.result_set == [[eval(f"{sign}{n}")]]


def test_parameters():
    for value in [None, True, False, 1, -1, 0.1, 'Avi', [1], {"a": 2}, {}]:
        res = query("RETURN $p", params={"p": value})
        assert res.result_set == [[value]]


class CustomNumber:
    def __init__(self, value):
        self.value = value

    def __add__(self, other):
        return CustomNumber(self.value + other.value)

    def __sub__(self, other):
        return CustomNumber(self.value - other.value)

    def __mul__(self, other):
        return CustomNumber(self.value * other.value)

    def __truediv__(self, other):
        if isinstance(self.value, int) and isinstance(other.value, int):
            return CustomNumber(self.value // other.value)
        return CustomNumber(self.value / other.value)

    def __mod__(self, other):
        return CustomNumber(self.value % other.value)


def test_operators():
    for op in ["and", "or"]:
        for a in [True, False]:
            for b in [True, False]:
                res = query(f"RETURN {a} {op} {b}")
                assert res.result_set == [[1 if eval(f"{a} {op} {b}") else 0]]

    for op1 in ["and", "or"]:
        for op2 in ["and", "or"]:
            for a in [True, False]:
                for b in [True, False]:
                    for c in [True, False]:
                        res = query(f"RETURN {a} {op1} {b} {op2} {c}")
                        assert res.result_set == [[1 if eval(f"{a} {op1} {b} {op2} {c}") else 0]]

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

    for op1 in ['+', '-', '*', '/', '%']:
        for op2 in ['+', '-', '*', '/', '%']:
            for op3 in ['+', '-', '*', '/', '%']:
                for op4 in ['+', '-', '*', '/', '%']:
                    for n1 in [2, 2.0]:
                        for n2 in [4, 4.0]:
                            for n3 in [8, 8.0]:
                                for n4 in [16, 16.0]:
                                    for n5 in [32, 32.0]:
                                        res = query(f"RETURN {n1} {op1} {n2} {op2} {n3} {op3} {n4} {op4} {n5}")
                                        assert res.result_set == [[eval(
                                            f"CustomNumber({n1}) {op1} CustomNumber({n2}) {op2} CustomNumber({n3}) {op3} CustomNumber({n4}) {op4} CustomNumber({n5})").value]]

    for i, a in enumerate([True, 1, 'Avi', [1]]):
        res = query(f"RETURN {{a0: true, a1: 1, a2: 'Avi', a3: [1]}}.a{i}")
        assert res.result_set == [[a]]

        res = query(f"RETURN {{a: {{a0: true, a1: 1, a2: 'Avi', a3: [1]}}}}.a.a{i}")
        assert res.result_set == [[a]]

    for a in range(5):
        res = query(f"RETURN [][{a}]")
        assert res.result_set == [[None]]

        res = query(f"RETURN [0, 1, 2, 3, 4][{a}]")
        assert res.result_set == [[[0, 1, 2, 3, 4][a]]]

        res = query(f"RETURN [[0, 1, 2, 3, 4]][0][{a}]")
        assert res.result_set == [[[0, 1, 2, 3, 4][a]]]

    res = query(f"UNWIND [NULL, true, false, 1, 'Avi', [], {{}}] AS x RETURN x IS NULL")
    assert res.result_set == [[True], [False], [False], [False], [False], [False], [False]]


@given(st.integers(-100, 100), st.integers(-100, 100))
def test_unwind(f, t):
    res = query(f"UNWIND range({f}, {t}) AS x RETURN x")
    assert res.result_set == [[i] for i in range(f, t + 1)]

    res = query(f"UNWIND {list(range(f, t + 1))} AS x RETURN x")
    assert res.result_set == [[i] for i in range(f, t + 1)]

@given(st.integers(-100, 100), st.integers(-100, 100), st.integers(-100, 100))
def test_unwind_range_step(f, t, s):
    if s == 0:
        query_exception(f"UNWIND range({f}, {t}, {s}) AS x RETURN x", "ArgumentError: step argument to range() can't be 0")
        return
    res = query(f"UNWIND range({f}, {t}, {s}) AS x RETURN x")
    if s > 0:
        if f == t:
            assert res.result_set == [[f]]
        else:
            assert res.result_set == [[i] for i in range(f, t + 1, s)]
    else:
        assert res.result_set == [[i] for i in range(f, t - 1, s)]

@given(st.integers(-10, 10), st.integers(-10, 10), st.integers(-10, 10), st.integers(-10, 10))
def test_nested_unwind_range(f1, t1, f2, t2):
    res = query(f"UNWIND range({f1}, {t1}) AS x UNWIND range({f2}, {t2}) AS y RETURN x, y")
    assert res.result_set == [[i, j] for i in range(f1, t1 + 1) for j in range(f2, t2 + 1)]

def test_graph_crud():
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
    assert_result_set_equal_no_order(res, [[Node(0, labels="N"), Node(0, labels="N")],
                                           [Node(0, labels="N"), Node(1, labels="N")],
                                           [Node(0, labels="N"), Node(2, labels="N")],
                                           [Node(1, labels="N"), Node(0, labels="N")],
                                           [Node(1, labels="N"), Node(1, labels="N")],
                                           [Node(1, labels="N"), Node(2, labels="N")],
                                           [Node(2, labels="N"), Node(0, labels="N")],
                                           [Node(2, labels="N"), Node(1, labels="N")],
                                           [Node(2, labels="N"), Node(2, labels="N")]])

    common.g.delete()

    res = query("UNWIND range(0, 2) AS x CREATE (n:N {v: x})-[r:R {v: x}]->(m:M {v: x}) RETURN n, r, m", write=True)
    assert res.nodes_created == 6
    assert res.relationships_created == 3
    assert_result_set_equal_no_order(res, [[Node(0, labels="N", properties={"v": 0}),
                                            Edge(0, "R", 1, 0, properties={"v": 0}),
                                            Node(1, labels="M", properties={"v": 0})],
                                           [Node(2, labels="N", properties={"v": 1}),
                                            Edge(2, "R", 3, 1, properties={"v": 1}),
                                            Node(3, labels="M", properties={"v": 1})],
                                           [Node(4, labels="N", properties={"v": 2}),
                                            Edge(4, "R", 5, 2, properties={"v": 2}),
                                            Node(5, labels="M", properties={"v": 2})]])

    res = query("MATCH (n)-[r:R]->(m) RETURN n, r, m")
    assert res.result_set == [[Node(0, labels="N", properties={"v": 0}), Edge(0, "R", 1, 0, properties={"v": 0}),
                               Node(1, labels="M", properties={"v": 0})],
                              [Node(2, labels="N", properties={"v": 1}), Edge(2, "R", 3, 1, properties={"v": 1}),
                               Node(3, labels="M", properties={"v": 1})],
                              [Node(4, labels="N", properties={"v": 2}), Edge(4, "R", 5, 2, properties={"v": 2}),
                               Node(5, labels="M", properties={"v": 2})]]

    res = query("MATCH (m)<-[r:R]-(n) RETURN n, r, m")
    assert res.result_set == [[Node(0, labels="N", properties={"v": 0}), Edge(0, "R", 1, 0, properties={"v": 0}),
                               Node(1, labels="M", properties={"v": 0})],
                              [Node(2, labels="N", properties={"v": 1}), Edge(2, "R", 3, 1, properties={"v": 1}),
                               Node(3, labels="M", properties={"v": 1})],
                              [Node(4, labels="N", properties={"v": 2}), Edge(4, "R", 5, 2, properties={"v": 2}),
                               Node(5, labels="M", properties={"v": 2})]]

    res = query("MATCH (n:N) RETURN n.v")
    assert res.result_set == [[0], [1], [2]]

    res = query("MATCH (n:N) DELETE n", write=True)
    assert res.nodes_deleted == 3
    assert res.relationships_deleted == 3


def test_node_labels():
    res = query("CREATE ()", write=True)
    assert res.result_set == []
    assert res.nodes_created == 1

    res = query("MATCH (n) RETURN labels(n)")
    assert res.result_set == [[[]]]

    res = query("MATCH (n) DELETE n", write=True)
    assert res.nodes_deleted == 1

    res = query("CREATE (:N:M)", write=True)
    assert res.result_set == []
    assert res.nodes_created == 1

    res = query("MATCH (n) RETURN labels(n)")
    assert res.result_set == [[["N", "M"]]]


def test_large_graph():
    query("UNWIND range(0, 100000) AS x CREATE (n:N {v: x})-[r:R {v: x}]->(m:M {v: x})", write=True)


def test_toInteger():
    for v in [None, '']:
        res = query("RETURN toInteger($p)", params={"p": v})
        assert res.result_set == [[None]]

    for v in [True, False]:
        res = query("RETURN toInteger($p)", params={"p": v})
        assert res.result_set == [[int(float(v))]]

@given(st.integers(-100, 100) | st.floats(-100, 100, allow_subnormal=False))
def test_prop_toInteger(x):
    res = query(f"RETURN toInteger({x}), toInteger('{x}')")
    if isinstance(x, float):
        assert res.result_set == [[int(math.floor(x)), int(math.floor(x))]]    
    else:
        assert res.result_set == [[int(x), int(x)]]

def test_list_range():
    res = query("RETURN [1, 2, 3][null..1] AS r")
    assert res.result_set == [[None]]
    res = query("RETURN [1, 2, 3][1..null] AS r")
    assert res.result_set == [[None]]
    res = query("RETURN [1, 2, 3][..] AS r")
    assert res.result_set == [[[1, 2, 3]]]

@given(st.integers(-10, 10), st.integers(-10, 10))
def test_prop_list_range(a, b):
    res = query(f"RETURN [1, 2, 3, 4, 5][{a}..{b}] AS r")
    assert res.result_set == [[[1, 2, 3, 4, 5][a:b]]]
    res = query("RETURN [1, 2, 3, 4, 5][$from..$to] AS r", params={"from": a, "to": b})
    assert res.result_set == [[[1, 2, 3, 4, 5][a:b]]]

    res = query(f"RETURN [1, 2, 3, 4, 5][{a}..] AS r")
    assert res.result_set == [[[1, 2, 3, 4, 5][a:]]]
    res = query(f"RETURN [1, 2, 3, 4, 5][..{a}] AS r")
    assert res.result_set == [[[1, 2, 3, 4, 5][:a]]]

@given(st.lists(st.booleans() | st.integers(-10, 10) | text_st), st.lists(st.booleans() | st.integers(-10, 10) | text_st))
def test_list_concat(a, b):
    res = query(f"RETURN $a + $b", params={"a": a, "b": b})
    assert res.result_set == [[a + b]]

@given(st.lists(st.booleans() | st.integers(-10, 10) | text_st), st.booleans() | st.integers(-10, 10) | text_st)
def test_list_append(a, b):
    res = query(f"RETURN $a + $b", params={"a": a, "b": b})
    assert res.result_set == [[a + [b]]]


def test_in_list():
    # test for simple values
    for value in [True, False, [1]]:
        res = query("RETURN $p IN [$p]", params={"p": value})
        assert res.result_set == [[True]]

@given(st.lists(st.integers(-10, 10) | text_st), st.integers(-10, 10) | text_st)
def test_prop_in_list(a, b):
    res = query("RETURN $b IN $a", params={"a": a, "b": b})
    assert res.result_set == [[b in a]]

@given(st.none() |  st.booleans() | st.integers(-10, 10) | text_st | st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st) | st.dictionaries(at_least_1_text_st, st.none() | st.booleans() | st.integers(-10, 10) | text_st))
def test_equal_null(a):
    res = query("RETURN $a = null", params={"a": a})
    assert res.result_set == [[None]]

    res = query("RETURN null = $a", params={"a": a})
    assert res.result_set == [[None]]

@given(st.booleans() | st.integers(-10, 10) | text_st | st.lists(st.booleans() | st.integers(-10, 10) | text_st) | st.dictionaries(at_least_1_text_st, st.booleans() | st.integers(-10, 10) | text_st))
def test_prop_equal(a):
    res = query("RETURN $a = $a", params={"a": a})
    assert res.result_set == [[True]]

@pytest.mark.extra
@given(st.booleans() | st.integers(-10, 10) | text_st | st.lists(st.booleans() | st.integers(-10, 10) | text_st) | st.dictionaries(at_least_1_text_st, st.booleans() | st.integers(-10, 10) | text_st))
def test_prop_equal_extra(a):
    res = query("RETURN $a = $a = $a AS res", params={"a": a})
    assert res.result_set == [[True]]

    res = query("RETURN $a = $a = $a = $b AS res", params={"a": a, "b": "foo"})
    assert res.result_set == [[False]]

@given(st.booleans() | st.integers(-10, 10) | text_st | st.lists(st.booleans() | st.integers(-10, 10) | text_st) | st.dictionaries(at_least_1_text_st, st.booleans() | st.integers(-10, 10) | text_st), st.booleans() | st.integers(-10, 10) | text_st | st.lists(st.booleans() | st.integers(-10, 10) | text_st) | st.dictionaries(at_least_1_text_st, st.booleans() | st.integers(-10, 10) | text_st))
def test_prop_equal2(a, b):
    res = query("RETURN $a = $b", params={"a": a, "b": b})
    if isinstance(a, list) and isinstance(b, list):
        assert res.result_set == [[a == b and all(type(x) == type(y) for x, y in zip(a, b))]]
    elif isinstance(a, dict) and isinstance(b, dict):
        assert res.result_set == [[a == b and all(type(a[k]) == type(b[k]) for k in a.keys())]]
    else:
        assert res.result_set == [[a == b and type(a) == type(b)]]

def test_is_equal():
    res = query("RETURN $a = $a AS res", params={"a": None})
    assert res.result_set == [[None]]
    res = query("RETURN [null] = [null] AS res")
    assert res.result_set == [[None]]

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


@given(st.none() | text_st | st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st | st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st)))
def test_list_size(a):
    res = query("RETURN size($a)", params={"a": a})
    assert res.result_set == [[len(a) if a is not None else None]]

@given(st.none() | st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st | st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st)))
def test_list_head(a):
    res = query("RETURN head($a)", params={"a": a})
    assert res.result_set == [[a[0] if a else None]]

@given(st.none() | st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st | st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st)))
def test_list_last(a):
    res = query("RETURN last($a)", params={"a": a})
    assert res.result_set == [[a[-1] if a else None]]


@given(st.none() | st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st | st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st)))
def test_list_tail(a):
    res = query("RETURN tail($a)", params={"a": a})
    if a is None:
        assert res.result_set == [[None]]
    elif len(a) == 0:
        assert res.result_set == [[[]]]
    else:
        assert res.result_set == [[a[1:]]]


@given(st.none() | st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st | st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st)))
def test_list_reverse(a):
    res = query("RETURN reverse($a)", params={"a": a})
    assert res.result_set == [[a[::-1] if a is not None else None]]


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

@given(st.none() | text_st, st.none() | text_st)
def test_split(a, b):
    res = query("RETURN split($a, $b)", params={"a": a, "b": b})
    if a is None or b is None:
        assert res.result_set == [[None]]
    elif b == "":
        if a == "":
            assert res.result_set == [[[""]]]
        else:
            assert res.result_set == [[list(a)]]
    else:
        if a == "":
            assert res.result_set == [[[""]]]
        else:
            assert res.result_set == [[a.split(b) if a else []]]

@given(st.none() | text_st)
def test_letter_casing(a):
    res = query("RETURN toUpper($a)", params={"a": a})
    assert res.result_set == [[a.upper() if a is not None else None]]


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

    query_exception("RETURN {} + 1 AS name", "")

@given(st.none() | text_st, st.none() | text_st)
def test_starts_with(a, b):
    res = query("RETURN $a STARTS WITH $b", params={"a": a, "b": b})
    assert res.result_set == [[a.startswith(b) if a is not None and b is not None else None]]

@given(st.none() | text_st, st.none() | text_st)
def test_ends_with(a, b):
    res = query("RETURN $a ENDS WITH $b", params={"a": a, "b": b})
    assert res.result_set == [[a.endswith(b) if a is not None and b is not None else None]]

@given(st.none() | text_st, st.none() | text_st)
def test_contains(a, b):
    res = query("RETURN $a CONTAINS $b", params={"a": a, "b": b})
    assert res.result_set == [[b in a if a is not None and b is not None else None]]

@given(st.none() | text_st, st.none() | text_st, st.none() | text_st)
def test_replace(a, b, c):
    res = query("RETURN replace($a, $b, $c)", params={"a": a, "b": b, "c": c})
    assert res.result_set == [[a.replace(b, c) if a is not None and b is not None and c is not None else None]]

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

@given(st.none() | text_st, st.none() | st.integers(-10, 10))
def test_left(a, b):
    if a is None:
        res = query("RETURN left($a, $b)", params={"a": a, "b": b})
        assert res.result_set == [[None]]
    elif b is None or b < 0:
        query_exception("RETURN left($a, $b)", "length must be a non-negative integer", params={"a": a, "b": b})
    else:
        res = query("RETURN left($a, $b)", params={"a": a, "b": b})
        assert res.result_set == [[a[:b]]]

@given(st.none() | text_st)
def test_ltrim(a):
    res = query("RETURN ltrim($a)", params={"a": a})
    assert res.result_set == [[a.lstrip(" ") if a is not None else None]]

@given(st.none() | text_st, st.none() | st.integers(-10, 10))
def test_right(a, b):
    if a is None:
        res = query("RETURN right($a, $b)", params={"a": a, "b": b})
        assert res.result_set == [[None]]
    elif b is None or b < 0:
        query_exception("RETURN right($a, $b)", "length must be a non-negative integer", params={"a": a, "b": b})
    else:
        res = query("RETURN right($a, $b)", params={"a": a, "b": b})
        assert res.result_set == [[a[-b:] if b > 0 else ""]]

@given(st.none() | text_st, st.integers(-10, 10), st.none() | st.integers(-10, 10))
def test_substring(a, b, c):
    q = "RETURN substring($a, $b)" if c is None else "RETURN substring($a, $b, $c)"
    if a is None:
        res = query(q, params={"a": a, "b": b, "c": c})
        assert res.result_set == [[None]]
    elif b < 0:
        query_exception(q, "start must be a non-negative integer", params={"a": a, "b": b, "c": c})
    elif b >= len(a):
        res = query(q, params={"a": a, "b": b, "c": c})
        assert res.result_set == [[a[b:(b + c if c is not None else None)]]]
    elif c is not None and c < 0:
        query_exception(q, "length must be a non-negative integer", params={"a": a, "b": b, "c": c})
    else:
        res = query(q, params={"a": a, "b": b, "c": c})
        assert res.result_set == [[a[b:(b + c if c is not None else None)]]]


@given(st.lists(at_least_1_text_st, unique=True))
def test_graph_list(a):
    for i in a:
        common.client.select_graph(i).query("return 1")
        common.client.connection.set(f"ng{i}", "ng")
    graphs = common.client.list_graphs()

    assert len(graphs) == len(a)
    for i in a:
        assert i in graphs
        common.client.select_graph(i).delete()

@given(st.lists(text_st), text_st)
def test_string_join(a, b):
    q = "RETURN string.join($a, $b)" if b else "RETURN string.join($a)"
    if a is None:
        res = query(q, params={"a": a, "b": b})
        assert res.result_set == [[None]]
    else:
        res = query(q, params={"a": a, "b": b})
        assert res.result_set == [[(b if b else "").join(a)]]


@pytest.mark.extra
def test_string_match_regex():
    res = query("RETURN string.matchRegEx(null, null) AS name")
    assert res.result_set == [[[]]]

    res = query("RETURN string.matchRegEx('foo bar', null) AS name")
    assert res.result_set == [[[]]]

    res = query("RETURN string.matchRegEx(null, '.*') AS name")
    assert res.result_set == [[[]]]

    res = query("RETURN string.matchRegEx('foo bar', '.*') AS name")
    assert res.result_set == [[["foo bar"]]]

    res = query("RETURN string.matchRegEx('foo bar', '[a-z]+\\s+[a-z]+') AS name")
    assert res.result_set == [[["foo bar"]]]

    ## multiple groups
    res = query("RETURN string.matchRegEx('foo bar', '([a-z]+)\\s+([a-z]+)') AS name")
    assert res.result_set == [[["foo bar", "foo", "bar"]]]


@pytest.mark.extra
def test_string_replace_regex():
    res = query(
        "RETURN string.replaceRegEx('foo-bar baz-qux', '(?<first>[a-z]+)-(?<last>[a-z]+)', '$first $last') AS name")
    assert res.result_set == [["foo bar baz qux"]]

    res = query(
        "RETURN string.replaceRegEx('foo-bar baz-qux', '([a-z]+)-([a-z]+)', '$1 $2') AS name")
    assert res.result_set == [["foo bar baz qux"]]

    res = query(
        "RETURN string.replaceRegEx('foo-bar baz-qux', '([a-z]+)-([a-z]+)', '${1}_${2}') AS name")
    assert res.result_set == [["foo_bar baz_qux"]]

    res = query(
        "RETURN string.replaceRegEx('foo-bar baz-qux', '(\\w+)-(\\w+)', '${1}_${2}') AS name")
    assert res.result_set == [["foo_bar baz_qux"]]

    res = query(
        "RETURN string.replaceRegEx('123', '(\\w+)-(\\w+)', '${1}_${2}') AS name")
    assert res.result_set == [["123"]]

    ## broken regex
    query_exception("RETURN string.replaceRegEx('foo bar', '**', 'a') AS name",
                    "Invalid regex")

@given(st.none() | st.integers(-100, 100) | st.decimals(-100, 100, places=13))
def test_abs(a):
    a = float(a) if isinstance(a, Decimal) else a
    res = query("RETURN abs($a)", params={"a": a})
    assert res.result_set == [[abs(a) if a is not None else None]]

@given(st.none() | st.integers(-100, 100) | st.decimals(-100, 100, places=13))
def test_ceil(a):
    a = float(a) if isinstance(a, Decimal) else a
    res = query("RETURN ceil($a)", params={"a": float(a) if a is not None else None})
    assert res.result_set == [[math.ceil(a) if a is not None else None]]


def test_e():
    res = query("RETURN e()")
    assert res.result_set == [[2.71828182845905e0]]


def test_exp():
    res = query("RETURN exp(1) AS name")
    assert res.result_set == [[2.71828182845905]]

    res = query("RETURN exp(0) AS name")
    assert res.result_set == [[1]]

    res = query("RETURN exp(-1) AS name")
    assert res.result_set == [[0.367879441171442]]

    res = query("RETURN exp(-1.0) AS name")
    assert res.result_set == [[0.367879441171442]]

    res = query("RETURN exp(null) AS name")
    assert res.result_set == [[None]]

@given(st.none() | st.integers(-100, 100) | st.floats(-100, 100, allow_subnormal=False))
def test_floor(a):
    res = query("RETURN floor($a)", params={"a": a})
    assert res.result_set == [[math.floor(a) if a is not None else None]]

def test_log():
    res = query("RETURN log(1) AS name")
    assert res.result_set == [[0]]

    res = query("RETURN log(1.0) AS name")
    assert res.result_set == [[0]]

    res = query("RETURN log(0) AS name")
    assert res.result_set == [[float('-inf')]]

    res = query("RETURN log(-1) AS name")
    assert math.isnan(res.result_set[0][0])

    res = query("RETURN log(null) AS name")
    assert res.result_set == [[None]]


def test_log10():
    res = query("RETURN log10(1) AS name")
    assert res.result_set == [[0]]

    res = query("RETURN log10(1.0) AS name")
    assert res.result_set == [[0]]

    res = query("RETURN log10(0) AS name")
    assert res.result_set == [[float('-inf')]]

    res = query("RETURN log10(-1) AS name")
    assert math.isnan(res.result_set[0][0])

    res = query("RETURN log10(null) AS name")
    assert res.result_set == [[None]]


def test_pow():
    res = query("RETURN pow(2, 3) AS name")
    assert res.result_set == [[8]]

    res = query("RETURN pow(2.0, 3) AS name")
    assert res.result_set == [[8.0]]

    res = query("RETURN pow(2.0, 3.0) AS name")
    assert res.result_set == [[8.0]]

    res = query("RETURN pow(2, 3.0) AS name")
    assert res.result_set == [[8.0]]

    res = query("RETURN pow(2, -3) AS name")
    assert res.result_set == [[0.125]]

    res = query("RETURN pow(2, 0) AS name")
    assert res.result_set == [[1]]

    res = query("RETURN pow(-2, 3) AS name")
    assert res.result_set == [[-8]]

    res = query("RETURN pow(-2, -3) AS name")
    assert res.result_set == [[-0.125]]

    res = query("RETURN pow(-2, 0) AS name")
    assert res.result_set == [[1]]

    res = query("RETURN pow(null, 3) AS name")
    assert res.result_set == [[None]]

    res = query("RETURN pow(3, null) AS name")
    assert res.result_set == [[None]]


def shannon_entropy(data):
    n = len(data)
    counts = Counter(data)
    probabilities = [count / n for count in counts.values()]

    return -sum(p * math.log2(p) for p in probabilities if p > 0)


def test_rand():
    data = []
    for _ in range(1000):
        res = query("RETURN rand()", compare_results=False)
        data.append(res.result_set[0][0])
        assert res.result_set[0][0] >= 0.0
        assert res.result_set[0][0] < 1.0
    assert shannon_entropy(data) > 0.9  # Check for randomness


def round_away_from_zero(num):
    if num > 0:
        return math.floor(num + 0.5)
    else:
        return math.ceil(num - 0.5)

@given(st.none() | st.integers(-100, 100) | st.floats(-100, 100, allow_subnormal=False))
def test_round(a):
    res = query("RETURN round($a)", params={"a": a})
    assert res.result_set == [[round_away_from_zero(a) if a is not None else None]]


def signum(x):
    return (x > 0) - (x < 0)

@given(st.none() | st.integers(-100, 100) | st.floats(-100, 100, allow_subnormal=False))
def test_sign(a):
    res = query("RETURN sign($a)", params={"a": a})
    assert res.result_set == [[signum(a) if a is not None else None]]


def test_sqrt():
    res = query("RETURN sqrt(4) AS result")
    assert res.result_set == [[2]]

    res = query("RETURN sqrt(4.0) AS result")
    assert res.result_set == [[2.0]]

    res = query("RETURN sqrt(0) AS result")
    assert res.result_set == [[0]]

    res = query("RETURN sqrt(-1) AS result")
    assert math.isnan(res.result_set[0][0])

    res = query("RETURN sqrt(-1.0) AS result")
    assert math.isnan(res.result_set[0][0])

    res = query("RETURN sqrt(null) AS result")
    assert res.result_set == [[None]]


@given(st.integers(-100, 100), st.integers(-100, 100))
def test_range(a, b):
    res = query("RETURN range($a, $b)", params={"a": a, "b": b})
    assert res.result_set == [[list(range(a, b + 1))]]


@given(st.integers(-10, 10), st.integers(-100, 10), st.integers(-10, 10))
def test_range_step(a, b, c):
    if c == 0:
        query_exception("RETURN range($a, $b, $c)", "ArgumentError: step argument to range() can't be 0", params={"a": a, "b": b, "c": c})
        return
    res = query("RETURN range($a, $b, $c)", params={"a": a, "b": b, "c": c})
    if c > 0:
        if a == b:
            assert res.result_set == [[[a]]]
        else:
            assert res.result_set == [[[i for i in range(a, b + 1, c)]]]
    else:
        assert res.result_set == [[[i for i in range(a, b - 1, c)]]]


@given(st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st | st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st)))
def test_collect(a):
    res = query("UNWIND $a AS x RETURN collect(x)", params={"a": a})
    assert_result_set_equal_no_order(res, [[[x for x in a if x is not None]]])

    res = query("UNWIND $a AS x WITH collect(x) AS xs UNWIND xs AS y RETURN collect(y)", params={"a": a})
    assert_result_set_equal_no_order(res, [[[x for x in a if x is not None]]])

@given(st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st | st.lists(st.none() | st.booleans() | st.integers(-10, 10) | text_st)))
def test_count(a):
    res = query("UNWIND $a AS x RETURN count(x)", params={"a": a})
    assert_result_set_equal_no_order(res, [[len([x for x in a if x is not None])]])

@given(st.lists(st.none() | st.integers(-10, 10)))
def test_sum(a):
    res = query("UNWIND $a AS x RETURN sum(x)", params={"a": a})
    assert_result_set_equal_no_order(res, [[sum(x for x in a if x is not None)]])

    res = query("UNWIND $a AS x RETURN sum(distinct x)", params={"a": a})
    assert_result_set_equal_no_order(res, [[sum(set(x for x in a if x is not None))]])

@given(st.lists(st.none() | st.integers(-10, 10)))
def test_min(a):
    res = query("UNWIND $a AS x RETURN min(x)", params={"a": a})
    if not a or all(x is None for x in a):
        assert res.result_set == [[None]]
    else:
        assert res.result_set == [[min(x for x in a if x is not None)]]

@given(st.lists(st.none() | st.integers(-10, 10)))
def test_max(a):
    res = query("UNWIND $a AS x RETURN max(x)", params={"a": a})
    if not a or all(x is None for x in a):
        assert res.result_set == [[None]]
    else:
        assert res.result_set == [[max(x for x in a if x is not None)]]

def test_aggregation():
    res = query("UNWIND range(1, 10) AS x RETURN sum(x / 10.0)")
    assert_result_set_equal_no_order(res, [[5.5]])

    res = query("UNWIND range(1, 11) AS x RETURN x % 2, count(x)", compare_results=False)
    assert_result_set_equal_no_order(res, [[1, 6], [0, 5]])

    res = query("UNWIND range(1, 100) AS x RETURN min(x), max(x)")
    assert_result_set_equal_no_order(res, [[1, 100]])

    res = query("UNWIND range(1, 100) AS x RETURN {min: min(x), max: max(x)}")
    assert_result_set_equal_no_order(res, [[{"min": 1, "max": 100}]])
    
    res = query("UNWIND range(0,-1) AS a RETURN count(a), 1 + sum(a)")
    assert res.result_set == [[0, 1]]

    res = query("UNWIND [1, 2, 3, 1, 2, 3] AS x RETURN x % 2 = 0, sum(x), sum(distinct x)", compare_results=False)
    assert_result_set_equal_no_order(res, [[False, 8, 4], [True, 4, 2]])
    
    res = query("UNWIND [1, 2, 3, 1, 2, 3] AS x RETURN sum(x), sum(distinct x)", compare_results=False)
    assert_result_set_equal_no_order(res, [[12, 6]])
    

def test_case():
    res = query("RETURN CASE 1 + 2 WHEN 'a' THEN 1 END")
    assert res.result_set == [[None]]
    res = query("RETURN CASE WHEN 1 = 2 THEN 1 END")
    assert res.result_set == [[None]]
    res = query("RETURN CASE WHEN '1 = 2' THEN 1 END")
    assert res.result_set == [[1]]
    res = query("RETURN CASE 1 + 2 WHEN 'a' THEN 1 ELSE 2 END")
    assert res.result_set == [[2]]
    res = query("RETURN CASE WHEN 1 = 3 THEN 1 ELSE 2 END")
    assert res.result_set == [[2]]
    res = query("RETURN CASE 1 + 2 WHEN 3 THEN 1 + 2 WHEN 2 THEN 2 ELSE 2 END")
    assert res.result_set == [[3]]
    res = query("RETURN CASE WHEN False THEN 1 WHEN 1 = 1 THEN 1 + 1 WHEN 3 = 3 THEN 3 ELSE 2 END")
    assert res.result_set == [[2]]


def test_quantifier():
    # Test empty list
    res = query("RETURN all(x IN [] WHERE x > 0) AS res")
    assert res.result_set == [[True]]  # `all` on an empty list is True

    res = query("RETURN any(x IN [] WHERE x > 0) AS res")
    assert res.result_set == [[False]]  # `any` on an empty list is False

    res = query("RETURN none(x IN [] WHERE x > 0) AS res")
    assert res.result_set == [[True]]  # `none` on an empty list is True

    res = query("RETURN single(x IN [] WHERE x > 0) AS res")
    assert res.result_set == [[False]]  # `single` on an empty list is False

    # Test singleton list
    res = query("RETURN all(x IN [1] WHERE x > 0) AS res")
    assert res.result_set == [[True]]

    res = query("RETURN any(x IN [1] WHERE x > 0) AS res")
    assert res.result_set == [[True]]

    res = query("RETURN none(x IN [1] WHERE x > 0) AS res")
    assert res.result_set == [[False]]

    res = query("RETURN single(x IN [1] WHERE x > 0) AS res")
    assert res.result_set == [[True]]

    # Test non-boolean expressions
    q = "RETURN all(x IN [1, 2, 3] WHERE x + 1) AS res"
    query_exception(q, "Type mismatch: expected Boolean but was Integer")

    res = query("RETURN any(x IN [1, 2, 3] WHERE null) AS res")
    assert res.result_set == [[None]]

    res = query("RETURN none(x IN [1, 2, 3] WHERE null) AS res")
    assert res.result_set == [[None]]

    res = query("RETURN single(x IN [1, 2, 3] WHERE null) AS res")
    assert res.result_set == [[None]]

    # Test mixed boolean and null values
    res = query("RETURN all(x IN [true, null] WHERE x) AS res")
    assert res.result_set == [[None]]

    res = query("RETURN any(x IN [false, null] WHERE x) AS res")
    assert res.result_set == [[None]]

    res = query("RETURN none(x IN [false, null] WHERE x) AS res")
    assert res.result_set == [[None]]

    res = query("RETURN single(x IN [true, null] WHERE x) AS res")
    assert res.result_set == [[None]]


def test_list_comprehension():
    ## without where and without expr
    res = query("RETURN [x IN range(1, 10)] AS result")
    assert res.result_set == [[[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]]

    ## with where and without expr
    res = query("RETURN [x IN range(1, 10) WHERE x % 2 = 0] AS result")
    assert res.result_set == [[[2, 4, 6, 8, 10]]]

    ## with where and with expr
    res = query("RETURN [x IN range(1, 10) WHERE x % 2 = 0 | x + 1] AS result")
    assert res.result_set == [[[3, 5, 7, 9, 11]]]

    ## error in where
    q = "RETURN [x IN range(1, 10) WHERE x % 'a' = 2] AS result"
    query_exception(q, "Type mismatch: expected Integer, Float, or Null but was")

    ## error in expr
    q = "RETURN [x IN range(1, 10) WHERE x % 2 = 0 | x / 'a'] AS result"
    query_exception(q, "Type mismatch: expected Integer, Float, or Null but was")

    ## embedded
    res = query("RETURN [y IN [x IN range(1, 10)]] AS result")
    assert res.result_set == [[[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]]

    res = query("RETURN [x IN range(1, 10) | range(1, x)] AS result")
    expected = [[[list(range(1, i + 1)) for i in range(1, 11)]]]
    assert res.result_set == expected

    res = query("RETURN [x IN range(1, 10) WHERE x > 5] AS result")
    assert res.result_set == [[[6, 7, 8, 9, 10]]]

    res = query("RETURN [x IN range(1, 10) WHERE x < 5] AS result")
    assert res.result_set == [[[1, 2, 3, 4]]]

    res = query("RETURN [x IN range(1, 10) WHERE x = 5] AS result")
    assert res.result_set == [[[5]]]

    res = query("RETURN [x IN range(1, 10) WHERE x < 0] AS result")
    assert res.result_set == [[[]]]

    res = query("RETURN [x IN range(1, 10) WHERE x > 0] AS result")
    assert res.result_set == [[[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]]

    res = query("RETURN [x IN range(1, 10) WHERE x < -5] AS result")
    assert res.result_set == [[[]]]

