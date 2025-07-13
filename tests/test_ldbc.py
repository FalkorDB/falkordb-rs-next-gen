import os
import subprocess

import common


def setup_module(module):
    common.start_redis()


def teardown_module(module):
    common.shutdown_redis()


def setup_function(function):
    if common.g.name in common.client.list_graphs():
        common.g.delete()


def test_load_csv():
    urls = [
        "https://repository.surfsara.nl/datasets/cwi/ldbc-snb-interactive-v1-datagen-v100/files/social_network-sf0.1-CsvBasic-LongDateFormatter.tar.zst"
    ]
    for url in urls:
        filename = url.split("/")[-1]
        if not os.path.exists(f"data/{filename}"):
            subprocess.run(
                ["wget", "--no-check-certificate", url, "-O", filename],
                check=True,
                stdin=subprocess.PIPE,
                cwd="data",
            )

            # decompress the file
            subprocess.run(
                ["zstd", "--decompress", filename],
                check=True,
                stdin=subprocess.PIPE,
                cwd="data",
            )

            # extract the tar file
            subprocess.run(
                ["tar", "-xf", filename], check=True, stdin=subprocess.PIPE, cwd="data"
            )

    files = [
        {
            "file": "static/organisation_0_0.csv",
            "label": "Organization",
            "properties": {
                "id": "toInteger(row.id)",
                "type": "row.type",
                "name": "row.name",
                "url": "row.url",
            },
            "expected": 7955,
        },
        {
            "file": "static/place_0_0.csv",
            "label": "Place",
            "properties": {
                "id": "toInteger(row.id)",
                "type": "row.type",
                "name": "row.name",
                "url": "row.url",
            },
            "expected": 1460,
        },
        {
            "file": "static/tag_0_0.csv",
            "label": "Tag",
            "properties": {
                "id": "toInteger(row.id)",
                "name": "row.name",
                "url": "row.url",
            },
            "expected": 16080,
        },
        {
            "file": "static/tagclass_0_0.csv",
            "label": "TagClass",
            "properties": {
                "id": "toInteger(row.id)",
                "name": "row.name",
                "url": "row.url",
            },
            "expected": 71,
        },
    ]
    for file in files:
        common.g.query(f"CREATE INDEX FOR (n:{file['label']}) ON n.id")
        properties_str = ", ".join(
            f"{key}: {value}" for key, value in file["properties"].items()
        )
        query = f"""
            LOAD CSV WITH HEADERS DELIMITER '|' FROM $file AS row
            CREATE (:{file['label']} {{{properties_str}}})
            """
        res = common.g.query(
            query,
            {
                "file": f"data/social_network-sf0.1-CsvBasic-LongDateFormatter/{file['file']}",
            },
        )
        assert res.nodes_created == file["expected"]

    files = [
        {
            "file": "static/organisation_isLocatedIn_place_0_0.csv",
            "type": "IS_LOCATED_IN",
            "properties": {
            },
            "from_label": "Organization",
            "to_label": "Place",
            "from_id": "Organisation.id",
            "to_id": "Place.id",
            "expected": 7955,
        },
    ]
    for file in files:
        properties_str = ", ".join(
            f"{key}: {value}" for key, value in file["properties"].items()
        )
        # query = f"""
        #     LOAD CSV WITH HEADERS DELIMITER '|' FROM $file AS row
        #     MATCH (f:{file['from_label']} {{id: toInteger(row.`{file['from_id']}`)}})
        #     WITH f, row
        #     MATCH (t:{file['to_label']} {{id: toInteger(row.`{file['to_id']}`)}})
        #     CREATE (f)-[r:{file['type']} {{{properties_str}}}]->(t)
        #     """
        query = f"""
            LOAD CSV WITH HEADERS DELIMITER '|' FROM $file AS row
            MATCH (f:{file['from_label']} {{id: toInteger(row.`{file['from_id']}`)}})
            WITH f, row
            MATCH (t:{file['to_label']} {{id: toInteger(row.`{file['to_id']}`)}})
            CREATE (f)-[r:{file['type']} {{{properties_str}}}]->(t)
            """
        res = common.g.query(
            query,
            {
                "file": f"data/social_network-sf0.1-CsvBasic-LongDateFormatter/{file['file']}",
            },
        )
        assert res.relationships_created == file["expected"]
        print(res.run_time_ms)
