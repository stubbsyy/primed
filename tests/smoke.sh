#!/bin/sh
# primed smoke tests: transpile + compile + run each example
set -e
cd "$(dirname "$0")/.."
BIN=./target/release/primed
cargo build --release

echo "== hello =="
out=$($BIN run examples/hello.pm llm)
[ "$out" = "hi llm" ] || { echo "FAIL hello: $out"; exit 1; }
out=$($BIN run examples/hello.pm)
[ "$out" = "hi world" ] || { echo "FAIL hello default: $out"; exit 1; }

echo "== notes =="
rm -f "$HOME/notes.txt"
$BIN run examples/notes.pm add "buy milk" >/dev/null
$BIN run examples/notes.pm add "second note" >/dev/null
out=$($BIN run examples/notes.pm list)
[ "$out" = "buy milk
second note" ] || { echo "FAIL notes list: $out"; exit 1; }
out=$($BIN run examples/notes.pm bogus)
[ "$out" = "commands: add <text> | list | clear" ] || { echo "FAIL notes usage: $out"; exit 1; }
$BIN run examples/notes.pm clear >/dev/null
out=$($BIN run examples/notes.pm list)
[ "$out" = "no notes" ] || { echo "FAIL notes cleared: $out"; exit 1; }
rm -f "$HOME/notes.txt"

echo "== build =="
$BIN build examples/hello.pm -o /tmp/pm_hello_test
[ "$(/tmp/pm_hello_test built)" = "hi built" ] || { echo "FAIL build"; exit 1; }

echo "== phase2 =="
out=$($BIN run examples/phase2.pm)
echo "$out" | grep -q "#1: buy milk" || { echo "FAIL phase2 structs"; exit 1; }
echo "$out" | grep -q "count=3" || { echo "FAIL phase2 lists"; exit 1; }
echo "$out" | grep -q "rev=cba" || { echo "FAIL phase2 strings"; exit 1; }
echo "$out" | grep -q "sorted: 1,3,5,9" || { echo "FAIL phase2 sort"; exit 1; }

echo "== todo =="
rm -f "$HOME/todo.txt"
$BIN run examples/todo.pm add "smoke task" >/dev/null
$BIN run examples/todo.pm add "second" >/dev/null
$BIN run examples/todo.pm done 1 >/dev/null
out=$($BIN run examples/todo.pm list)
echo "$out" | grep -q "\[x\] #1 smoke task" || { echo "FAIL todo done"; exit 1; }
echo "$out" | grep -q "\[ \] #2 second" || { echo "FAIL todo pending"; exit 1; }
$BIN run examples/todo.pm rm 1 >/dev/null
out=$($BIN run examples/todo.pm list)
echo "$out" | grep -q "#2 second" || { echo "FAIL todo rm"; exit 1; }
rm -f "$HOME/todo.txt"

echo "== phase3 =="
out=$($BIN run examples/journal.pm write "test entry alpha")
echo "$out" | grep -q "saved" || { echo "FAIL journal write"; exit 1; }
out=$($BIN run examples/journal.pm today)
echo "$out" | grep -q "test entry alpha" || { echo "FAIL journal today"; exit 1; }
out=$($BIN run examples/journal.pm find "alpha")
echo "$out" | grep -q "test entry alpha" || { echo "FAIL journal find"; exit 1; }
$BIN run examples/journal.pm clear >/dev/null
out=$($BIN run examples/journal.pm today)
echo "$out" | grep -q "no entry" || { echo "FAIL journal cleared"; exit 1; }
out=$($BIN run examples/timer.pm 0 2>&1 || true)
echo "$out" | grep -q "usage" || { echo "FAIL timer bounds"; exit 1; }
out=$($BIN run examples/timer.pm 999 2>&1 || true)
echo "$out" | grep -q "usage" || { echo "FAIL timer bounds hi"; exit 1; }

echo "== phase4 =="
out=$($BIN cheat)
echo "$out" | grep -q "primed language" || { echo "FAIL cheat sheet"; exit 1; }
out=$($BIN tokens examples/hello.pm)
echo "$out" | grep -q "tokens" || { echo "FAIL tokens report"; exit 1; }
out=$($BIN transpile examples/hello.pm)
echo "$out" | grep -q "fn main" || { echo "FAIL transpile output"; exit 1; }

echo "== phase5 web =="
$BIN build examples/api.pm -o /tmp/pm_api_test >/dev/null 2>&1
/tmp/pm_api_test >/dev/null 2>&1 &
SRV=$!
sleep 1
out=$(python3 - <<'PYCHK'
import socket
try:
    s=socket.create_connection(("127.0.0.1",8091),timeout=2)
    print("skip")
except Exception:
    # server binds 8080; check that
    try:
        s=socket.create_connection(("127.0.0.1",8080),timeout=2)
        s.sendall(b"GET / HTTP/1.1\r\nHost: x\r\n\r\n")
        d=s.recv(4096).decode()
        print("OK" if "hello from primed" in d else "BAD")
    except Exception as e:
        print("CONN:"+str(e))
PYCHK
)
kill $SRV 2>/dev/null
[ "$out" = "OK" ] || { echo "FAIL api server: $out"; exit 1; }
out=$($BIN transpile examples/api.pm)
echo "$out" | grep -q "fn main" || { echo "FAIL api transpile"; exit 1; }

echo "ALL PASS"
