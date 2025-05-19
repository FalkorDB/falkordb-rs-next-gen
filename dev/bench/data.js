window.BENCHMARK_DATA = {
  "lastUpdate": 1747644986315,
  "repoUrl": "https://github.com/FalkorDB/falkordb-rs-next-gen",
  "entries": {
    "FalkorDB Benchmark": [
      {
        "commit": {
          "author": {
            "email": "avi.avni@gmail.com",
            "name": "Avi Avni",
            "username": "AviAvni"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5c267adfb54385a366483a5d0ad484a921c55f65",
          "message": "Merge pull request #63 from FalkorDB/benchmark\n\nadd permissions",
          "timestamp": "2025-05-08T20:23:11+03:00",
          "tree_id": "6cb73a745f5c0b9bd80a7273b88aeb205cfd6d7c",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/5c267adfb54385a366483a5d0ad484a921c55f65"
        },
        "date": 1746725301736,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 9601.37359550969,
            "unit": "iter/sec",
            "range": "stddev: 0.00001626636160911111",
            "extra": "mean: 104.15176433376925 usec\nrounds: 2529"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.20452450646313428,
            "unit": "iter/sec",
            "range": "stddev: 0.06944593012166002",
            "extra": "mean: 4.889389625200005 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "avi.avni@gmail.com",
            "name": "Avi Avni",
            "username": "AviAvni"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5774ecc3229219e498b8a579fc21cbd981b62e14",
          "message": "Merge pull request #61 from FalkorDB/improve-keyword-parsing\n\nimprove keyword parsing",
          "timestamp": "2025-05-08T20:39:11+03:00",
          "tree_id": "0bd21e467a87050fed486d0c43783d421a6ea650",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/5774ecc3229219e498b8a579fc21cbd981b62e14"
        },
        "date": 1746726104271,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 8328.396275264748,
            "unit": "iter/sec",
            "range": "stddev: 0.000024455591119047573",
            "extra": "mean: 120.07113578036504 usec\nrounds: 2261"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.19466840401271499,
            "unit": "iter/sec",
            "range": "stddev: 0.047662396614272094",
            "extra": "mean: 5.136940455599995 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "barak.bar@gmail.com",
            "name": "Barak Bar Orion",
            "username": "barakb"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2158f673077e5dff85819fbcbf0543b2f1d0fd42",
          "message": "Update README.md",
          "timestamp": "2025-05-08T20:54:04+03:00",
          "tree_id": "5fc3900b65f14ced1d809e41b4ecd15c0911b8e5",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/2158f673077e5dff85819fbcbf0543b2f1d0fd42"
        },
        "date": 1746726993580,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 10194.114636120776,
            "unit": "iter/sec",
            "range": "stddev: 0.000008060156152745754",
            "extra": "mean: 98.09581662508512 usec\nrounds: 2803"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.20381076928867195,
            "unit": "iter/sec",
            "range": "stddev: 0.02921117992754837",
            "extra": "mean: 4.9065120724 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "avi.avni@gmail.com",
            "name": "Avi Avni",
            "username": "AviAvni"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "065c0f6fc77400d64f21acae7a2a265035e48e22",
          "message": "Merge pull request #64 from FalkorDB/iterator-runtime\n\nimplement iterator runtime",
          "timestamp": "2025-05-11T17:20:15+03:00",
          "tree_id": "1a488106bd9cdc04140fd175ffb61cc011757281",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/065c0f6fc77400d64f21acae7a2a265035e48e22"
        },
        "date": 1746973360298,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 9286.517813922279,
            "unit": "iter/sec",
            "range": "stddev: 0.000017203325568769133",
            "extra": "mean: 107.68298947327784 usec\nrounds: 2470"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.2126517260070433,
            "unit": "iter/sec",
            "range": "stddev: 0.07255796180498388",
            "extra": "mean: 4.702524727999991 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "avi.avni@gmail.com",
            "name": "Avi Avni",
            "username": "AviAvni"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bb725bf73f7579908facc88ac69a984305c9946b",
          "message": "Merge pull request #65 from FalkorDB/simplify-runtime\n\nsimplify runtime",
          "timestamp": "2025-05-12T11:50:18+03:00",
          "tree_id": "c94cc3d0af54b104a175cb55589a92610b711430",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/bb725bf73f7579908facc88ac69a984305c9946b"
        },
        "date": 1747039970749,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 10655.43943400501,
            "unit": "iter/sec",
            "range": "stddev: 0.00001309007347588695",
            "extra": "mean: 93.8487808216216 usec\nrounds: 2774"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.2109548927087914,
            "unit": "iter/sec",
            "range": "stddev: 0.0631738066749367",
            "extra": "mean: 4.74034987839998 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "barak.bar@gmail.com",
            "name": "Barak Bar Orion",
            "username": "barakb"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cd0e41ed352bd4bff4b698e7960d7a151c5ce7ee",
          "message": "Implement TCK expressions/boolean (#66)\n\n* Implement TCK expressions/boolean\n\n* fix keyword parsing\n\n* Use TCK_DONE with code coverage\n\n---------\n\nCo-authored-by: Avi Avni <avi.avni@gmail.com>",
          "timestamp": "2025-05-13T11:18:17+03:00",
          "tree_id": "ea7585ccaa0c4ae20d1f1e9ad8d1e3b7058cc77d",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/cd0e41ed352bd4bff4b698e7960d7a151c5ce7ee"
        },
        "date": 1747124450562,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 10005.914217570522,
            "unit": "iter/sec",
            "range": "stddev: 0.000015630228067896316",
            "extra": "mean: 99.94089278158975 usec\nrounds: 2369"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.20763380192182013,
            "unit": "iter/sec",
            "range": "stddev: 0.049867191681219565",
            "extra": "mean: 4.816171503600015 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "barak.bar@gmail.com",
            "name": "Barak Bar Orion",
            "username": "barakb"
          },
          "committer": {
            "email": "barak.bar@gmail.com",
            "name": "Barak Bar Orion",
            "username": "barakb"
          },
          "distinct": true,
          "id": "0881e72417e9df86649b444ed65c221fa484a173",
          "message": "Use TCK_DONE with code coverage",
          "timestamp": "2025-05-13T11:58:13+03:00",
          "tree_id": "ff2adc53eb416dbe8cf8b97bde22527e5eae6175",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/0881e72417e9df86649b444ed65c221fa484a173"
        },
        "date": 1747127307385,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 9478.697346819925,
            "unit": "iter/sec",
            "range": "stddev: 0.000017245876659078093",
            "extra": "mean: 105.49972885625441 usec\nrounds: 2353"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.21125014203978978,
            "unit": "iter/sec",
            "range": "stddev: 0.033495167813967976",
            "extra": "mean: 4.733724627799996 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "avi.avni@gmail.com",
            "name": "Avi Avni",
            "username": "AviAvni"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a2fa0871f4323767369c3041f77cbabd7e0de3d9",
          "message": "Merge pull request #67 from FalkorDB/crud\n\nWork on create tck",
          "timestamp": "2025-05-13T15:38:17+03:00",
          "tree_id": "4f7387b97e2e8e15df1fb39021e13357dd6f554c",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/a2fa0871f4323767369c3041f77cbabd7e0de3d9"
        },
        "date": 1747140049813,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 10685.421516288347,
            "unit": "iter/sec",
            "range": "stddev: 0.000009988727216261484",
            "extra": "mean: 93.58545177423723 usec\nrounds: 2198"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.2081961086149698,
            "unit": "iter/sec",
            "range": "stddev: 0.04839856461639859",
            "extra": "mean: 4.803163741400004 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "avi.avni@gmail.com",
            "name": "Avi Avni",
            "username": "AviAvni"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c9522457432e34127f00e1f8011e0e9d792fbc13",
          "message": "Merge pull request #69 from FalkorDB/crud\n\nimplement TCK Create 1-4",
          "timestamp": "2025-05-19T11:39:22+03:00",
          "tree_id": "b46227b99ee90e261880422449ba6707a0ad6613",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/c9522457432e34127f00e1f8011e0e9d792fbc13"
        },
        "date": 1747644985836,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 10757.79875277398,
            "unit": "iter/sec",
            "range": "stddev: 0.000009355897170499418",
            "extra": "mean: 92.95581958550233 usec\nrounds: 2461"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.20786032095661347,
            "unit": "iter/sec",
            "range": "stddev: 0.024779424875536443",
            "extra": "mean: 4.810923005400002 sec\nrounds: 5"
          }
        ]
      }
    ]
  }
}