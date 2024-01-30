oha http://127.0.0.1:8080/lock -z 10s -q 1000 -c 10

timeout /t 2 /nobreak

oha http://127.0.0.1:8080/nolock -z 10s -q 1000 -c 10

cmd /k