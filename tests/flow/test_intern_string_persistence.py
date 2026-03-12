import random
import string
from common import *

GRAPH_ID = "intern_string"
SMALL_STRING = 'A'

def random_string(length=10):
    chars = string.ascii_letters + string.digits  # A-Z, a-z, 0-9
    return ''.join(random.choices(chars, k=length))

def assertStringPoolStats(conn, count, avg):
    import time
    time.sleep(0.01)
    stats = conn.execute_command("GRAPH.INFO", "ObjectPool")
    objs_in_pool   = int(stats[1][0][1])
    avg_ref_count = float(stats[1][1][1])
    assert avg_ref_count == avg, f"expected avg={avg}, got {avg_ref_count}"
    assert objs_in_pool == count, f"expected count={count}, got {objs_in_pool}"

class testInternStringPersistency():
    def __init__(self):
        self.env, self.db = Env(enableDebugCommand=True)
        self.conn = self.env.getConnection()

        # skip test if we're running under Sanitizer
        if SANITIZER:
            self.env.skip() # sanitizer is not working correctly with bulk

        # Synchronous deletion
        self.db.config_set('ASYNC_DELETE', 'no')

        # clear DB
        self.conn.flushall()

    def tearDown(self):
        # clear DB
        self.conn.flushall()
        self.graph = self.db.select_graph(GRAPH_ID)

        assertStringPoolStats(self.conn, 0, 0)

    def testInternStringPersistent(self):
        # populate DB

        # create first node
        q = "CREATE ({value: intern($s)})"

        # create multiple EMPTY graphs
        graphs = []
        for _ in range(0, 10):
            g = self.db.select_graph(random_string())
            g.query(q, {'s': SMALL_STRING})
            graphs.append(g)

        # validate string pool stats
        assertStringPoolStats(self.conn, 1, 10)

        # Save RDB & Load from RDB
        self.env.dumpAndReload()

        # string-pool stats expected to match former stats before reload
        assertStringPoolStats(self.conn, 1, 10)

        for g in graphs:
            res = g.query("MATCH (n) RETURN n.value").result_set[0][0]
            self.env.assertEqual(res, SMALL_STRING)
