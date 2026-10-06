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

echo "ALL PASS"
