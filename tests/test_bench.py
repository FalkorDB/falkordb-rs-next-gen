import pytest
import common


def setup_module(module):
    common.start_redis(release=True)


def teardown_module(module):
    common.shutdown_redis()


def setup_function(function):
    if common.g.name in common.client.list_graphs():
        common.g.delete()

def query(query: str, params=None):
    common.g.query(query, params)

def test_return(benchmark):
    benchmark(query, "RETURN 1")

@pytest.mark.parametrize("n", [
    1, 10, 100, 1000, 10000, 100000, 1000000
])
def test_unwind(benchmark, n):
    benchmark(query, f"UNWIND range(1, {n}) AS x RETURN x")

@pytest.mark.parametrize("n", [
    1, 10, 100, 1000, 10000, 100000, 1000000
])
def test_create_node(benchmark, n):
    benchmark(query, f"UNWIND range(1, {n}) AS x CREATE (:N {{id: x}})")

@pytest.mark.parametrize("n", [
    1, 10, 100, 1000, 10000, 100000, 1000000
])
def test_create_relationship(benchmark, n):
    benchmark(query, f"UNWIND range(1, {n}) AS x CREATE (:N {{id: x}})-[:R]->(:N {{id: x + 1}})")

@pytest.mark.parametrize("n", [
    1, 10, 100, 1000, 10000, 100000, 1000000
])
def test_match_node(benchmark, n):
    query(f"UNWIND range(1, {n}) AS x CREATE (:N {{id: x}})")
    benchmark(query, f"MATCH (n:N) RETURN n")

@pytest.mark.parametrize("n", [
    1, 10, 100, 1000, 10000, 100000, 1000000
])
def test_match_relationship(benchmark, n):
    query(f"UNWIND range(1, {n}) AS x CREATE (:N {{id: x}})-[:R]->(:N {{id: x + 1}})")
    benchmark(query, f"MATCH (n)-[r:R]->(m) RETURN n, r, m")

@pytest.mark.parametrize("n", [
    1, 10, 100, 1000, 10000, 100000, 1000000
])
def test_delete_node(benchmark, n):
    query(f"UNWIND range(1, {n}) AS x CREATE (:N {{id: x}})")
    benchmark(query, f"MATCH (n:N) DELETE n")

@pytest.mark.parametrize("n", [
    1, 10, 100, 1000, 10000, 100000, 1000000
])
def test_delete_relationship(benchmark, n):
    query(f"UNWIND range(1, {n}) AS x CREATE (:N {{id: x}})-[:R]->(:N {{id: x + 1}})")
    benchmark(query, f"MATCH (n)-[r:R]->(m) DELETE r")