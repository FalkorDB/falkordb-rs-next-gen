window.BENCHMARK_DATA = {
  "lastUpdate": 1749450333755,
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
      }
    ]
  }
}