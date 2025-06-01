import common


def setup_module(module):
    common.start_redis()


def teardown_module(module):
    common.shutdown_redis()


def setup_function(function):
    if common.g.name in common.client.list_graphs():
        common.g.delete()

def query(query: str, params=None):
    common.g.query(query, params)

def test_return(benchmark):
    benchmark(query, "RETURN 1")

def test_unwind(benchmark):
    benchmark(query, "UNWIND range(1, 1000000) AS x RETURN x")