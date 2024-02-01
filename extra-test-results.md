## Running the code with `cargo run --release`

#### 10sec /lock

```Bash

oha http://127.0.0.1:8080/lock -z 10s -q 1000 -c 10

Summary:
  Success rate: 100.00%
  Total:        10.0097 secs
  Slowest:      0.0157 secs
  Fastest:      0.0002 secs
  Average:      0.0012 secs
  Requests/sec: 998.5348

  Total data:   48.80 KiB
  Size/request: 5
  Size/sec:     4.88 KiB

Response time histogram:
  0.000 [1]    |
  0.002 [8486] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.003 [1484] |■■■■■
  0.005 [16]   |
  0.006 [0]    |
  0.008 [0]    |
  0.009 [0]    |
  0.011 [0]    |
  0.013 [0]    |
  0.014 [5]    |
  0.016 [3]    |

Response time distribution:
  10.00% in 0.0005 secs
  25.00% in 0.0007 secs
  50.00% in 0.0011 secs
  75.00% in 0.0015 secs
  90.00% in 0.0019 secs
  95.00% in 0.0022 secs
  99.00% in 0.0027 secs
  99.90% in 0.0040 secs
  99.99% in 0.0157 secs


Details (average, fastest, slowest):
  DNS+dialup:   0.0100 secs, 0.0019 secs, 0.0124 secs
  DNS-lookup:   0.0001 secs, 0.0000 secs, 0.0002 secs

Status code distribution:
  [200] 9995 responses

  ```

#### 10sec /nolock

```Bash

oha http://127.0.0.1:8080/nolock -z 10s -q 1000 -c 10

Summary:
  Success rate: 100.00%
  Total:        10.0030 secs
  Slowest:      0.0141 secs
  Fastest:      0.0002 secs
  Average:      0.0007 secs
  Requests/sec: 998.4988

  Total data:   48.77 KiB
  Size/request: 5
  Size/sec:     4.88 KiB

Response time histogram:
  0.000 [1]    |
  0.002 [9758] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.003 [219]  |
  0.004 [2]    |
  0.006 [0]    |
  0.007 [0]    |
  0.009 [0]    |
  0.010 [0]    |
  0.011 [0]    |
  0.013 [3]    |
  0.014 [5]    |

Response time distribution:
  10.00% in 0.0004 secs
  25.00% in 0.0005 secs
  50.00% in 0.0007 secs
  75.00% in 0.0009 secs
  90.00% in 0.0012 secs
  95.00% in 0.0014 secs
  99.00% in 0.0018 secs
  99.90% in 0.0030 secs
  99.99% in 0.0141 secs


Details (average, fastest, slowest):
  DNS+dialup:   0.0097 secs, 0.0024 secs, 0.0121 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0000 secs

Status code distribution:
  [200] 9988 responses

```

#### 5min /lock

```Bash

oha http://127.0.0.1:8080/lock -z 300s -q 1000 -c 10

Summary:
  Success rate: 100.00%
  Total:        300.0068 secs
  Slowest:      0.0247 secs
  Fastest:      0.0002 secs
  Average:      0.0011 secs
  Requests/sec: 999.9508

  Total data:   1.43 MiB
  Size/request: 5
  Size/sec:     4.88 KiB

Response time histogram:
  0.000 [1]      |
  0.003 [297604] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.005 [2376]   |
  0.008 [0]      |
  0.010 [0]      |
  0.012 [7]      |
  0.015 [1]      |
  0.017 [0]      |
  0.020 [0]      |
  0.022 [0]      |
  0.025 [3]      |

Response time distribution:
  10.00% in 0.0005 secs
  25.00% in 0.0008 secs
  50.00% in 0.0011 secs
  75.00% in 0.0015 secs
  90.00% in 0.0019 secs
  95.00% in 0.0021 secs
  99.00% in 0.0026 secs
  99.90% in 0.0032 secs
  99.99% in 0.0039 secs


Details (average, fastest, slowest):
  DNS+dialup:   0.0090 secs, 0.0031 secs, 0.0111 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0000 secs

Status code distribution:
  [200] 299992 responses

```

#### 5min /nolock

```Bash

oha http://127.0.0.1:8080/nolock -z 300s -q 1000 -c 10

Summary:
  Success rate: 100.00%
  Total:        300.0134 secs
  Slowest:      0.0266 secs
  Fastest:      0.0002 secs
  Average:      0.0011 secs
  Requests/sec: 999.9552

  Total data:   1.43 MiB
  Size/request: 5
  Size/sec:     4.88 KiB

Response time histogram:
  0.000 [1]      |
  0.003 [298889] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.005 [1101]   |
  0.008 [1]      |
  0.011 [0]      |
  0.013 [3]      |
  0.016 [3]      |
  0.019 [0]      |
  0.021 [0]      |
  0.024 [0]      |
  0.027 [2]      |

Response time distribution:
  10.00% in 0.0005 secs
  25.00% in 0.0007 secs
  50.00% in 0.0010 secs
  75.00% in 0.0013 secs
  90.00% in 0.0018 secs
  95.00% in 0.0020 secs
  99.00% in 0.0025 secs
  99.90% in 0.0032 secs
  99.99% in 0.0041 secs


Details (average, fastest, slowest):
  DNS+dialup:   0.0122 secs, 0.0022 secs, 0.0259 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0000 secs

Status code distribution:
  [200] 300000 responses

  ```

  