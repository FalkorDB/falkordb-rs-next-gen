import time
import redis
from graph_utils import graph_eq
from redis import ResponseError
from common import Env, FalkorDB, SANITIZER
from random_graph import create_random_schema, create_random_graph

GRAPH_ID = "graph_restore"

# tests the GRAPH.RESTORE command
class testGraphRestore():
    def __init__(self):
        self.env, self.db = Env(enableDebugCommand=True)
        self.conn = self.env.getConnection()

    def raw_conn(self):
        # DUMP payloads are binary, the default test connection decodes
        # replies as UTF-8 which corrupts them -- use a raw byte connection
        return redis.Redis(host=getattr(self.env, "host", "localhost"),
                           port=self.env.port, decode_responses=False)

    def test_01_invalid_invocation(self):
        src = GRAPH_ID
        src_graph = self.db.select_graph(src)
        src_graph.query("CREATE (:A {v: 1})")
        raw = self.raw_conn()
        payload = raw.execute_command("DUMP", src)

        # wrong number of arguments
        try:
            self.conn.execute_command("GRAPH.RESTORE", "dest")
            self.env.assertTrue(False)
        except ResponseError as e:
            self.env.assertContains("wrong number of arguments", str(e))

        try:
            raw.execute_command("GRAPH.RESTORE", "dest", payload, "extra")
            self.env.assertTrue(False)
        except ResponseError as e:
            self.env.assertContains("wrong number of arguments", str(e))

        # destination key already exists
        try:
            raw.execute_command("GRAPH.RESTORE", src, payload)
            self.env.assertTrue(False)
        except ResponseError as e:
            self.env.assertContains("key already exists", str(e))

        # corrupted DUMP payload, the key must not be created
        try:
            raw.execute_command("GRAPH.RESTORE", "corrupt", b"\x07\x00\x00garbage")
            self.env.assertTrue(False)
        except ResponseError:
            pass
        self.env.assertEqual(self.conn.exists("corrupt"), 0)

        # DUMP payload of a non-graph key, the key must not be created
        self.conn.set("plain_string", "just a plain string value")
        try:
            raw.execute_command("GRAPH.RESTORE", "not_a_graph",
                                raw.execute_command("DUMP", "plain_string"))
            self.env.assertTrue(False)
        except ResponseError:
            pass
        self.env.assertEqual(self.conn.exists("not_a_graph"), 0)

        self.conn.flushall()

    def test_02_restore_from_dump(self):
        # GRAPH.RESTORE loads a payload produced by the Redis DUMP command
        src = GRAPH_ID
        dest = GRAPH_ID + "_restored"

        src_graph = self.db.select_graph(src)
        nodes, edges = create_random_schema()
        create_random_graph(src_graph, nodes, edges)

        raw = self.raw_conn()
        payload = raw.execute_command("DUMP", src)
        raw.execute_command("GRAPH.RESTORE", dest, payload)

        self.env.assertEqual(self.conn.type(dest), 'graphdata')
        self.env.assertContains(dest, self.conn.execute_command("GRAPH.LIST"))

        dest_graph = self.db.select_graph(dest)
        self.env.assertTrue(graph_eq(src_graph, dest_graph))

        self.conn.flushall()

    def test_03_restored_graph_is_operational(self):
        # a restored graph must be writable, indexed and queryable
        src = GRAPH_ID
        dest = GRAPH_ID + "_restored"

        src_graph = self.db.select_graph(src)
        src_graph.query("""CREATE (a:P {name: 'alice', age: 30}),
                                  (b:P {name: 'bob', age: 25}),
                                  (a)-[:KNOWS {since: 2020}]->(b)""")
        src_graph.query("CREATE INDEX FOR (n:P) ON (n.name)")

        raw = self.raw_conn()
        payload = raw.execute_command("DUMP", src)
        raw.execute_command("GRAPH.RESTORE", dest, payload)
        dest_graph = self.db.select_graph(dest)

        # index survived and is used
        indexes = dest_graph.query("CALL db.indexes()").result_set
        self.env.assertEqual(len(indexes), 1)
        plan = str(dest_graph.explain("MATCH (n:P {name: 'alice'}) RETURN n"))
        self.env.assertContains("Index Scan", plan)

        # writes work and do not affect the source graph
        dest_graph.query("CREATE (:P {name: 'carol', age: 40})")
        self.env.assertEqual(
            dest_graph.query("MATCH (n:P) RETURN count(n)").result_set[0][0], 3)
        self.env.assertEqual(
            src_graph.query("MATCH (n:P) RETURN count(n)").result_set[0][0], 2)

        # the new node is reachable through the index
        self.env.assertEqual(
            dest_graph.query("MATCH (n:P {name: 'carol'}) RETURN n.age").result_set[0][0], 40)

        # deletes work
        dest_graph.query("MATCH (n) DETACH DELETE n")
        self.env.assertEqual(
            dest_graph.query("MATCH (n) RETURN count(n)").result_set[0][0], 0)

        self.conn.flushall()

    def test_04_restored_graph_survives_reload(self):
        # a graph restored into a different key must keep its own identity,
        # otherwise it collides with the source graph on RDB reload
        src = GRAPH_ID
        dest = GRAPH_ID + "_restored"

        src_graph = self.db.select_graph(src)
        src_graph.query("UNWIND range(1, 50) AS i CREATE (:A {id: i})")

        raw = self.raw_conn()
        payload = raw.execute_command("DUMP", src)
        raw.execute_command("GRAPH.RESTORE", dest, payload)

        dest_graph = self.db.select_graph(dest)
        dest_graph.query("CREATE (:B {id: 999})")

        # force multi virtual key encoding
        vkey_max_entity_count = self.db.config_get("VKEY_MAX_ENTITY_COUNT")
        self.db.config_set("VKEY_MAX_ENTITY_COUNT", 1)
        try:
            self.conn.execute_command("DEBUG", "RELOAD")
        finally:
            self.db.config_set("VKEY_MAX_ENTITY_COUNT", vkey_max_entity_count)

        self.env.assertEqual(
            src_graph.query("MATCH (n) RETURN count(n)").result_set[0][0], 50)
        self.env.assertEqual(
            dest_graph.query("MATCH (n) RETURN count(n)").result_set[0][0], 51)
        self.env.assertEqual(
            src_graph.query("MATCH (n:B) RETURN count(n)").result_set[0][0], 0)
        self.env.assertEqual(
            dest_graph.query("MATCH (n:B) RETURN count(n)").result_set[0][0], 1)

        self.conn.flushall()

    def test_05_replicated_restore(self):
        # skip test if we're running under sanitizer
        if SANITIZER:
            self.env.skip() # sanitizer is not working correctly with replication

        # make sure the GRAPH.RESTORE command is replicated

        # stop old environment
        self.env.stop()

        # start a new environment, one which have a master and a replica
        self.env, self.db = Env(env='oss', useSlaves=True)

        master_con = self.env.getConnection()
        self.conn = master_con

        src = GRAPH_ID
        dest = GRAPH_ID + "_restored"

        src_graph = self.db.select_graph(src)
        nodes, edges = create_random_schema()
        create_random_graph(src_graph, nodes, edges)

        raw = self.raw_conn()
        payload = raw.execute_command("DUMP", src)
        raw.execute_command("GRAPH.RESTORE", dest, payload)

        # the WAIT command forces master slave sync to complete
        master_con.execute_command("WAIT", "1", "0")

        replica_host = getattr(self.env, "replica_host", "localhost")
        replica_port = getattr(self.env, "replica_port", self.env.port + 1)
        replica_db = FalkorDB(replica_host, replica_port)
        replica_graph = replica_db.select_graph(dest)

        # the restored graph on the replica matches the source graph
        self.env.assertTrue(graph_eq(src_graph, replica_graph))

        # writes to the restored graph keep replicating
        dest_graph = self.db.select_graph(dest)
        dest_graph.query("CREATE (:REPLICATED {v: 1})")
        master_con.execute_command("WAIT", "1", "0")
        time.sleep(1)

        self.env.assertEqual(
            replica_graph.ro_query("MATCH (n:REPLICATED) RETURN count(n)").result_set[0][0], 1)
