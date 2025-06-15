window.BENCHMARK_DATA = {
  "lastUpdate": 1750000239424,
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
          "id": "4844b3c218dee305556bcd94e33d850dcdc2c431",
          "message": "Merge pull request #68 from FalkorDB/fix-numeric-edge-case\n\nfix numeric edge case",
          "timestamp": "2025-05-19T13:38:51+03:00",
          "tree_id": "9c27a4a048f6c96f36809f7f87d30d90e11628f6",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/4844b3c218dee305556bcd94e33d850dcdc2c431"
        },
        "date": 1747651284295,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 9342.354441195903,
            "unit": "iter/sec",
            "range": "stddev: 0.00001690550537495136",
            "extra": "mean: 107.03939850434439 usec\nrounds: 2542"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.2101170522192479,
            "unit": "iter/sec",
            "range": "stddev: 0.048428221205754074",
            "extra": "mean: 4.7592519952000085 sec\nrounds: 5"
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
          "id": "44446dc77d1dc93629c7b68defc61696080b22c7",
          "message": "add aggregation tck (#70)",
          "timestamp": "2025-05-19T14:50:47+03:00",
          "tree_id": "3e2b1e9cd307ac47122c0bfa36a57ef29f7325d0",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/44446dc77d1dc93629c7b68defc61696080b22c7"
        },
        "date": 1747655599672,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 10797.135569315084,
            "unit": "iter/sec",
            "range": "stddev: 0.000010470161373810654",
            "extra": "mean: 92.61715698392726 usec\nrounds: 1618"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.21562851454101817,
            "unit": "iter/sec",
            "range": "stddev: 0.02857558652159948",
            "extra": "mean: 4.637605569599998 sec\nrounds: 5"
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
          "id": "abebc5672bb5cdffafe97aca76a3b8be180dc903",
          "message": "add list and map tck (#71)",
          "timestamp": "2025-05-20T13:50:43+03:00",
          "tree_id": "0108b1187ae4aa1f86d121e04f2a5979fa011c44",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/abebc5672bb5cdffafe97aca76a3b8be180dc903"
        },
        "date": 1747738400892,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 9373.741516793614,
            "unit": "iter/sec",
            "range": "stddev: 0.000016537936081860833",
            "extra": "mean: 106.68098733130635 usec\nrounds: 2447"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.21159070638285765,
            "unit": "iter/sec",
            "range": "stddev: 0.03418021304354492",
            "extra": "mean: 4.726105494399997 sec\nrounds: 5"
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
          "id": "16897060d107db854f0e3811e6284ea104beffb0",
          "message": "implement null tck (#73)",
          "timestamp": "2025-05-21T08:32:13+03:00",
          "tree_id": "b74da1ef1a0f59789d9f1371a41f8b63c777022d",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/16897060d107db854f0e3811e6284ea104beffb0"
        },
        "date": 1747805685904,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 9542.426809329398,
            "unit": "iter/sec",
            "range": "stddev: 0.000016521378706321405",
            "extra": "mean: 104.79514488099866 usec\nrounds: 2471"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.21180128411860988,
            "unit": "iter/sec",
            "range": "stddev: 0.06896615285709451",
            "extra": "mean: 4.721406691000015 sec\nrounds: 5"
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
          "id": "b52538be7031decb26311231a2a12960080d295f",
          "message": "Merge pull request #72 from FalkorDB/tck-case\n\nexpressions/conditional, fix #50",
          "timestamp": "2025-05-21T13:06:28+03:00",
          "tree_id": "5cf78e40e93546d2b7432861292f171336aa1445",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/b52538be7031decb26311231a2a12960080d295f"
        },
        "date": 1747822141535,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 10064.419528547622,
            "unit": "iter/sec",
            "range": "stddev: 0.000016278246429727917",
            "extra": "mean: 99.35992802799112 usec\nrounds: 1709"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.20876799979985522,
            "unit": "iter/sec",
            "range": "stddev: 0.05517709127635773",
            "extra": "mean: 4.7900061358000015 sec\nrounds: 5"
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
          "id": "72eec303d06f254a4fc716d6f083d8bb65c468e8",
          "message": "implement comparison and add precedence tck (#74)",
          "timestamp": "2025-05-21T13:50:10+03:00",
          "tree_id": "d036ccb46ef3d0c2a652be33f8d8c90e31f61bde",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/72eec303d06f254a4fc716d6f083d8bb65c468e8"
        },
        "date": 1747824762606,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 10807.632554294973,
            "unit": "iter/sec",
            "range": "stddev: 0.000013005478572854597",
            "extra": "mean: 92.52720195438161 usec\nrounds: 2456"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.2080244369281642,
            "unit": "iter/sec",
            "range": "stddev: 0.05419247063841905",
            "extra": "mean: 4.807127541199998 sec\nrounds: 5"
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
          "id": "0513b06b4e5fe5589a34b147bb309ee9cdc439d9",
          "message": "refactor runtime to return the environment each time (#76)\n\n* refactor runtime to return the environment each time\n\n* rc string\n\n* use hashbrown map\n\n* fix hash for env\n\n* address review\n\n* use Rc for strings\n\n* use take while\n\n* remove to string\n\n* allow multiple aggregation\n\n* fix short circuit map and flatmap",
          "timestamp": "2025-05-25T13:09:51+03:00",
          "tree_id": "8091fcdd1f4ab0926b5235e5de72e8c673f44ae5",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/0513b06b4e5fe5589a34b147bb309ee9cdc439d9"
        },
        "date": 1748167945879,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 10764.309655497113,
            "unit": "iter/sec",
            "range": "stddev: 0.000009248119235279944",
            "extra": "mean: 92.89959430787282 usec\nrounds: 2354"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.20933593323672167,
            "unit": "iter/sec",
            "range": "stddev: 0.021076769486507555",
            "extra": "mean: 4.7770107336000365 sec\nrounds: 5"
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
            "email": "avi.avni@gmail.com",
            "name": "Avi Avni",
            "username": "AviAvni"
          },
          "distinct": true,
          "id": "3905989a861cc3532b66d30525de75432ebe7274",
          "message": "fix",
          "timestamp": "2025-05-25T13:29:22+03:00",
          "tree_id": "06d4704f75dee07706d070855593dc9e7adaf6dd",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/3905989a861cc3532b66d30525de75432ebe7274"
        },
        "date": 1748171746142,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 10187.142244954866,
            "unit": "iter/sec",
            "range": "stddev: 0.000015307845041070306",
            "extra": "mean: 98.16295639684871 usec\nrounds: 2087"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.19498477822198088,
            "unit": "iter/sec",
            "range": "stddev: 0.036988290220584336",
            "extra": "mean: 5.128605469199999 sec\nrounds: 5"
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
          "id": "9fcbc57d6880ca8e81372cef289c3558f1d932d0",
          "message": "Merge pull request #75 from FalkorDB/tck_quantifier\n\ntck quantifier",
          "timestamp": "2025-05-25T17:16:03+03:00",
          "tree_id": "9f697cd53a8fb2b884960944e34e34b4d1d02948",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/9fcbc57d6880ca8e81372cef289c3558f1d932d0"
        },
        "date": 1748182718306,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 10665.786433528388,
            "unit": "iter/sec",
            "range": "stddev: 0.000007989288025125493",
            "extra": "mean: 93.75773706254367 usec\nrounds: 2628"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.2067749148859724,
            "unit": "iter/sec",
            "range": "stddev: 0.06964956670855851",
            "extra": "mean: 4.836176576600008 sec\nrounds: 5"
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
            "email": "avi.avni@gmail.com",
            "name": "Avi Avni",
            "username": "AviAvni"
          },
          "distinct": true,
          "id": "dde7a1a5a5604fe8e539f18d4a2e334d5e844c02",
          "message": "fix precedence",
          "timestamp": "2025-05-25T17:28:54+03:00",
          "tree_id": "2e331ed42b5db3710c75bbff2d95594bee7b8e41",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/dde7a1a5a5604fe8e539f18d4a2e334d5e844c02"
        },
        "date": 1748183494939,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 9686.638722414957,
            "unit": "iter/sec",
            "range": "stddev: 0.000017605697810467072",
            "extra": "mean: 103.23498466872643 usec\nrounds: 2870"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.2050130607600918,
            "unit": "iter/sec",
            "range": "stddev: 0.02362609994696313",
            "extra": "mean: 4.8777380147999905 sec\nrounds: 5"
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
          "id": "65bda6bc3ced936c9bb49a5540e2cd0025c627bc",
          "message": "Merge pull request #80 from FalkorDB/delete-tck\n\ndelete tck",
          "timestamp": "2025-05-30T09:03:45+03:00",
          "tree_id": "8f614b97d71d658b8dc42794a54c6498be47d643",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/65bda6bc3ced936c9bb49a5540e2cd0025c627bc"
        },
        "date": 1748585189764,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 10940.002304494605,
            "unit": "iter/sec",
            "range": "stddev: 0.000008387648744931037",
            "extra": "mean: 91.4076589900862 usec\nrounds: 2258"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.20863436055114454,
            "unit": "iter/sec",
            "range": "stddev: 0.09274264639105237",
            "extra": "mean: 4.793074340000004 sec\nrounds: 5"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "barak.bar@gmail.com",
            "name": "Barak Bar Orion",
            "username": "barakb"
          },
          "distinct": true,
          "id": "1cd5f27b2dfbc71e00a99e30e2e9679b909095f5",
          "message": "Bump orx-tree from 1.5.0 to 1.6.0\n\nBumps [orx-tree](https://github.com/orxfun/orx-tree) from 1.5.0 to 1.6.0.\n- [Release notes](https://github.com/orxfun/orx-tree/releases)\n- [Commits](https://github.com/orxfun/orx-tree/compare/1.5.0...1.6.0)\n\n---\nupdated-dependencies:\n- dependency-name: orx-tree\n  dependency-version: 1.6.0\n  dependency-type: direct:production\n  update-type: version-update:semver-minor\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>",
          "timestamp": "2025-06-03T08:20:44+03:00",
          "tree_id": "aa33a819be2217d2e2656626a4747c4ab086e516",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/1cd5f27b2dfbc71e00a99e30e2e9679b909095f5"
        },
        "date": 1748928202718,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/bench.py::test_return",
            "value": 10178.662794755704,
            "unit": "iter/sec",
            "range": "stddev: 0.00001616772812742006",
            "extra": "mean: 98.244732158258 usec\nrounds: 2270"
          },
          {
            "name": "tests/bench.py::test_unwind",
            "value": 0.2036503412538119,
            "unit": "iter/sec",
            "range": "stddev: 0.030966556418318324",
            "extra": "mean: 4.9103772370000005 sec\nrounds: 5"
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
          "id": "2fc1385083a84ece121f861c00e48c631eb4309b",
          "message": "Merge pull request #81 from FalkorDB/function-arguments-validation\n\nFunction arguments validation",
          "timestamp": "2025-06-03T14:21:37+03:00",
          "tree_id": "9798ee65b017710764afdccb01ebab988494b290",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/2fc1385083a84ece121f861c00e48c631eb4309b"
        },
        "date": 1748949856524,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 10486.973528046274,
            "unit": "iter/sec",
            "range": "stddev: 0.000009177728652137063",
            "extra": "mean: 95.35639594451234 usec\nrounds: 3748"
          },
          {
            "name": "tests/test_bench.py::test_unwind",
            "value": 0.2004338807690505,
            "unit": "iter/sec",
            "range": "stddev: 0.03504229440236635",
            "extra": "mean: 4.989176461399995 sec\nrounds: 5"
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
          "id": "cc94d693d70fcc29bfab486901d8a3166af0d415",
          "message": "Merge pull request #85 from FalkorDB/alert-autofix-33\n\nPotential fix for code scanning alert no. 33: Workflow does not contain permissions",
          "timestamp": "2025-06-04T09:36:27+03:00",
          "tree_id": "3a67edeff7390bce820d5160472d810a6a4a31b5",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/cc94d693d70fcc29bfab486901d8a3166af0d415"
        },
        "date": 1749019144891,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 9246.27348615458,
            "unit": "iter/sec",
            "range": "stddev: 0.000019794736007021114",
            "extra": "mean: 108.15167878144699 usec\nrounds: 2724"
          },
          {
            "name": "tests/test_bench.py::test_unwind",
            "value": 0.21132211994272385,
            "unit": "iter/sec",
            "range": "stddev: 0.0650594633095819",
            "extra": "mean: 4.732112285599999 sec\nrounds: 5"
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
          "id": "1611d919c3f73913d192a69a7c060b63181f4285",
          "message": "Merge pull request #84 from FalkorDB/alert-autofix-49\n\nPotential fix for code scanning alert no. 49: Workflow does not contain permissions",
          "timestamp": "2025-06-04T09:41:03+03:00",
          "tree_id": "25023a0d1582bebc4e424efd7bd3f7a952af05a2",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/1611d919c3f73913d192a69a7c060b63181f4285"
        },
        "date": 1749019421035,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 10163.254491070034,
            "unit": "iter/sec",
            "range": "stddev: 0.000011381525998628586",
            "extra": "mean: 98.39367900101806 usec\nrounds: 2405"
          },
          {
            "name": "tests/test_bench.py::test_unwind",
            "value": 0.2023013165948369,
            "unit": "iter/sec",
            "range": "stddev: 0.02819008211053131",
            "extra": "mean: 4.943121561599969 sec\nrounds: 5"
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
          "id": "2cb05c9915a47056fac857577e61ffc5953dc640",
          "message": "Potential fix for code scanning alert no. 26: Workflow does not contain permissions\n\nCo-authored-by: Copilot Autofix powered by AI <62310815+github-advanced-security[bot]@users.noreply.github.com>",
          "timestamp": "2025-06-04T17:01:29+03:00",
          "tree_id": "99a619df06054bc2e0e5c8bfc00bf394dd3c5e96",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/2cb05c9915a47056fac857577e61ffc5953dc640"
        },
        "date": 1749045846167,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 8473.226098754209,
            "unit": "iter/sec",
            "range": "stddev: 0.000013782663956270824",
            "extra": "mean: 118.01880279661447 usec\nrounds: 3433"
          },
          {
            "name": "tests/test_bench.py::test_unwind",
            "value": 0.21191182842658518,
            "unit": "iter/sec",
            "range": "stddev: 0.06483200258084738",
            "extra": "mean: 4.718943757999995 sec\nrounds: 5"
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
          "id": "d2f88b79d248adf290f1ec54aa48a1d3edaa7dd4",
          "message": "Revert \"pin docker/setup-qemu-action to full sha\"\n\nThis reverts commit c8d534d9dae7191cf2263ce06913ae89e220c716.",
          "timestamp": "2025-06-04T17:53:25+03:00",
          "tree_id": "96d3f475fbd9b1bf57e0de554ea3f1b97b6da1c6",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/d2f88b79d248adf290f1ec54aa48a1d3edaa7dd4"
        },
        "date": 1749048970248,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 9819.749786592698,
            "unit": "iter/sec",
            "range": "stddev: 0.000016247424136037996",
            "extra": "mean: 101.83558865882107 usec\nrounds: 2222"
          },
          {
            "name": "tests/test_bench.py::test_unwind",
            "value": 0.20258838892998257,
            "unit": "iter/sec",
            "range": "stddev: 0.020778913194591186",
            "extra": "mean: 4.9361170464 sec\nrounds: 5"
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
          "id": "0e9ec8eca5d66acbaaabff1b5c3d04a44cef4a71",
          "message": "Merge pull request #89 from FalkorDB/dependabot/cargo/orx-tree-1.7.1\n\nBump orx-tree from 1.6.0 to 1.7.1",
          "timestamp": "2025-06-09T09:18:30+03:00",
          "tree_id": "077b352fcf33fa65d9b40f0f59e4df12ca927770",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/0e9ec8eca5d66acbaaabff1b5c3d04a44cef4a71"
        },
        "date": 1749450066407,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 10592.831466410591,
            "unit": "iter/sec",
            "range": "stddev: 0.000008647995262174926",
            "extra": "mean: 94.40346551070473 usec\nrounds: 2595"
          },
          {
            "name": "tests/test_bench.py::test_unwind",
            "value": 0.21041565697284553,
            "unit": "iter/sec",
            "range": "stddev: 0.07180444320667612",
            "extra": "mean: 4.752498052600009 sec\nrounds: 5"
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
          "id": "01b42ef48f2479cbd7c7abbf0c7c72bb1751bb28",
          "message": "Merge pull request #88 from FalkorDB/dependabot/cargo/hashbrown-0.15.4\n\nBump hashbrown from 0.15.3 to 0.15.4",
          "timestamp": "2025-06-09T09:22:49+03:00",
          "tree_id": "78be782cebb5ede07651f36c2737d5fcbf075094",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/01b42ef48f2479cbd7c7abbf0c7c72bb1751bb28"
        },
        "date": 1749450332793,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 9870.368571923998,
            "unit": "iter/sec",
            "range": "stddev: 0.000014348896607326367",
            "extra": "mean: 101.31333928547244 usec\nrounds: 2632"
          },
          {
            "name": "tests/test_bench.py::test_unwind",
            "value": 0.20535931975871097,
            "unit": "iter/sec",
            "range": "stddev: 0.07384993909769146",
            "extra": "mean: 4.869513597799994 sec\nrounds: 5"
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
          "id": "20b5aa8fd9dc8cb109673c80a805d70c88899c56",
          "message": "Merge pull request #83 from FalkorDB/match-tck\n\nwork on match tck",
          "timestamp": "2025-06-09T09:36:22+03:00",
          "tree_id": "625509c9aa6e1c34a4c1890c5177a3eff9b3160e",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/20b5aa8fd9dc8cb109673c80a805d70c88899c56"
        },
        "date": 1749451691602,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 10382.798972113482,
            "unit": "iter/sec",
            "range": "stddev: 0.00001191077405566588",
            "extra": "mean: 96.31314279375322 usec\nrounds: 2255"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1]",
            "value": 9346.811712940316,
            "unit": "iter/sec",
            "range": "stddev: 0.000017397577294326166",
            "extra": "mean: 106.98835396625535 usec\nrounds: 4992"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10]",
            "value": 7265.389638655046,
            "unit": "iter/sec",
            "range": "stddev: 0.000016883189368820186",
            "extra": "mean: 137.63886725077526 usec\nrounds: 4226"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100]",
            "value": 1996.823139718609,
            "unit": "iter/sec",
            "range": "stddev: 0.000023513457204956837",
            "extra": "mean: 500.79547863258404 usec\nrounds: 1638"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000]",
            "value": 232.34676145771076,
            "unit": "iter/sec",
            "range": "stddev: 0.001136847851128291",
            "extra": "mean: 4.303911936306498 msec\nrounds: 157"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10000]",
            "value": 23.62021166935485,
            "unit": "iter/sec",
            "range": "stddev: 0.0033339435824477574",
            "extra": "mean: 42.33662314285745 msec\nrounds: 21"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100000]",
            "value": 2.194292441991388,
            "unit": "iter/sec",
            "range": "stddev: 0.00932227998420756",
            "extra": "mean: 455.7277693999936 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000000]",
            "value": 0.20827982093619954,
            "unit": "iter/sec",
            "range": "stddev: 0.041365951445586496",
            "extra": "mean: 4.801233242399997 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1]",
            "value": 9949.057511729283,
            "unit": "iter/sec",
            "range": "stddev: 0.000017844245018024472",
            "extra": "mean: 100.51203330778478 usec\nrounds: 3903"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10]",
            "value": 7807.028933827556,
            "unit": "iter/sec",
            "range": "stddev: 0.0000541931350307269",
            "extra": "mean: 128.0897007652986 usec\nrounds: 4702"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100]",
            "value": 4319.531408112005,
            "unit": "iter/sec",
            "range": "stddev: 0.000406050977784055",
            "extra": "mean: 231.5065930813739 usec\nrounds: 1966"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000]",
            "value": 707.0696624814873,
            "unit": "iter/sec",
            "range": "stddev: 0.002005445189906783",
            "extra": "mean: 1.414287803680422 msec\nrounds: 815"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10000]",
            "value": 74.45101356363477,
            "unit": "iter/sec",
            "range": "stddev: 0.0099955362734423",
            "extra": "mean: 13.431650586533387 msec\nrounds: 104"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100000]",
            "value": 5.782778474979976,
            "unit": "iter/sec",
            "range": "stddev: 0.030721041689397634",
            "extra": "mean: 172.92725362499084 msec\nrounds: 8"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000000]",
            "value": 0.4607442800678609,
            "unit": "iter/sec",
            "range": "stddev: 0.1351801061015948",
            "extra": "mean: 2.170401333799987 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1]",
            "value": 8491.344849264875,
            "unit": "iter/sec",
            "range": "stddev: 0.00002216290480050343",
            "extra": "mean: 117.76697540279187 usec\nrounds: 2114"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10]",
            "value": 4049.0966914400774,
            "unit": "iter/sec",
            "range": "stddev: 0.004885893438715876",
            "extra": "mean: 246.96866392794044 usec\nrounds: 3529"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100]",
            "value": 2320.3310617862053,
            "unit": "iter/sec",
            "range": "stddev: 0.0010997576090339925",
            "extra": "mean: 430.972983325144 usec\nrounds: 2039"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000]",
            "value": 270.6831452706421,
            "unit": "iter/sec",
            "range": "stddev: 0.005367297578499037",
            "extra": "mean: 3.694356362676928 msec\nrounds: 284"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10000]",
            "value": 28.063802313206196,
            "unit": "iter/sec",
            "range": "stddev: 0.019072326263519315",
            "extra": "mean: 35.63309022916764 msec\nrounds: 48"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100000]",
            "value": 1.9978622051285884,
            "unit": "iter/sec",
            "range": "stddev: 0.07093448957138496",
            "extra": "mean: 500.53502060000034 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000000]",
            "value": 0.17278230316388807,
            "unit": "iter/sec",
            "range": "stddev: 0.39153056047898094",
            "extra": "mean: 5.787629761200003 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1]",
            "value": 8766.703918517658,
            "unit": "iter/sec",
            "range": "stddev: 0.000015325830687060716",
            "extra": "mean: 114.06795636016959 usec\nrounds: 2154"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10]",
            "value": 4316.643224987576,
            "unit": "iter/sec",
            "range": "stddev: 0.00002666268469613366",
            "extra": "mean: 231.6614896990654 usec\nrounds: 1893"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100]",
            "value": 698.7934077159767,
            "unit": "iter/sec",
            "range": "stddev: 0.0001584167827540073",
            "extra": "mean: 1.4310381136372257 msec\nrounds: 484"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000]",
            "value": 73.40120310660203,
            "unit": "iter/sec",
            "range": "stddev: 0.002439192156266994",
            "extra": "mean: 13.623754893331656 msec\nrounds: 75"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10000]",
            "value": 7.072550313054451,
            "unit": "iter/sec",
            "range": "stddev: 0.004981821690694093",
            "extra": "mean: 141.39171242857174 msec\nrounds: 7"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100000]",
            "value": 0.634287379798243,
            "unit": "iter/sec",
            "range": "stddev: 0.028693290907966847",
            "extra": "mean: 1.5765724367999951 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000000]",
            "value": 0.061754353338713645,
            "unit": "iter/sec",
            "range": "stddev: 0.31852297863841744",
            "extra": "mean: 16.1931903734 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1]",
            "value": 6572.190433405118,
            "unit": "iter/sec",
            "range": "stddev: 0.000024506361120193677",
            "extra": "mean: 152.1562727271568 usec\nrounds: 1496"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10]",
            "value": 2289.316237237636,
            "unit": "iter/sec",
            "range": "stddev: 0.000025850669014161952",
            "extra": "mean: 436.81164870722836 usec\nrounds: 928"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100]",
            "value": 306.38353995116125,
            "unit": "iter/sec",
            "range": "stddev: 0.00004771350838011665",
            "extra": "mean: 3.263882910157001 msec\nrounds: 256"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000]",
            "value": 31.611614531792064,
            "unit": "iter/sec",
            "range": "stddev: 0.0005032357268038511",
            "extra": "mean: 31.63394261290551 msec\nrounds: 31"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10000]",
            "value": 2.757194931606005,
            "unit": "iter/sec",
            "range": "stddev: 0.010534053936774012",
            "extra": "mean: 362.6874504000057 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100000]",
            "value": 0.2505996560828647,
            "unit": "iter/sec",
            "range": "stddev: 0.027579445246238014",
            "extra": "mean: 3.9904284612000196 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000000]",
            "value": 0.025075140833393197,
            "unit": "iter/sec",
            "range": "stddev: 0.5646847922090575",
            "extra": "mean: 39.88013493700002 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1]",
            "value": 11587.714019155297,
            "unit": "iter/sec",
            "range": "stddev: 0.000009652847033027509",
            "extra": "mean: 86.29829821023633 usec\nrounds: 4527"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10]",
            "value": 9105.436400877112,
            "unit": "iter/sec",
            "range": "stddev: 0.000016466277301983264",
            "extra": "mean: 109.8244999991073 usec\nrounds: 4652"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100]",
            "value": 9576.990884758821,
            "unit": "iter/sec",
            "range": "stddev: 0.000016687107788372725",
            "extra": "mean: 104.41693137574528 usec\nrounds: 2929"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000]",
            "value": 10769.753576985293,
            "unit": "iter/sec",
            "range": "stddev: 0.00001543397739041442",
            "extra": "mean: 92.85263519278436 usec\nrounds: 466"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10000]",
            "value": 8119.379651988254,
            "unit": "iter/sec",
            "range": "stddev: 0.000030283665839561548",
            "extra": "mean: 123.16211864229338 usec\nrounds: 59"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100000]",
            "value": 2674.4103325764054,
            "unit": "iter/sec",
            "range": "stddev: 0.0005917745051808047",
            "extra": "mean: 373.9142000085849 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000000]",
            "value": 799.7916702692221,
            "unit": "iter/sec",
            "range": "stddev: 0.002519059453444254",
            "extra": "mean: 1.2503255999945395 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1]",
            "value": 9210.381693638057,
            "unit": "iter/sec",
            "range": "stddev: 0.00001787292376225442",
            "extra": "mean: 108.57313336870024 usec\nrounds: 5751"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10]",
            "value": 7625.037530444049,
            "unit": "iter/sec",
            "range": "stddev: 0.0020492228151849584",
            "extra": "mean: 131.14689547524947 usec\nrounds: 5348"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100]",
            "value": 10120.299259714604,
            "unit": "iter/sec",
            "range": "stddev: 0.00001591905261966357",
            "extra": "mean: 98.81130728817998 usec\nrounds: 3238"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000]",
            "value": 10801.457193511225,
            "unit": "iter/sec",
            "range": "stddev: 0.00000854645225372549",
            "extra": "mean: 92.58010119234018 usec\nrounds: 504"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10000]",
            "value": 317.64306778997616,
            "unit": "iter/sec",
            "range": "stddev: 0.0242850923258398",
            "extra": "mean: 3.148187703127192 msec\nrounds: 64"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100000]",
            "value": 2190.4384734253163,
            "unit": "iter/sec",
            "range": "stddev: 0.0007514695210224339",
            "extra": "mean: 456.5295999555019 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000000]",
            "value": 294.0086217439287,
            "unit": "iter/sec",
            "range": "stddev: 0.0073153025418155335",
            "extra": "mean: 3.4012608000011824 msec\nrounds: 5"
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
          "id": "5ad22a8e748a60edf9cda1bf7373e7e29f7bd1c4",
          "message": "Merge pull request #78 from FalkorDB/tck_return\n\nTCK return",
          "timestamp": "2025-06-09T18:18:41+03:00",
          "tree_id": "8d0578972db574ff27692b6210f08330d6dba2f6",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/5ad22a8e748a60edf9cda1bf7373e7e29f7bd1c4"
        },
        "date": 1749483054676,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 8695.997848166884,
            "unit": "iter/sec",
            "range": "stddev: 0.000017455783157325833",
            "extra": "mean: 114.99542863971614 usec\nrounds: 2102"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1]",
            "value": 9815.555884108147,
            "unit": "iter/sec",
            "range": "stddev: 0.000013844295110888012",
            "extra": "mean: 101.87910005372672 usec\nrounds: 5557"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10]",
            "value": 6902.288191367734,
            "unit": "iter/sec",
            "range": "stddev: 0.00001819765020248118",
            "extra": "mean: 144.8794910143912 usec\nrounds: 4841"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100]",
            "value": 1969.1585103477532,
            "unit": "iter/sec",
            "range": "stddev: 0.000021950165709755245",
            "extra": "mean: 507.83113433737753 usec\nrounds: 1727"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000]",
            "value": 234.12370521749892,
            "unit": "iter/sec",
            "range": "stddev: 0.001302795985423447",
            "extra": "mean: 4.271246258771654 msec\nrounds: 228"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10000]",
            "value": 23.542654302793025,
            "unit": "iter/sec",
            "range": "stddev: 0.004347972076337237",
            "extra": "mean: 42.47609411999747 msec\nrounds: 25"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100000]",
            "value": 2.1731125614040896,
            "unit": "iter/sec",
            "range": "stddev: 0.008742653643430682",
            "extra": "mean: 460.16944440000884 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000000]",
            "value": 0.2063915771247335,
            "unit": "iter/sec",
            "range": "stddev: 0.044847400353299184",
            "extra": "mean: 4.845158964000001 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1]",
            "value": 10741.723469802235,
            "unit": "iter/sec",
            "range": "stddev: 0.000010718661838365579",
            "extra": "mean: 93.09493051196662 usec\nrounds: 4749"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10]",
            "value": 9249.072214540982,
            "unit": "iter/sec",
            "range": "stddev: 0.00005468653253399736",
            "extra": "mean: 108.11895256130062 usec\nrounds: 5017"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100]",
            "value": 4103.988646230844,
            "unit": "iter/sec",
            "range": "stddev: 0.0005421709342363299",
            "extra": "mean: 243.66539145238931 usec\nrounds: 2644"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000]",
            "value": 649.8523763989414,
            "unit": "iter/sec",
            "range": "stddev: 0.0023249187015144655",
            "extra": "mean: 1.5388110228069776 msec\nrounds: 570"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10000]",
            "value": 66.83986770184342,
            "unit": "iter/sec",
            "range": "stddev: 0.010242480924457198",
            "extra": "mean: 14.961130750000276 msec\nrounds: 108"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100000]",
            "value": 4.927791803082924,
            "unit": "iter/sec",
            "range": "stddev: 0.02974073484300683",
            "extra": "mean: 202.93065128570984 msec\nrounds: 7"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000000]",
            "value": 0.4033095922630581,
            "unit": "iter/sec",
            "range": "stddev: 0.09862614875482228",
            "extra": "mean: 2.479484790799995 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1]",
            "value": 7694.55321764488,
            "unit": "iter/sec",
            "range": "stddev: 0.000018290021710196072",
            "extra": "mean: 129.9620616966863 usec\nrounds: 2334"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10]",
            "value": 4410.1435728369925,
            "unit": "iter/sec",
            "range": "stddev: 0.00432537196940667",
            "extra": "mean: 226.749987496827 usec\nrounds: 4559"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100]",
            "value": 2201.1929183093544,
            "unit": "iter/sec",
            "range": "stddev: 0.0012223840257247146",
            "extra": "mean: 454.2991173931538 usec\nrounds: 1857"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000]",
            "value": 267.4674307130859,
            "unit": "iter/sec",
            "range": "stddev: 0.0053112926054879085",
            "extra": "mean: 3.7387729688580538 msec\nrounds: 289"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10000]",
            "value": 25.20735484454724,
            "unit": "iter/sec",
            "range": "stddev: 0.016142686131652123",
            "extra": "mean: 39.670961358974814 msec\nrounds: 39"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100000]",
            "value": 1.8338074870594456,
            "unit": "iter/sec",
            "range": "stddev: 0.06413385495186086",
            "extra": "mean: 545.313511400002 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000000]",
            "value": 0.1518581245020802,
            "unit": "iter/sec",
            "range": "stddev: 0.5094195556715975",
            "extra": "mean: 6.5850938386 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1]",
            "value": 8604.205920513117,
            "unit": "iter/sec",
            "range": "stddev: 0.000018167449364312776",
            "extra": "mean: 116.2222300626162 usec\nrounds: 1743"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10]",
            "value": 4039.7983596512213,
            "unit": "iter/sec",
            "range": "stddev: 0.000027616197997546247",
            "extra": "mean: 247.53710729422042 usec\nrounds: 1892"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100]",
            "value": 682.3194423152706,
            "unit": "iter/sec",
            "range": "stddev: 0.0003252635290344823",
            "extra": "mean: 1.4655891917820258 msec\nrounds: 511"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000]",
            "value": 73.03301578387216,
            "unit": "iter/sec",
            "range": "stddev: 0.0023271428732899338",
            "extra": "mean: 13.692437444447275 msec\nrounds: 45"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10000]",
            "value": 6.960705176551799,
            "unit": "iter/sec",
            "range": "stddev: 0.003222102562505475",
            "extra": "mean: 143.66360514286012 msec\nrounds: 7"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100000]",
            "value": 0.6233415675794517,
            "unit": "iter/sec",
            "range": "stddev: 0.020861634933879568",
            "extra": "mean: 1.6042568825999866 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000000]",
            "value": 0.061115751838281246,
            "unit": "iter/sec",
            "range": "stddev: 0.22377448306669914",
            "extra": "mean: 16.362393817000008 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1]",
            "value": 7260.07953984994,
            "unit": "iter/sec",
            "range": "stddev: 0.000025745764107545603",
            "extra": "mean: 137.73953777105163 usec\nrounds: 1562"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10]",
            "value": 2292.4504691462525,
            "unit": "iter/sec",
            "range": "stddev: 0.000026280310209903066",
            "extra": "mean: 436.2144410354117 usec\nrounds: 1043"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100]",
            "value": 291.28547699523824,
            "unit": "iter/sec",
            "range": "stddev: 0.00029078678434959516",
            "extra": "mean: 3.433058216000063 msec\nrounds: 250"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000]",
            "value": 31.175983563768103,
            "unit": "iter/sec",
            "range": "stddev: 0.001628677292296528",
            "extra": "mean: 32.07597277418934 msec\nrounds: 31"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10000]",
            "value": 2.7385222975418917,
            "unit": "iter/sec",
            "range": "stddev: 0.007568550372774045",
            "extra": "mean: 365.1604373999817 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100000]",
            "value": 0.24866604019074054,
            "unit": "iter/sec",
            "range": "stddev: 0.017767333541259277",
            "extra": "mean: 4.021457852599997 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000000]",
            "value": 0.024836868312523613,
            "unit": "iter/sec",
            "range": "stddev: 0.6739435181386949",
            "extra": "mean: 40.26272505120001 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1]",
            "value": 10183.343768222236,
            "unit": "iter/sec",
            "range": "stddev: 0.000017048168110771906",
            "extra": "mean: 98.19957204239367 usec\nrounds: 4886"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10]",
            "value": 10148.359082742789,
            "unit": "iter/sec",
            "range": "stddev: 0.000016898273811415886",
            "extra": "mean: 98.53809781923195 usec\nrounds: 6369"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100]",
            "value": 10138.36956811793,
            "unit": "iter/sec",
            "range": "stddev: 0.00001599906248996409",
            "extra": "mean: 98.63518914764106 usec\nrounds: 2617"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000]",
            "value": 11550.54018295964,
            "unit": "iter/sec",
            "range": "stddev: 0.000010313439008328401",
            "extra": "mean: 86.57603749782083 usec\nrounds: 480"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10000]",
            "value": 10289.977066737492,
            "unit": "iter/sec",
            "range": "stddev: 0.00005187613278175836",
            "extra": "mean: 97.18194642362374 usec\nrounds: 56"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100000]",
            "value": 2107.960459826916,
            "unit": "iter/sec",
            "range": "stddev: 0.0008059943115017763",
            "extra": "mean: 474.3921999761369 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000000]",
            "value": 744.3251903054743,
            "unit": "iter/sec",
            "range": "stddev: 0.002721281370363449",
            "extra": "mean: 1.3434988000199155 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1]",
            "value": 9028.200005271172,
            "unit": "iter/sec",
            "range": "stddev: 0.00001658999763682522",
            "extra": "mean: 110.76405035512545 usec\nrounds: 5779"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10]",
            "value": 7464.908719037795,
            "unit": "iter/sec",
            "range": "stddev: 0.0021202093085402316",
            "extra": "mean: 133.96011091866333 usec\nrounds: 5184"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100]",
            "value": 9855.735869759024,
            "unit": "iter/sec",
            "range": "stddev: 0.000016990266697057645",
            "extra": "mean: 101.46375808105441 usec\nrounds: 2939"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000]",
            "value": 10662.017532814349,
            "unit": "iter/sec",
            "range": "stddev: 0.000012509273266132328",
            "extra": "mean: 93.79087934551913 usec\nrounds: 489"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10000]",
            "value": 289.38371458840976,
            "unit": "iter/sec",
            "range": "stddev: 0.02600117126260864",
            "extra": "mean: 3.455619475416228 msec\nrounds: 61"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100000]",
            "value": 2155.094384443589,
            "unit": "iter/sec",
            "range": "stddev: 0.0007623072809141819",
            "extra": "mean: 464.01680001508794 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000000]",
            "value": 215.0064527739104,
            "unit": "iter/sec",
            "range": "stddev: 0.010056110817401735",
            "extra": "mean: 4.651023199994597 msec\nrounds: 5"
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
          "id": "380baf5059c33539920d98740bac19aba190961a",
          "message": "Merge pull request #90 from FalkorDB/tck_return\n\nTCK return",
          "timestamp": "2025-06-10T15:09:47+03:00",
          "tree_id": "b81218ef6390c2f59f317e244572298a3d2c6c99",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/380baf5059c33539920d98740bac19aba190961a"
        },
        "date": 1749558128396,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 9669.2189976552,
            "unit": "iter/sec",
            "range": "stddev: 0.00001767532957391889",
            "extra": "mean: 103.42096918505017 usec\nrounds: 2012"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1]",
            "value": 9936.70595709536,
            "unit": "iter/sec",
            "range": "stddev: 0.00005555244463863973",
            "extra": "mean: 100.63697208288069 usec\nrounds: 5982"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10]",
            "value": 6903.214696462419,
            "unit": "iter/sec",
            "range": "stddev: 0.00003065422808966308",
            "extra": "mean: 144.86004622056072 usec\nrounds: 4154"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100]",
            "value": 1994.6525707127892,
            "unit": "iter/sec",
            "range": "stddev: 0.00016571449583335923",
            "extra": "mean: 501.34044127928 usec\nrounds: 1720"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000]",
            "value": 217.5348271159962,
            "unit": "iter/sec",
            "range": "stddev: 0.0015147975611041924",
            "extra": "mean: 4.596965061906016 msec\nrounds: 210"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10000]",
            "value": 22.854928693417385,
            "unit": "iter/sec",
            "range": "stddev: 0.005007000329452589",
            "extra": "mean: 43.754238458333816 msec\nrounds: 24"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100000]",
            "value": 2.107653555615283,
            "unit": "iter/sec",
            "range": "stddev: 0.010165524937667467",
            "extra": "mean: 474.461278199999 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000000]",
            "value": 0.20320316280397363,
            "unit": "iter/sec",
            "range": "stddev: 0.10381597540530826",
            "extra": "mean: 4.921183244399998 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1]",
            "value": 10493.269831950021,
            "unit": "iter/sec",
            "range": "stddev: 0.00010185402442976777",
            "extra": "mean: 95.29917899901794 usec\nrounds: 3257"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10]",
            "value": 8625.389231309295,
            "unit": "iter/sec",
            "range": "stddev: 0.00005977970213954475",
            "extra": "mean: 115.93679695869267 usec\nrounds: 4669"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100]",
            "value": 4004.0050433882107,
            "unit": "iter/sec",
            "range": "stddev: 0.0005338524651190666",
            "extra": "mean: 249.749935168362 usec\nrounds: 2761"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000]",
            "value": 669.2049055388262,
            "unit": "iter/sec",
            "range": "stddev: 0.002087734359113979",
            "extra": "mean: 1.494310624030507 msec\nrounds: 516"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10000]",
            "value": 67.78892017947646,
            "unit": "iter/sec",
            "range": "stddev: 0.009857984476575472",
            "extra": "mean: 14.751673243244202 msec\nrounds: 111"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100000]",
            "value": 4.91458368774161,
            "unit": "iter/sec",
            "range": "stddev: 0.023132061589002814",
            "extra": "mean: 203.47603449999005 msec\nrounds: 6"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000000]",
            "value": 0.4039050046384377,
            "unit": "iter/sec",
            "range": "stddev: 0.13052453627600427",
            "extra": "mean: 2.475829683999996 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1]",
            "value": 7569.7921680140535,
            "unit": "iter/sec",
            "range": "stddev: 0.0003064811728817345",
            "extra": "mean: 132.1040231758901 usec\nrounds: 2632"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10]",
            "value": 4092.527804547385,
            "unit": "iter/sec",
            "range": "stddev: 0.004873623990547035",
            "extra": "mean: 244.34775956533676 usec\nrounds: 3685"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100]",
            "value": 2115.5066320360106,
            "unit": "iter/sec",
            "range": "stddev: 0.0012069165726857588",
            "extra": "mean: 472.70000710779044 usec\nrounds: 1829"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000]",
            "value": 275.37874154554095,
            "unit": "iter/sec",
            "range": "stddev: 0.004164933067322245",
            "extra": "mean: 3.6313623716470653 msec\nrounds: 261"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10000]",
            "value": 23.847669785043614,
            "unit": "iter/sec",
            "range": "stddev: 0.019961185342494907",
            "extra": "mean: 41.9328181333324 msec\nrounds: 45"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100000]",
            "value": 1.7168627281839213,
            "unit": "iter/sec",
            "range": "stddev: 0.061653751902631754",
            "extra": "mean: 582.4577489999967 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000000]",
            "value": 0.1472905761623081,
            "unit": "iter/sec",
            "range": "stddev: 0.3426645520430377",
            "extra": "mean: 6.789300619600004 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1]",
            "value": 6989.302152540001,
            "unit": "iter/sec",
            "range": "stddev: 0.000012699528688203818",
            "extra": "mean: 143.07580044119103 usec\nrounds: 1358"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10]",
            "value": 4275.126613076978,
            "unit": "iter/sec",
            "range": "stddev: 0.00002215400998637414",
            "extra": "mean: 233.9112008849395 usec\nrounds: 1807"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100]",
            "value": 710.894879640821,
            "unit": "iter/sec",
            "range": "stddev: 0.00017514499205239503",
            "extra": "mean: 1.4066777362431546 msec\nrounds: 527"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000]",
            "value": 75.24341180003726,
            "unit": "iter/sec",
            "range": "stddev: 0.0015249635863166075",
            "extra": "mean: 13.290200112902172 msec\nrounds: 62"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10000]",
            "value": 7.090102061009049,
            "unit": "iter/sec",
            "range": "stddev: 0.007464988323738238",
            "extra": "mean: 141.0416932499956 msec\nrounds: 8"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100000]",
            "value": 0.6144210051969455,
            "unit": "iter/sec",
            "range": "stddev: 0.036614381368225424",
            "extra": "mean: 1.6275485237999987 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000000]",
            "value": 0.05987798644786026,
            "unit": "iter/sec",
            "range": "stddev: 0.2771141105011938",
            "extra": "mean: 16.7006283832 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1]",
            "value": 6349.32867129294,
            "unit": "iter/sec",
            "range": "stddev: 0.00002192078926321827",
            "extra": "mean: 157.4969657061974 usec\nrounds: 1458"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10]",
            "value": 2343.4351381583883,
            "unit": "iter/sec",
            "range": "stddev: 0.000032022152725764204",
            "extra": "mean: 426.72399321700874 usec\nrounds: 1032"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100]",
            "value": 308.5738923970576,
            "unit": "iter/sec",
            "range": "stddev: 0.00008509878078489636",
            "extra": "mean: 3.240714864863712 msec\nrounds: 259"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000]",
            "value": 31.46359386391641,
            "unit": "iter/sec",
            "range": "stddev: 0.0004362581308545206",
            "extra": "mean: 31.782764687502407 msec\nrounds: 32"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10000]",
            "value": 2.9355982623115193,
            "unit": "iter/sec",
            "range": "stddev: 0.02080338451364384",
            "extra": "mean: 340.6460662000086 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100000]",
            "value": 0.2450305581122759,
            "unit": "iter/sec",
            "range": "stddev: 0.08119075810778127",
            "extra": "mean: 4.081123626800002 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000000]",
            "value": 0.024558029208139896,
            "unit": "iter/sec",
            "range": "stddev: 0.7213879720759185",
            "extra": "mean: 40.71987990260002 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1]",
            "value": 9536.643929910773,
            "unit": "iter/sec",
            "range": "stddev: 0.0000178872284220366",
            "extra": "mean: 104.85869110239038 usec\nrounds: 4911"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10]",
            "value": 10791.750566741723,
            "unit": "iter/sec",
            "range": "stddev: 0.0000163410822616325",
            "extra": "mean: 92.66337225045064 usec\nrounds: 6501"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100]",
            "value": 10608.269882841842,
            "unit": "iter/sec",
            "range": "stddev: 0.000015926071907846705",
            "extra": "mean: 94.26607835622963 usec\nrounds: 2361"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000]",
            "value": 11571.25754841105,
            "unit": "iter/sec",
            "range": "stddev: 0.000012104941601276712",
            "extra": "mean: 86.42103036910787 usec\nrounds: 461"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10000]",
            "value": 10179.104112775893,
            "unit": "iter/sec",
            "range": "stddev: 0.00006190639687651178",
            "extra": "mean: 98.24047272931321 usec\nrounds: 55"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100000]",
            "value": 2463.2507618852046,
            "unit": "iter/sec",
            "range": "stddev: 0.0006641423290180328",
            "extra": "mean: 405.96759999971255 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000000]",
            "value": 305.15087971689934,
            "unit": "iter/sec",
            "range": "stddev: 0.006852299816805905",
            "extra": "mean: 3.2770673999948485 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1]",
            "value": 10381.773802623755,
            "unit": "iter/sec",
            "range": "stddev: 0.00001633235957660284",
            "extra": "mean: 96.32265343204385 usec\nrounds: 6045"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10]",
            "value": 8190.066991864066,
            "unit": "iter/sec",
            "range": "stddev: 0.0020198069966196695",
            "extra": "mean: 122.09912336411783 usec\nrounds: 5731"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100]",
            "value": 9732.292301326977,
            "unit": "iter/sec",
            "range": "stddev: 0.0000181223436713478",
            "extra": "mean: 102.75071576545766 usec\nrounds: 3184"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000]",
            "value": 11035.018863097152,
            "unit": "iter/sec",
            "range": "stddev: 0.000011281361787364723",
            "extra": "mean: 90.62059724647668 usec\nrounds: 509"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10000]",
            "value": 302.5833311251648,
            "unit": "iter/sec",
            "range": "stddev: 0.025265264147924276",
            "extra": "mean: 3.3048747142860493 msec\nrounds: 63"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100000]",
            "value": 2165.5305569659804,
            "unit": "iter/sec",
            "range": "stddev: 0.0007497991635001268",
            "extra": "mean: 461.780600039674 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000000]",
            "value": 311.7680854493455,
            "unit": "iter/sec",
            "range": "stddev: 0.0067423283501608635",
            "extra": "mean: 3.2075123999902644 msec\nrounds: 5"
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
            "email": "barak.bar@gmail.com",
            "name": "Barak Bar Orion",
            "username": "barakb"
          },
          "distinct": true,
          "id": "7af5517adcb9b27fbb782b510e5f6037be7513be",
          "message": "fix test",
          "timestamp": "2025-06-11T12:39:01+03:00",
          "tree_id": "01cd2f0728bbce385308cd78a98046bdaa196bb8",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/7af5517adcb9b27fbb782b510e5f6037be7513be"
        },
        "date": 1749635452767,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 8542.19148774734,
            "unit": "iter/sec",
            "range": "stddev: 0.000014392806215458717",
            "extra": "mean: 117.06597790910796 usec\nrounds: 2535"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1]",
            "value": 9213.022158135973,
            "unit": "iter/sec",
            "range": "stddev: 0.000016162637518478274",
            "extra": "mean: 108.54201616316587 usec\nrounds: 6187"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10]",
            "value": 7395.12331110917,
            "unit": "iter/sec",
            "range": "stddev: 0.000012777605659869205",
            "extra": "mean: 135.2242495399327 usec\nrounds: 4889"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100]",
            "value": 2017.4638644846882,
            "unit": "iter/sec",
            "range": "stddev: 0.000021316614953741016",
            "extra": "mean: 495.67182719053335 usec\nrounds: 1655"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000]",
            "value": 234.77466844580283,
            "unit": "iter/sec",
            "range": "stddev: 0.001067997858842245",
            "extra": "mean: 4.259403310501735 msec\nrounds: 219"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10000]",
            "value": 23.903573986486393,
            "unit": "iter/sec",
            "range": "stddev: 0.0035432340445753587",
            "extra": "mean: 41.834748250003884 msec\nrounds: 24"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100000]",
            "value": 2.2164543332599265,
            "unit": "iter/sec",
            "range": "stddev: 0.008872215230853384",
            "extra": "mean: 451.1710369999889 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000000]",
            "value": 0.21035827819568978,
            "unit": "iter/sec",
            "range": "stddev: 0.052768426809492945",
            "extra": "mean: 4.753794376799999 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1]",
            "value": 9153.09735873074,
            "unit": "iter/sec",
            "range": "stddev: 0.000019559821889304048",
            "extra": "mean: 109.25263446981079 usec\nrounds: 4224"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10]",
            "value": 8737.821468206736,
            "unit": "iter/sec",
            "range": "stddev: 0.00005980853943657735",
            "extra": "mean: 114.44500252592482 usec\nrounds: 4751"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100]",
            "value": 4109.0649298276185,
            "unit": "iter/sec",
            "range": "stddev: 0.0005117631834067523",
            "extra": "mean: 243.36437050215983 usec\nrounds: 2807"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000]",
            "value": 676.747228177036,
            "unit": "iter/sec",
            "range": "stddev: 0.002355944839851324",
            "extra": "mean: 1.4776565878130226 msec\nrounds: 558"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10000]",
            "value": 73.89446122432634,
            "unit": "iter/sec",
            "range": "stddev: 0.009320094988404503",
            "extra": "mean: 13.532814008403598 msec\nrounds: 119"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100000]",
            "value": 5.816720653723243,
            "unit": "iter/sec",
            "range": "stddev: 0.02645428938323521",
            "extra": "mean: 171.91817512500052 msec\nrounds: 8"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000000]",
            "value": 0.4339882291688361,
            "unit": "iter/sec",
            "range": "stddev: 0.12737809566950561",
            "extra": "mean: 2.3042099596000014 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1]",
            "value": 9990.88232776066,
            "unit": "iter/sec",
            "range": "stddev: 0.000015459909537363328",
            "extra": "mean: 100.0912599302066 usec\nrounds: 2870"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10]",
            "value": 4371.756918594693,
            "unit": "iter/sec",
            "range": "stddev: 0.00443019821364993",
            "extra": "mean: 228.7409887193479 usec\nrounds: 4255"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100]",
            "value": 2206.1413339452997,
            "unit": "iter/sec",
            "range": "stddev: 0.00123365894651588",
            "extra": "mean: 453.2801161073729 usec\nrounds: 1757"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000]",
            "value": 271.3435581006738,
            "unit": "iter/sec",
            "range": "stddev: 0.005458192238113823",
            "extra": "mean: 3.685364808362173 msec\nrounds: 287"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10000]",
            "value": 27.623161539013275,
            "unit": "iter/sec",
            "range": "stddev: 0.018593286357653147",
            "extra": "mean: 36.20150425532069 msec\nrounds: 47"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100000]",
            "value": 1.9273365518286365,
            "unit": "iter/sec",
            "range": "stddev: 0.05210345733085885",
            "extra": "mean: 518.8507419999951 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000000]",
            "value": 0.16439332223836753,
            "unit": "iter/sec",
            "range": "stddev: 0.40440695048178726",
            "extra": "mean: 6.082972145000008 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1]",
            "value": 8840.540852722135,
            "unit": "iter/sec",
            "range": "stddev: 0.000019726748737275275",
            "extra": "mean: 113.11525127923423 usec\nrounds: 1564"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10]",
            "value": 4125.289926477478,
            "unit": "iter/sec",
            "range": "stddev: 0.00002158290538777026",
            "extra": "mean: 242.407204783758 usec\nrounds: 1714"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100]",
            "value": 731.767134101262,
            "unit": "iter/sec",
            "range": "stddev: 0.00006294065661022911",
            "extra": "mean: 1.3665549508836237 msec\nrounds: 509"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000]",
            "value": 78.20440826416217,
            "unit": "iter/sec",
            "range": "stddev: 0.001166326360181876",
            "extra": "mean: 12.78700296052567 msec\nrounds: 76"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10000]",
            "value": 7.21235412504447,
            "unit": "iter/sec",
            "range": "stddev: 0.004241448255001273",
            "extra": "mean: 138.6509844999928 msec\nrounds: 8"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100000]",
            "value": 0.6442471754868626,
            "unit": "iter/sec",
            "range": "stddev: 0.028880305527724585",
            "extra": "mean: 1.5521992769999997 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000000]",
            "value": 0.06258175216763533,
            "unit": "iter/sec",
            "range": "stddev: 0.2444591174722469",
            "extra": "mean: 15.979098784600001 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1]",
            "value": 6291.93672797774,
            "unit": "iter/sec",
            "range": "stddev: 0.000021149950073756934",
            "extra": "mean: 158.93357534149982 usec\nrounds: 1314"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10]",
            "value": 2233.396761647276,
            "unit": "iter/sec",
            "range": "stddev: 0.00001839524961242777",
            "extra": "mean: 447.74847764283254 usec\nrounds: 984"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100]",
            "value": 296.46960886634906,
            "unit": "iter/sec",
            "range": "stddev: 0.0004533239406200403",
            "extra": "mean: 3.3730270155643782 msec\nrounds: 257"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000]",
            "value": 31.332468801983538,
            "unit": "iter/sec",
            "range": "stddev: 0.002666580532679208",
            "extra": "mean: 31.915774218745696 msec\nrounds: 32"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10000]",
            "value": 2.860168871680756,
            "unit": "iter/sec",
            "range": "stddev: 0.009980601961111656",
            "extra": "mean: 349.6297054000024 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100000]",
            "value": 0.25305833495073843,
            "unit": "iter/sec",
            "range": "stddev: 0.015483055459812153",
            "extra": "mean: 3.951658024599999 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000000]",
            "value": 0.025193121903322367,
            "unit": "iter/sec",
            "range": "stddev: 0.9605955020407408",
            "extra": "mean: 39.69337360560003 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1]",
            "value": 10037.338186652278,
            "unit": "iter/sec",
            "range": "stddev: 0.000018434348570835004",
            "extra": "mean: 99.62800708755704 usec\nrounds: 4797"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10]",
            "value": 9126.284458575823,
            "unit": "iter/sec",
            "range": "stddev: 0.000016136558612827676",
            "extra": "mean: 109.573617230429 usec\nrounds: 4875"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100]",
            "value": 11390.129106494911,
            "unit": "iter/sec",
            "range": "stddev: 0.000008665309208196593",
            "extra": "mean: 87.79531738843743 usec\nrounds: 3157"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000]",
            "value": 11556.388107038385,
            "unit": "iter/sec",
            "range": "stddev: 0.000007676942517583627",
            "extra": "mean: 86.53222708840602 usec\nrounds: 502"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10000]",
            "value": 9089.5744978158,
            "unit": "iter/sec",
            "range": "stddev: 0.000050795469228166966",
            "extra": "mean: 110.01615094747254 usec\nrounds: 53"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100000]",
            "value": 2744.2627073222125,
            "unit": "iter/sec",
            "range": "stddev: 0.000555438315735679",
            "extra": "mean: 364.3965999799548 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000000]",
            "value": 731.1519269557521,
            "unit": "iter/sec",
            "range": "stddev: 0.0028118335179305744",
            "extra": "mean: 1.367704799963576 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1]",
            "value": 9794.231357130693,
            "unit": "iter/sec",
            "range": "stddev: 0.000016745843424016544",
            "extra": "mean: 102.10091670664383 usec\nrounds: 6207"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10]",
            "value": 8209.038803271951,
            "unit": "iter/sec",
            "range": "stddev: 0.0018694409164737998",
            "extra": "mean: 121.81694154027156 usec\nrounds: 6295"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100]",
            "value": 10373.100973502163,
            "unit": "iter/sec",
            "range": "stddev: 0.000014564536128499227",
            "extra": "mean: 96.40318768268774 usec\nrounds: 3069"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000]",
            "value": 8572.658294868796,
            "unit": "iter/sec",
            "range": "stddev: 0.000014888014128753615",
            "extra": "mean: 116.6499311652903 usec\nrounds: 523"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10000]",
            "value": 327.37927163904726,
            "unit": "iter/sec",
            "range": "stddev: 0.023881118240078287",
            "extra": "mean: 3.054561136364651 msec\nrounds: 66"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100000]",
            "value": 2238.977736576215,
            "unit": "iter/sec",
            "range": "stddev: 0.0007390819020498059",
            "extra": "mean: 446.63239998499193 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000000]",
            "value": 289.89464938359555,
            "unit": "iter/sec",
            "range": "stddev: 0.007408968188400343",
            "extra": "mean: 3.4495290000222667 msec\nrounds: 5"
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
          "id": "f514845543c5dc2bdc7f988b4c70c8f7f94ebab2",
          "message": "Merge pull request #91 from FalkorDB/match-tck-2\n\nadd fuzzer",
          "timestamp": "2025-06-11T16:14:35+03:00",
          "tree_id": "cc1b029d1c9adaa2af779905afb171b33dba7c1b",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/f514845543c5dc2bdc7f988b4c70c8f7f94ebab2"
        },
        "date": 1749648385950,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 9636.937099712286,
            "unit": "iter/sec",
            "range": "stddev: 0.000016348222865016887",
            "extra": "mean: 103.76740967105154 usec\nrounds: 2585"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1]",
            "value": 9729.875847164312,
            "unit": "iter/sec",
            "range": "stddev: 0.000015457870125209408",
            "extra": "mean: 102.7762343228091 usec\nrounds: 5996"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10]",
            "value": 7312.356637791704,
            "unit": "iter/sec",
            "range": "stddev: 0.000015377513451506486",
            "extra": "mean: 136.7548178424179 usec\nrounds: 2948"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100]",
            "value": 2066.790734778024,
            "unit": "iter/sec",
            "range": "stddev: 0.000017124713241242943",
            "extra": "mean: 483.84192128062796 usec\nrounds: 1156"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000]",
            "value": 236.9342849589091,
            "unit": "iter/sec",
            "range": "stddev: 0.0010797365671147492",
            "extra": "mean: 4.220579559321385 msec\nrounds: 236"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10000]",
            "value": 24.351295031920067,
            "unit": "iter/sec",
            "range": "stddev: 0.003226954418691815",
            "extra": "mean: 41.065577772729704 msec\nrounds: 22"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100000]",
            "value": 2.2318186994256406,
            "unit": "iter/sec",
            "range": "stddev: 0.007869696763650017",
            "extra": "mean: 448.06506919999833 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000000]",
            "value": 0.21278739163848073,
            "unit": "iter/sec",
            "range": "stddev: 0.08581160370531989",
            "extra": "mean: 4.699526566399993 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1]",
            "value": 9279.152333170672,
            "unit": "iter/sec",
            "range": "stddev: 0.000020294425282643",
            "extra": "mean: 107.76846462852512 usec\nrounds: 4481"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10]",
            "value": 8705.036327132128,
            "unit": "iter/sec",
            "range": "stddev: 0.000054080936010501377",
            "extra": "mean: 114.87602836109586 usec\nrounds: 5430"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100]",
            "value": 4131.124527840284,
            "unit": "iter/sec",
            "range": "stddev: 0.0005073091280132009",
            "extra": "mean: 242.0648405200197 usec\nrounds: 2922"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000]",
            "value": 656.1065296999192,
            "unit": "iter/sec",
            "range": "stddev: 0.0023058753118953067",
            "extra": "mean: 1.5241427340425433 msec\nrounds: 564"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10000]",
            "value": 77.21369566457513,
            "unit": "iter/sec",
            "range": "stddev: 0.007859784753760476",
            "extra": "mean: 12.951070291261685 msec\nrounds: 103"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100000]",
            "value": 5.8098772261537235,
            "unit": "iter/sec",
            "range": "stddev: 0.028582118682185825",
            "extra": "mean: 172.12067675000142 msec\nrounds: 8"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000000]",
            "value": 0.42950839846746214,
            "unit": "iter/sec",
            "range": "stddev: 0.14356472374456905",
            "extra": "mean: 2.328243181199997 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1]",
            "value": 8590.687263113648,
            "unit": "iter/sec",
            "range": "stddev: 0.00016375077794152811",
            "extra": "mean: 116.4051221249504 usec\nrounds: 3087"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10]",
            "value": 3893.7317799009384,
            "unit": "iter/sec",
            "range": "stddev: 0.004978423751887615",
            "extra": "mean: 256.8230316124757 usec\nrounds: 3448"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100]",
            "value": 2290.90731454858,
            "unit": "iter/sec",
            "range": "stddev: 0.0011323243451128314",
            "extra": "mean: 436.50827497447165 usec\nrounds: 1982"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000]",
            "value": 271.40105180013694,
            "unit": "iter/sec",
            "range": "stddev: 0.005181963989337887",
            "extra": "mean: 3.6845840993144434 msec\nrounds: 292"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10000]",
            "value": 27.66839879814164,
            "unit": "iter/sec",
            "range": "stddev: 0.016391871329538267",
            "extra": "mean: 36.14231554545778 msec\nrounds: 44"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100000]",
            "value": 1.9800057020441295,
            "unit": "iter/sec",
            "range": "stddev: 0.04978076435391436",
            "extra": "mean: 505.04905060001306 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000000]",
            "value": 0.16128501141839474,
            "unit": "iter/sec",
            "range": "stddev: 0.3380950717088026",
            "extra": "mean: 6.20020416779999 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1]",
            "value": 8224.611682956309,
            "unit": "iter/sec",
            "range": "stddev: 0.00042979692154312824",
            "extra": "mean: 121.58628741977923 usec\nrounds: 2345"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10]",
            "value": 4222.071832772176,
            "unit": "iter/sec",
            "range": "stddev: 0.00002387701487138647",
            "extra": "mean: 236.85054153695165 usec\nrounds: 1926"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100]",
            "value": 736.5295747059442,
            "unit": "iter/sec",
            "range": "stddev: 0.00004996954414621286",
            "extra": "mean: 1.3577187316602257 msec\nrounds: 518"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000]",
            "value": 81.9320350324266,
            "unit": "iter/sec",
            "range": "stddev: 0.0004803593905488856",
            "extra": "mean: 12.205238153845754 msec\nrounds: 78"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10000]",
            "value": 7.349307146732827,
            "unit": "iter/sec",
            "range": "stddev: 0.004094219988950379",
            "extra": "mean: 136.06724825000072 msec\nrounds: 8"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100000]",
            "value": 0.6411899045079356,
            "unit": "iter/sec",
            "range": "stddev: 0.05211569372201889",
            "extra": "mean: 1.559600350800008 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000000]",
            "value": 0.0626768415787644,
            "unit": "iter/sec",
            "range": "stddev: 0.2805890191484861",
            "extra": "mean: 15.954856288400004 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1]",
            "value": 7044.015665564571,
            "unit": "iter/sec",
            "range": "stddev: 0.00001755269376768637",
            "extra": "mean: 141.9644770082792 usec\nrounds: 1457"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10]",
            "value": 2281.9841265760256,
            "unit": "iter/sec",
            "range": "stddev: 0.00001896667995779672",
            "extra": "mean: 438.21514284607997 usec\nrounds: 7"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100]",
            "value": 307.94930667041416,
            "unit": "iter/sec",
            "range": "stddev: 0.0000509608713907671",
            "extra": "mean: 3.2472877137218563 msec\nrounds: 255"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000]",
            "value": 31.043904027537135,
            "unit": "iter/sec",
            "range": "stddev: 0.0028631302313007396",
            "extra": "mean: 32.212443354835834 msec\nrounds: 31"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10000]",
            "value": 2.7518829081836653,
            "unit": "iter/sec",
            "range": "stddev: 0.011235605466465114",
            "extra": "mean: 363.38755439999204 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100000]",
            "value": 0.2535423381268546,
            "unit": "iter/sec",
            "range": "stddev: 0.0320589456233088",
            "extra": "mean: 3.944114452000008 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000000]",
            "value": 0.025244199706409887,
            "unit": "iter/sec",
            "range": "stddev: 0.2956651395374158",
            "extra": "mean: 39.613060094199966 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1]",
            "value": 11430.943495817819,
            "unit": "iter/sec",
            "range": "stddev: 0.00001191748647931303",
            "extra": "mean: 87.48184262881405 usec\nrounds: 5433"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10]",
            "value": 11567.22415584646,
            "unit": "iter/sec",
            "range": "stddev: 0.000009060109827639318",
            "extra": "mean: 86.45116464649531 usec\nrounds: 6517"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100]",
            "value": 10897.528248006438,
            "unit": "iter/sec",
            "range": "stddev: 0.000015548331156741712",
            "extra": "mean: 91.76392822683778 usec\nrounds: 2912"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000]",
            "value": 11670.291599872458,
            "unit": "iter/sec",
            "range": "stddev: 0.000006082331044175635",
            "extra": "mean: 85.68766182422802 usec\nrounds: 482"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10000]",
            "value": 10153.143243107103,
            "unit": "iter/sec",
            "range": "stddev: 0.000034744653000123725",
            "extra": "mean: 98.49166667464216 usec\nrounds: 57"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100000]",
            "value": 2545.898735396574,
            "unit": "iter/sec",
            "range": "stddev: 0.0006202034638357867",
            "extra": "mean: 392.78859999285487 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000000]",
            "value": 806.666550521729,
            "unit": "iter/sec",
            "range": "stddev: 0.002508182633884397",
            "extra": "mean: 1.239669599976878 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1]",
            "value": 9714.56609424783,
            "unit": "iter/sec",
            "range": "stddev: 0.000017528430955849293",
            "extra": "mean: 102.93820540189829 usec\nrounds: 5998"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10]",
            "value": 7738.331443190837,
            "unit": "iter/sec",
            "range": "stddev: 0.002008353736403395",
            "extra": "mean: 129.22682458631655 usec\nrounds: 5621"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100]",
            "value": 10611.122253275113,
            "unit": "iter/sec",
            "range": "stddev: 0.000014695103148528985",
            "extra": "mean: 94.24073873914242 usec\nrounds: 2997"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000]",
            "value": 8445.157307687849,
            "unit": "iter/sec",
            "range": "stddev: 0.00001486734591608161",
            "extra": "mean: 118.41105660514738 usec\nrounds: 477"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10000]",
            "value": 291.6021567772656,
            "unit": "iter/sec",
            "range": "stddev: 0.02570417058926417",
            "extra": "mean: 3.4293299166639217 msec\nrounds: 60"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100000]",
            "value": 1898.9529173010483,
            "unit": "iter/sec",
            "range": "stddev: 0.0009043036825893524",
            "extra": "mean: 526.6060000167272 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000000]",
            "value": 191.54329458775425,
            "unit": "iter/sec",
            "range": "stddev: 0.011367610657826365",
            "extra": "mean: 5.2207518000159325 msec\nrounds: 5"
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
          "id": "10d6a7f373e78ab2b08a73f6623b82fdf938d710",
          "message": "Merge pull request #93 from FalkorDB/property-based-testing\n\nproperty based testing",
          "timestamp": "2025-06-12T14:26:43+03:00",
          "tree_id": "81784c82d967549cec9514a6543e46e61fe193e4",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/10d6a7f373e78ab2b08a73f6623b82fdf938d710"
        },
        "date": 1749728612031,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 8447.416443535942,
            "unit": "iter/sec",
            "range": "stddev: 0.000016010471222438725",
            "extra": "mean: 118.379389329765 usec\nrounds: 2268"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1]",
            "value": 10617.2570430454,
            "unit": "iter/sec",
            "range": "stddev: 0.000009861429442505696",
            "extra": "mean: 94.18628520960863 usec\nrounds: 5943"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10]",
            "value": 7101.136768319549,
            "unit": "iter/sec",
            "range": "stddev: 0.000017852144437171233",
            "extra": "mean: 140.8225235797909 usec\nrounds: 4771"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100]",
            "value": 2057.8091748290235,
            "unit": "iter/sec",
            "range": "stddev: 0.000022132133523590643",
            "extra": "mean: 485.95370855175946 usec\nrounds: 1719"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000]",
            "value": 244.28672904505405,
            "unit": "iter/sec",
            "range": "stddev: 0.001516009434890745",
            "extra": "mean: 4.093550246913204 msec\nrounds: 243"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10000]",
            "value": 24.01808392632239,
            "unit": "iter/sec",
            "range": "stddev: 0.004648839441106398",
            "extra": "mean: 41.63529460000177 msec\nrounds: 25"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100000]",
            "value": 2.1995419116999138,
            "unit": "iter/sec",
            "range": "stddev: 0.01632895816077422",
            "extra": "mean: 454.64012059999845 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000000]",
            "value": 0.21369994048436508,
            "unit": "iter/sec",
            "range": "stddev: 0.07528544636676424",
            "extra": "mean: 4.679458486200014 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1]",
            "value": 9444.938612085632,
            "unit": "iter/sec",
            "range": "stddev: 0.000021026472039893042",
            "extra": "mean: 105.8768130817083 usec\nrounds: 3975"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10]",
            "value": 8602.309754346057,
            "unit": "iter/sec",
            "range": "stddev: 0.00005237849037544833",
            "extra": "mean: 116.24784837523204 usec\nrounds: 5540"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100]",
            "value": 4204.663985642742,
            "unit": "iter/sec",
            "range": "stddev: 0.0004686870762134029",
            "extra": "mean: 237.83113309757996 usec\nrounds: 3396"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000]",
            "value": 735.5921884023568,
            "unit": "iter/sec",
            "range": "stddev: 0.0019162652563354467",
            "extra": "mean: 1.359448911729085 msec\nrounds: 827"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10000]",
            "value": 80.45498351609841,
            "unit": "iter/sec",
            "range": "stddev: 0.00626772500944751",
            "extra": "mean: 12.429310855553252 msec\nrounds: 90"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100000]",
            "value": 5.877828465427713,
            "unit": "iter/sec",
            "range": "stddev: 0.029191376641871464",
            "extra": "mean: 170.13085799999317 msec\nrounds: 8"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000000]",
            "value": 0.45639095064192897,
            "unit": "iter/sec",
            "range": "stddev: 0.12611671016662274",
            "extra": "mean: 2.1911039178000067 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1]",
            "value": 7374.406017800871,
            "unit": "iter/sec",
            "range": "stddev: 0.000010500007249148282",
            "extra": "mean: 135.60414189103884 usec\nrounds: 148"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10]",
            "value": 3976.916932257028,
            "unit": "iter/sec",
            "range": "stddev: 0.0047729709025898905",
            "extra": "mean: 251.45106549471421 usec\nrounds: 3527"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100]",
            "value": 2346.0895591510503,
            "unit": "iter/sec",
            "range": "stddev: 0.0010775999850787382",
            "extra": "mean: 426.24118763899935 usec\nrounds: 2249"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000]",
            "value": 282.0494307276404,
            "unit": "iter/sec",
            "range": "stddev: 0.005182159467553014",
            "extra": "mean: 3.545477817204478 msec\nrounds: 279"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10000]",
            "value": 28.357305547299266,
            "unit": "iter/sec",
            "range": "stddev: 0.01415453339917073",
            "extra": "mean: 35.26428130952094 msec\nrounds: 42"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100000]",
            "value": 2.036715914450661,
            "unit": "iter/sec",
            "range": "stddev: 0.05805612152454575",
            "extra": "mean: 490.9864910000067 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000000]",
            "value": 0.17040237387094448,
            "unit": "iter/sec",
            "range": "stddev: 0.385629832636041",
            "extra": "mean: 5.868462846400002 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1]",
            "value": 6836.29239867278,
            "unit": "iter/sec",
            "range": "stddev: 0.0003153411731830688",
            "extra": "mean: 146.2781200222131 usec\nrounds: 1808"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10]",
            "value": 4608.375680230333,
            "unit": "iter/sec",
            "range": "stddev: 0.000016785559509573597",
            "extra": "mean: 216.99619766026078 usec\nrounds: 1710"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100]",
            "value": 745.1356787403295,
            "unit": "iter/sec",
            "range": "stddev: 0.000028515934058879946",
            "extra": "mean: 1.3420374685191898 msec\nrounds: 540"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000]",
            "value": 75.58263970894652,
            "unit": "iter/sec",
            "range": "stddev: 0.003124100734789904",
            "extra": "mean: 13.23055140506865 msec\nrounds: 79"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10000]",
            "value": 7.061952364793713,
            "unit": "iter/sec",
            "range": "stddev: 0.005578014921531866",
            "extra": "mean: 141.6039005000016 msec\nrounds: 8"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100000]",
            "value": 0.6411670591338323,
            "unit": "iter/sec",
            "range": "stddev: 0.019466620279069563",
            "extra": "mean: 1.5596559207999916 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000000]",
            "value": 0.06309609306504112,
            "unit": "iter/sec",
            "range": "stddev: 0.40304172387446463",
            "extra": "mean: 15.848841844600008 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1]",
            "value": 7076.8985139983215,
            "unit": "iter/sec",
            "range": "stddev: 0.00001948579454173387",
            "extra": "mean: 141.30483827371123 usec\nrounds: 1552"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10]",
            "value": 2342.797552158173,
            "unit": "iter/sec",
            "range": "stddev: 0.000019034059006958463",
            "extra": "mean: 426.84012499449864 usec\nrounds: 8"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100]",
            "value": 309.19159858768654,
            "unit": "iter/sec",
            "range": "stddev: 0.000057405157404557904",
            "extra": "mean: 3.2342405310097733 msec\nrounds: 258"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000]",
            "value": 31.96274642819377,
            "unit": "iter/sec",
            "range": "stddev: 0.0010567873747916341",
            "extra": "mean: 31.28642284374905 msec\nrounds: 32"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10000]",
            "value": 3.1086534141430002,
            "unit": "iter/sec",
            "range": "stddev: 0.0021418539750162916",
            "extra": "mean: 321.6826924000088 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100000]",
            "value": 0.25548145165373115,
            "unit": "iter/sec",
            "range": "stddev: 0.04566846933871993",
            "extra": "mean: 3.914178479599991 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000000]",
            "value": 0.025569644898313388,
            "unit": "iter/sec",
            "range": "stddev: 0.5393520846051773",
            "extra": "mean: 39.10887319619998 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1]",
            "value": 11105.883146495476,
            "unit": "iter/sec",
            "range": "stddev: 0.000015629817501717324",
            "extra": "mean: 90.0423664475126 usec\nrounds: 4560"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10]",
            "value": 10193.852513530026,
            "unit": "iter/sec",
            "range": "stddev: 0.000017804721767320394",
            "extra": "mean: 98.09833904038997 usec\nrounds: 6483"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100]",
            "value": 11502.628100099095,
            "unit": "iter/sec",
            "range": "stddev: 0.000013329245921552832",
            "extra": "mean: 86.93665406702881 usec\nrounds: 2778"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000]",
            "value": 12197.58596514608,
            "unit": "iter/sec",
            "range": "stddev: 0.000006162887940441334",
            "extra": "mean: 81.9834353172377 usec\nrounds: 487"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10000]",
            "value": 10761.949823256718,
            "unit": "iter/sec",
            "range": "stddev: 0.00004705340189527963",
            "extra": "mean: 92.91996491555709 usec\nrounds: 57"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100000]",
            "value": 2028.2000941148492,
            "unit": "iter/sec",
            "range": "stddev: 0.0008505277119994807",
            "extra": "mean: 493.04799999845267 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000000]",
            "value": 389.55047978066864,
            "unit": "iter/sec",
            "range": "stddev: 0.005322464897818056",
            "extra": "mean: 2.567061400009152 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1]",
            "value": 9684.41994101543,
            "unit": "iter/sec",
            "range": "stddev: 0.00001664501468943431",
            "extra": "mean: 103.25863666493876 usec\nrounds: 6022"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10]",
            "value": 8024.109429238398,
            "unit": "iter/sec",
            "range": "stddev: 0.0020308955959575966",
            "extra": "mean: 124.62442203943301 usec\nrounds: 5227"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100]",
            "value": 10816.687062599609,
            "unit": "iter/sec",
            "range": "stddev: 0.000015086759641614529",
            "extra": "mean: 92.44974863492693 usec\nrounds: 2566"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000]",
            "value": 11502.265131155991,
            "unit": "iter/sec",
            "range": "stddev: 0.000011471654446796993",
            "extra": "mean: 86.93939746627096 usec\nrounds: 473"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10000]",
            "value": 290.9476649217229,
            "unit": "iter/sec",
            "range": "stddev: 0.025549910441353588",
            "extra": "mean: 3.4370442542271027 msec\nrounds: 59"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100000]",
            "value": 2202.8935446640407,
            "unit": "iter/sec",
            "range": "stddev: 0.0007704360511006552",
            "extra": "mean: 453.9484000133598 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000000]",
            "value": 301.4870729256554,
            "unit": "iter/sec",
            "range": "stddev: 0.007133095421428195",
            "extra": "mean: 3.3168918000228587 msec\nrounds: 5"
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
          "id": "b0c92a319e3c99f1a7c919ce2401052740a64f8f",
          "message": "Merge pull request #94 from FalkorDB/property-based-test-2\n\nrewrite tests to property based",
          "timestamp": "2025-06-12T21:02:41+03:00",
          "tree_id": "ae0be02ff6d8585b396876ddb9c8ad5574c311d7",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/b0c92a319e3c99f1a7c919ce2401052740a64f8f"
        },
        "date": 1749752073865,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 10021.092717270069,
            "unit": "iter/sec",
            "range": "stddev: 0.000015706276400544266",
            "extra": "mean: 99.78951679357564 usec\nrounds: 2233"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1]",
            "value": 9550.643867323499,
            "unit": "iter/sec",
            "range": "stddev: 0.000016389701074270037",
            "extra": "mean: 104.70498260555945 usec\nrounds: 5289"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10]",
            "value": 6796.273633662481,
            "unit": "iter/sec",
            "range": "stddev: 0.00001763402601773085",
            "extra": "mean: 147.13945522247968 usec\nrounds: 4835"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100]",
            "value": 2024.5718569742057,
            "unit": "iter/sec",
            "range": "stddev: 0.000019756307604468364",
            "extra": "mean: 493.93159178579884 usec\nrounds: 1193"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000]",
            "value": 235.43554798243332,
            "unit": "iter/sec",
            "range": "stddev: 0.0017122450603628452",
            "extra": "mean: 4.247446949152358 msec\nrounds: 236"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10000]",
            "value": 24.023349840815452,
            "unit": "iter/sec",
            "range": "stddev: 0.005172317303717123",
            "extra": "mean: 41.62616814999751 msec\nrounds: 20"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100000]",
            "value": 2.1525793591711535,
            "unit": "iter/sec",
            "range": "stddev: 0.007163842271566506",
            "extra": "mean: 464.5589468000139 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000000]",
            "value": 0.20949976342029733,
            "unit": "iter/sec",
            "range": "stddev: 0.11305017703759086",
            "extra": "mean: 4.773275079999996 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1]",
            "value": 10855.331450558886,
            "unit": "iter/sec",
            "range": "stddev: 0.000010309862517026902",
            "extra": "mean: 92.12063257161208 usec\nrounds: 4507"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10]",
            "value": 9337.472494826092,
            "unit": "iter/sec",
            "range": "stddev: 0.00006469099213084725",
            "extra": "mean: 107.09536232145278 usec\nrounds: 5324"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100]",
            "value": 4131.420477570442,
            "unit": "iter/sec",
            "range": "stddev: 0.0004711728681934702",
            "extra": "mean: 242.04750047326783 usec\nrounds: 3173"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000]",
            "value": 716.3876091019581,
            "unit": "iter/sec",
            "range": "stddev: 0.0019509805517932185",
            "extra": "mean: 1.3958923734786113 msec\nrounds: 822"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10000]",
            "value": 77.50567174348345,
            "unit": "iter/sec",
            "range": "stddev: 0.008056086634459059",
            "extra": "mean: 12.90228156862699 msec\nrounds: 102"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100000]",
            "value": 5.740231416783638,
            "unit": "iter/sec",
            "range": "stddev: 0.027620832304896852",
            "extra": "mean: 174.20900437500464 msec\nrounds: 8"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000000]",
            "value": 0.446013097644173,
            "unit": "iter/sec",
            "range": "stddev: 0.11445329067440618",
            "extra": "mean: 2.2420866232000094 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1]",
            "value": 8740.776657799312,
            "unit": "iter/sec",
            "range": "stddev: 0.00014917139250690778",
            "extra": "mean: 114.40630954775736 usec\nrounds: 3027"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10]",
            "value": 4651.472383532423,
            "unit": "iter/sec",
            "range": "stddev: 0.004257377777757317",
            "extra": "mean: 214.98569002372093 usec\nrounds: 4681"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100]",
            "value": 2308.8580514830624,
            "unit": "iter/sec",
            "range": "stddev: 0.0011035704661547959",
            "extra": "mean: 433.11454307797925 usec\nrounds: 2066"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000]",
            "value": 274.42242813548563,
            "unit": "iter/sec",
            "range": "stddev: 0.005389285791749631",
            "extra": "mean: 3.64401702438945 msec\nrounds: 287"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10000]",
            "value": 28.709077398504263,
            "unit": "iter/sec",
            "range": "stddev: 0.0181456923303998",
            "extra": "mean: 34.832188653059944 msec\nrounds: 49"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100000]",
            "value": 2.0041636483761565,
            "unit": "iter/sec",
            "range": "stddev: 0.05579674000893838",
            "extra": "mean: 498.96125040000356 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000000]",
            "value": 0.16406249345493168,
            "unit": "iter/sec",
            "range": "stddev: 0.4664271031474039",
            "extra": "mean: 6.095238338400009 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1]",
            "value": 6909.391137829003,
            "unit": "iter/sec",
            "range": "stddev: 0.000013006866582570572",
            "extra": "mean: 144.73055296073593 usec\nrounds: 1841"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10]",
            "value": 4351.062059120169,
            "unit": "iter/sec",
            "range": "stddev: 0.00002036516605712432",
            "extra": "mean: 229.82894438472124 usec\nrounds: 1870"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100]",
            "value": 745.5558918472879,
            "unit": "iter/sec",
            "range": "stddev: 0.000030462600124769483",
            "extra": "mean: 1.3412810641496882 msec\nrounds: 530"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000]",
            "value": 80.37612376634839,
            "unit": "iter/sec",
            "range": "stddev: 0.00019975839054894564",
            "extra": "mean: 12.441505675329378 msec\nrounds: 77"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10000]",
            "value": 7.0226156974291,
            "unit": "iter/sec",
            "range": "stddev: 0.007341134885891188",
            "extra": "mean: 142.39708437499843 msec\nrounds: 8"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100000]",
            "value": 0.6037540026070832,
            "unit": "iter/sec",
            "range": "stddev: 0.035479999266932696",
            "extra": "mean: 1.6563037192000025 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000000]",
            "value": 0.06213276760891718,
            "unit": "iter/sec",
            "range": "stddev: 0.4119827165170316",
            "extra": "mean: 16.094567141999995 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1]",
            "value": 6836.1367448105975,
            "unit": "iter/sec",
            "range": "stddev: 0.00002034213946675624",
            "extra": "mean: 146.28145066862703 usec\nrounds: 1571"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10]",
            "value": 2383.0851002588347,
            "unit": "iter/sec",
            "range": "stddev: 0.00001783006277785943",
            "extra": "mean: 419.62412500140545 usec\nrounds: 8"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100]",
            "value": 310.7982110857829,
            "unit": "iter/sec",
            "range": "stddev: 0.00004305070554287323",
            "extra": "mean: 3.2175217370346827 msec\nrounds: 270"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000]",
            "value": 32.662824101758744,
            "unit": "iter/sec",
            "range": "stddev: 0.00043573849829388653",
            "extra": "mean: 30.615846225806134 msec\nrounds: 31"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10000]",
            "value": 2.9740166382014737,
            "unit": "iter/sec",
            "range": "stddev: 0.016011510459141017",
            "extra": "mean: 336.2455970000042 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100000]",
            "value": 0.2507217794256219,
            "unit": "iter/sec",
            "range": "stddev: 0.040024888878439115",
            "extra": "mean: 3.988484774999995 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000000]",
            "value": 0.025383624734292305,
            "unit": "iter/sec",
            "range": "stddev: 0.6347948117783834",
            "extra": "mean: 39.39547682680001 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1]",
            "value": 9428.688735144984,
            "unit": "iter/sec",
            "range": "stddev: 0.000017608106616982896",
            "extra": "mean: 106.05928651272028 usec\nrounds: 5731"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10]",
            "value": 10497.979119647729,
            "unit": "iter/sec",
            "range": "stddev: 0.000017566750272114775",
            "extra": "mean: 95.25642874716978 usec\nrounds: 6512"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100]",
            "value": 10436.883827791746,
            "unit": "iter/sec",
            "range": "stddev: 0.000017235799171154705",
            "extra": "mean: 95.8140395639128 usec\nrounds: 3210"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000]",
            "value": 11842.611596955014,
            "unit": "iter/sec",
            "range": "stddev: 0.000008803309735239181",
            "extra": "mean: 84.44083400127056 usec\nrounds: 500"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10000]",
            "value": 10513.579117114929,
            "unit": "iter/sec",
            "range": "stddev: 0.000049872801231901294",
            "extra": "mean: 95.1150877223259 usec\nrounds: 57"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100000]",
            "value": 2635.216933566557,
            "unit": "iter/sec",
            "range": "stddev: 0.0005846193161379366",
            "extra": "mean: 379.47540001823654 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000000]",
            "value": 684.1467062206914,
            "unit": "iter/sec",
            "range": "stddev: 0.003016922470317778",
            "extra": "mean: 1.4616748000207735 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1]",
            "value": 9967.00878598025,
            "unit": "iter/sec",
            "range": "stddev: 0.00006421387145228217",
            "extra": "mean: 100.33100416311618 usec\nrounds: 6005"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10]",
            "value": 7868.806183638074,
            "unit": "iter/sec",
            "range": "stddev: 0.0019405308689465738",
            "extra": "mean: 127.0840806931222 usec\nrounds: 6122"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100]",
            "value": 11174.893680488622,
            "unit": "iter/sec",
            "range": "stddev: 0.000012462086454762108",
            "extra": "mean: 89.48630999022399 usec\nrounds: 3013"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000]",
            "value": 11466.185430157966,
            "unit": "iter/sec",
            "range": "stddev: 0.000006355405096753838",
            "extra": "mean: 87.21296250537117 usec\nrounds: 480"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10000]",
            "value": 293.31785410263717,
            "unit": "iter/sec",
            "range": "stddev: 0.025510582299380438",
            "extra": "mean: 3.4092708166686707 msec\nrounds: 60"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100000]",
            "value": 2205.4286184044636,
            "unit": "iter/sec",
            "range": "stddev: 0.0007426279061910925",
            "extra": "mean: 453.42660000642354 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000000]",
            "value": 235.02238752804925,
            "unit": "iter/sec",
            "range": "stddev: 0.009208779839580763",
            "extra": "mean: 4.2549137999913 msec\nrounds: 5"
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
          "id": "36412997e69d0fb7089c2c363ac21f6333b4ca9c",
          "message": "Merge pull request #95 from FalkorDB/work-orderby-skip-limit\n\norder by skip limit",
          "timestamp": "2025-06-15T17:57:41+03:00",
          "tree_id": "417f6666e726e8d0d13e714a2b81e97a412e5a17",
          "url": "https://github.com/FalkorDB/falkordb-rs-next-gen/commit/36412997e69d0fb7089c2c363ac21f6333b4ca9c"
        },
        "date": 1750000238926,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_bench.py::test_return",
            "value": 10197.19998516457,
            "unit": "iter/sec",
            "range": "stddev: 0.000012947259337412373",
            "extra": "mean: 98.0661359446567 usec\nrounds: 2170"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1]",
            "value": 9642.301363813252,
            "unit": "iter/sec",
            "range": "stddev: 0.000026050783422380075",
            "extra": "mean: 103.70968115069667 usec\nrounds: 5040"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10]",
            "value": 7051.9766744311,
            "unit": "iter/sec",
            "range": "stddev: 0.00001767730469564219",
            "extra": "mean: 141.80421265795982 usec\nrounds: 4740"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100]",
            "value": 2036.7915134602629,
            "unit": "iter/sec",
            "range": "stddev: 0.000017978171172330318",
            "extra": "mean: 490.96826719447625 usec\nrounds: 1134"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000]",
            "value": 232.47270764744366,
            "unit": "iter/sec",
            "range": "stddev: 0.001836734004343089",
            "extra": "mean: 4.301580216102397 msec\nrounds: 236"
          },
          {
            "name": "tests/test_bench.py::test_unwind[10000]",
            "value": 23.07135461994405,
            "unit": "iter/sec",
            "range": "stddev: 0.005243575580493261",
            "extra": "mean: 43.34379218182314 msec\nrounds: 22"
          },
          {
            "name": "tests/test_bench.py::test_unwind[100000]",
            "value": 2.1036483186139576,
            "unit": "iter/sec",
            "range": "stddev: 0.006927230967493874",
            "extra": "mean: 475.36462779999056 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_unwind[1000000]",
            "value": 0.2056264760752911,
            "unit": "iter/sec",
            "range": "stddev: 0.07505646823072212",
            "extra": "mean: 4.863186974199982 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1]",
            "value": 9569.398873062737,
            "unit": "iter/sec",
            "range": "stddev: 0.000019781627392883274",
            "extra": "mean: 104.4997719569343 usec\nrounds: 3894"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10]",
            "value": 8082.774252326703,
            "unit": "iter/sec",
            "range": "stddev: 0.00005518992887021103",
            "extra": "mean: 123.71989725089013 usec\nrounds: 5129"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100]",
            "value": 4151.228646788317,
            "unit": "iter/sec",
            "range": "stddev: 0.0005164325425179792",
            "extra": "mean: 240.89253690559073 usec\nrounds: 2967"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000]",
            "value": 662.2046209809959,
            "unit": "iter/sec",
            "range": "stddev: 0.002382946305969114",
            "extra": "mean: 1.5101072513184686 msec\nrounds: 569"
          },
          {
            "name": "tests/test_bench.py::test_create_node[10000]",
            "value": 75.32546409966247,
            "unit": "iter/sec",
            "range": "stddev: 0.008050822798022219",
            "extra": "mean: 13.27572305000217 msec\nrounds: 100"
          },
          {
            "name": "tests/test_bench.py::test_create_node[100000]",
            "value": 5.2990870997699435,
            "unit": "iter/sec",
            "range": "stddev: 0.0302823666613",
            "extra": "mean: 188.71174999999798 msec\nrounds: 7"
          },
          {
            "name": "tests/test_bench.py::test_create_node[1000000]",
            "value": 0.42239482041277765,
            "unit": "iter/sec",
            "range": "stddev: 0.12698591049393734",
            "extra": "mean: 2.3674532728000033 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1]",
            "value": 9787.8941905505,
            "unit": "iter/sec",
            "range": "stddev: 0.00017753703722602954",
            "extra": "mean: 102.16702188765252 usec\nrounds: 2924"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10]",
            "value": 4254.846904843674,
            "unit": "iter/sec",
            "range": "stddev: 0.004273239703780807",
            "extra": "mean: 235.02608257458346 usec\nrounds: 4614"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100]",
            "value": 2228.8422336113817,
            "unit": "iter/sec",
            "range": "stddev: 0.0011834277805376344",
            "extra": "mean: 448.663429344528 usec\nrounds: 1847"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000]",
            "value": 268.91678800761105,
            "unit": "iter/sec",
            "range": "stddev: 0.005555873106807763",
            "extra": "mean: 3.718622431157766 msec\nrounds: 276"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[10000]",
            "value": 27.543014664391116,
            "unit": "iter/sec",
            "range": "stddev: 0.0193673496018981",
            "extra": "mean: 36.306846297868994 msec\nrounds: 47"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[100000]",
            "value": 1.860806636439117,
            "unit": "iter/sec",
            "range": "stddev: 0.07079472395483528",
            "extra": "mean: 537.4013508000075 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_create_relationship[1000000]",
            "value": 0.16067056379085362,
            "unit": "iter/sec",
            "range": "stddev: 0.3934413690696006",
            "extra": "mean: 6.2239154230000056 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1]",
            "value": 8917.553210352262,
            "unit": "iter/sec",
            "range": "stddev: 0.000016105007228625734",
            "extra": "mean: 112.13838329992964 usec\nrounds: 1988"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10]",
            "value": 4249.337941059499,
            "unit": "iter/sec",
            "range": "stddev: 0.000025464618400901256",
            "extra": "mean: 235.33077713999543 usec\nrounds: 1916"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100]",
            "value": 734.1969953507304,
            "unit": "iter/sec",
            "range": "stddev: 0.0001875717335784186",
            "extra": "mean: 1.3620322697211447 msec\nrounds: 393"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000]",
            "value": 75.32660749409752,
            "unit": "iter/sec",
            "range": "stddev: 0.0028024914077740123",
            "extra": "mean: 13.27552153571178 msec\nrounds: 56"
          },
          {
            "name": "tests/test_bench.py::test_match_node[10000]",
            "value": 7.087905644649134,
            "unit": "iter/sec",
            "range": "stddev: 0.005090023403447051",
            "extra": "mean: 141.08539957144168 msec\nrounds: 7"
          },
          {
            "name": "tests/test_bench.py::test_match_node[100000]",
            "value": 0.6321933655061259,
            "unit": "iter/sec",
            "range": "stddev: 0.01977677255023216",
            "extra": "mean: 1.5817945182000017 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_node[1000000]",
            "value": 0.0625944187278675,
            "unit": "iter/sec",
            "range": "stddev: 0.37793608423243347",
            "extra": "mean: 15.975865266000028 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1]",
            "value": 7419.22639340186,
            "unit": "iter/sec",
            "range": "stddev: 0.000010946593324939059",
            "extra": "mean: 134.78494211867292 usec\nrounds: 1745"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10]",
            "value": 2404.062143730023,
            "unit": "iter/sec",
            "range": "stddev: 0.000015638024407665537",
            "extra": "mean: 415.96262501286674 usec\nrounds: 8"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100]",
            "value": 315.518955472936,
            "unit": "iter/sec",
            "range": "stddev: 0.00004517436219947478",
            "extra": "mean: 3.1693816889736 msec\nrounds: 254"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000]",
            "value": 32.382817721835956,
            "unit": "iter/sec",
            "range": "stddev: 0.00034577687862671593",
            "extra": "mean: 30.880574031261432 msec\nrounds: 32"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[10000]",
            "value": 2.7838739188044586,
            "unit": "iter/sec",
            "range": "stddev: 0.011868869959585278",
            "extra": "mean: 359.2116702000112 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[100000]",
            "value": 0.25019518186496176,
            "unit": "iter/sec",
            "range": "stddev: 0.04490630574533354",
            "extra": "mean: 3.996879526400039 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_match_relationship[1000000]",
            "value": 0.0252235046860317,
            "unit": "iter/sec",
            "range": "stddev: 0.674693611432144",
            "extra": "mean: 39.6455612512 sec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1]",
            "value": 11108.438266411886,
            "unit": "iter/sec",
            "range": "stddev: 0.000015182266462119679",
            "extra": "mean: 90.02165525136488 usec\nrounds: 5694"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10]",
            "value": 10219.364377565395,
            "unit": "iter/sec",
            "range": "stddev: 0.00001700931612880003",
            "extra": "mean: 97.85344401607827 usec\nrounds: 5448"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100]",
            "value": 9489.861761343893,
            "unit": "iter/sec",
            "range": "stddev: 0.000017395384526220285",
            "extra": "mean: 105.3756129592331 usec\nrounds: 2948"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000]",
            "value": 8999.65061405219,
            "unit": "iter/sec",
            "range": "stddev: 0.000015567792187152507",
            "extra": "mean: 111.11542468532998 usec\nrounds: 478"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[10000]",
            "value": 10017.415021508265,
            "unit": "iter/sec",
            "range": "stddev: 0.00004547523581467553",
            "extra": "mean: 99.82615254064173 usec\nrounds: 59"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[100000]",
            "value": 1893.1828378999944,
            "unit": "iter/sec",
            "range": "stddev: 0.0009257013178524368",
            "extra": "mean: 528.2110000052853 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_node[1000000]",
            "value": 345.7339336751071,
            "unit": "iter/sec",
            "range": "stddev: 0.00617461068463991",
            "extra": "mean: 2.8923975999987306 msec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1]",
            "value": 9414.022673174928,
            "unit": "iter/sec",
            "range": "stddev: 0.000016952600649744497",
            "extra": "mean: 106.22451578000553 usec\nrounds: 5735"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10]",
            "value": 7656.816651138442,
            "unit": "iter/sec",
            "range": "stddev: 0.0020972416736225416",
            "extra": "mean: 130.60257879510758 usec\nrounds: 5178"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100]",
            "value": 10947.549662932033,
            "unit": "iter/sec",
            "range": "stddev: 0.00001028710269671972",
            "extra": "mean: 91.34464156723219 usec\nrounds: 2935"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000]",
            "value": 8314.023794090892,
            "unit": "iter/sec",
            "range": "stddev: 0.000015031427434605197",
            "extra": "mean: 120.27870316064525 usec\nrounds: 475"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[10000]",
            "value": 282.0486494723953,
            "unit": "iter/sec",
            "range": "stddev: 0.0260104338941245",
            "extra": "mean: 3.5454876379327316 msec\nrounds: 58"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[100000]",
            "value": 2310.2051002387375,
            "unit": "iter/sec",
            "range": "stddev: 0.000714012752164942",
            "extra": "mean: 432.86199995691277 usec\nrounds: 5"
          },
          {
            "name": "tests/test_bench.py::test_delete_relationship[1000000]",
            "value": 195.7041759157277,
            "unit": "iter/sec",
            "range": "stddev: 0.01110773341289871",
            "extra": "mean: 5.109753000010642 msec\nrounds: 5"
          }
        ]
      }
    ]
  }
}