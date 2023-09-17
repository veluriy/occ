#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  # /tmp ディレクトリが存在しなかったら作る
  if [ ! -d tmp ]; then
    mkdir tmp
  fi
  # cargo runコマンドで実行し、出力を.sファイルに
  cargo run $input > tmp/test.s
  cc -o tmp/test tmp/test.s
  tmp/test
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0

echo OK