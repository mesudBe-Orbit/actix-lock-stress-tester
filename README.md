# Actix Lock Stress Tester

This is a simple Actix web server application designed to stress test the performance of endpoints with and without a lock.

## Overview

The application has two endpoints:

- `/lock`: This endpoint accesses a shared state using a mutex to ensure safe concurrent access.
- `/nolock`: This endpoint accesses the shared state without using a mutex.

The shared state is a HashMap with a single key-value pair: "key" maps to "value".

## Running the Application

1. Make sure you have Rust and Cargo installed.
2. Clone the repository and navigate to the project directory.
3. Run the application with the command `cargo run`.

The application will start running on `http://127.0.0.1:8080`.

## Performing a Stress Test

You can use the `oha` tool to perform a stress test on the application. A `testit.bat` script is included in the repository for this purpose. To run the script, use the following command:

```bash
./testit.bat
```

## Test Results

The following are the results of running the `testit.bat` script, which performs a stress test on both the `/lock` and `/nolock` endpoints using the `oha` tool.

### /lock Endpoint

```bash
oha http://127.0.0.1:8080/lock -z 10s -q 1000 -c 10

Summary:
  Success rate: 100.00%
  Total:        10.0013 secs
  Slowest:      0.0200 secs
  Fastest:      0.0003 secs
  Average:      0.0013 secs
  Requests/sec: 999.8686

  Total data:   48.83 KiB
  Size/request: 5
  Size/sec:     4.88 KiB

Response time histogram:
  0.000 [1]    |
  0.002 [9264] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.004 [703]  |■■
  0.006 [22]   |
  0.008 [2]    |
  0.010 [0]    |
  0.012 [0]    |
  0.014 [0]    |
  0.016 [0]    |
  0.018 [0]    |
  0.020 [8]    |

Response time distribution:
  10.00% in 0.0007 secs
  25.00% in 0.0009 secs
  50.00% in 0.0012 secs
  75.00% in 0.0016 secs
  90.00% in 0.0021 secs
  95.00% in 0.0024 secs
  99.00% in 0.0034 secs
  99.90% in 0.0068 secs
  99.99% in 0.0197 secs


Details (average, fastest, slowest):
  DNS+dialup:   0.0143 secs, 0.0017 secs, 0.0179 secs
  DNS-lookup:   0.0001 secs, 0.0000 secs, 0.0001 secs

Status code distribution:
  [200] 10000 responses
```

### /nolock Endpoint


```bash
oha http://127.0.0.1:8080/nolock -z 10s -q 1000 -c 10

Summary:
  Success rate: 100.00%
  Total:        10.0132 secs
  Slowest:      0.0201 secs
  Fastest:      0.0003 secs
  Average:      0.0013 secs
  Requests/sec: 998.6770

  Total data:   48.83 KiB
  Size/request: 5
  Size/sec:     4.88 KiB

Response time histogram:
  0.000 [1]    |
  0.002 [9271] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.004 [671]  |■■
  0.006 [38]   |
  0.008 [7]    |
  0.010 [3]    |
  0.012 [1]    |
  0.014 [0]    |
  0.016 [0]    |
  0.018 [6]    |
  0.020 [2]    |

Response time distribution:
  10.00% in 0.0006 secs
  25.00% in 0.0009 secs
  50.00% in 0.0012 secs
  75.00% in 0.0016 secs
  90.00% in 0.0021 secs
  95.00% in 0.0025 secs
  99.00% in 0.0036 secs
  99.90% in 0.0086 secs
  99.99% in 0.0181 secs


Details (average, fastest, slowest):
  DNS+dialup:   0.0135 secs, 0.0031 secs, 0.0163 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0001 secs

Status code distribution:
  [200] 10000 responses
```