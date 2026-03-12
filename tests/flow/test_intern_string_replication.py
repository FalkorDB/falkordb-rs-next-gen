import time
from common import *

GRAPH_ID = "intern_string"
LARGE_STRING = 'A' * 16384

def assertStringPoolStats(conn, count, avg):
    time.sleep(0.01)
    stats = conn.execute_command("GRAPH.INFO", "ObjectPool")
    objs_in_pool   = int(stats[1][0][1])
    avg_ref_count = float(stats[1][1][1])
    assert avg_ref_count == avg, f"expected avg={avg}, got {avg_ref_count}"
    assert objs_in_pool == count, f"expected count={count}, got {objs_in_pool}"

class testInternStringReplication():
    def __init__(self):
        # skip test if we're running under sanitizer
        if SANITIZER:
            Environment.skip(None) # sanitizer is not working correctly with replication

        self.env, self.db = Env(env='oss', useSlaves=True)
        self.conn = self.env.getConnection()
        self.graph = self.db.select_graph(GRAPH_ID)

        self.source_con = self.env.getConnection()
        self.replica_con = self.env.getSlaveConnection()

        # force effects replication
        self.db.config_set('EFFECTS_THRESHOLD', 0)

        # Synchronous deletion
        self.source_con.execute_command("GRAPH.CONFIG", "SET", 'ASYNC_DELETE', 'no')
        self.replica_con.execute_command("GRAPH.CONFIG", "SET", 'ASYNC_DELETE', 'no')

        # clear DB
        self.conn.flushall()
        self.source_con.execute_command("WAIT", "1", "0")

    def tearDown(self):
        # clear DB
        self.source_con.flushall()
        self.source_con.execute_command("WAIT", "1", "0")

        assertStringPoolStats(self.source_con, 0, 0)
        assertStringPoolStats(self.replica_con, 0, 0)

    def query_and_wait(self, q, p=None):
        if p is None:
            p = {}

        res = self.graph.query(q, p)

        # the WAIT command forces master slave sync to complete
        self.source_con.execute_command("WAIT", "1", "0")

        return res

    def test_intern_string_replication(self):
        # both master and replica should be empty
        assertStringPoolStats(self.source_con, 0, 0)
        assertStringPoolStats(self.replica_con, 0, 0)

        # replicate a node creation containing an intern string

        s = LARGE_STRING

        # create first node
        p = {'s': s}
        q = "CREATE ({value: intern($s)})"
        self.query_and_wait(q, p)

        assertStringPoolStats(self.source_con, 1, 1)
        assertStringPoolStats(self.replica_con, 1, 1)

        # replicate an addition of an intern string
        q = "MATCH (n) SET n.s = intern($s)"
        self.query_and_wait(q, p)

        assertStringPoolStats(self.source_con, 1, 2)
        assertStringPoolStats(self.replica_con, 1, 2)

        # replicate deletion of an intern string
        q = "MATCH (n) SET n.s = null"
        self.query_and_wait(q)

        assertStringPoolStats(self.source_con, 1, 1)
        assertStringPoolStats(self.replica_con, 1, 1)

        # replicate update of an intern string
        q = "MATCH (n) SET n.value = intern('intern-string')"
        self.query_and_wait(q)

        assertStringPoolStats(self.source_con, 1, 1)
        assertStringPoolStats(self.replica_con, 1, 1)

        # replicate deletion if a node
        q = "MATCH (n) DELETE n"
        self.query_and_wait(q)

        assertStringPoolStats(self.source_con, 0, 0)
        assertStringPoolStats(self.replica_con, 0, 0)
