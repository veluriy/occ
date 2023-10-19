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


assert 0 "3-3;"
assert 4 "3+4-3;"
assert 18 "3*(1+5);"
assert 0 "(3*(2+1)-(4+5));"
assert 3 "3;"
assert 3 "+3;"
assert 0 "-- -4+4;"
assert 1 "(-3)-(-4);"
assert 0 "3>4;"
assert 1 "4<=5;"
assert 1 "(1+2+3+4)>(1+2+3);"
assert 1 "1>2==0;"
assert 1 "1+2+3+4>1+2+3;"
assert 0 "1+2+3+4!=4+3+2+1;"
assert 0 "a=0;"
assert 5 "a=1;a+4;"
assert 5 "a=1;b=4;a+b;"

echo OK