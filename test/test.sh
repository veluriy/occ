#!/bin/bash

INPUTC="./test/input.c"
OUTPUTS="./test/output.s"
OUTPUT="./test/output"
OCC="./target/debug/occ"

cargo build

assert() {
  expected="$1"
  input="$2"

  echo $input > $INPUTC

  # cargo runコマンドで実行し、出力を.sファイルに

  $OCC $INPUTC $OUTPUTS
  gcc $OUTPUTS -o $OUTPUT

  $OUTPUT
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
assert 0 "-4+4;"
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
assert 0 "a=1<2;a==4;"
assert 0 "ab=11;return 0;b=ab;return 1;"
assert 0 "i=0;return i;"
assert 22 "a=11;b=a;c=b+a;c;"
assert 1 "sum=41;i=0;sum == i + 41;"
assert 0 "if 3 == 3 return 0; else return 222;"
assert 10 "sum = 0;for(i = 0; i < 5; i = i + 1) sum = sum + i; return sum;"


echo OK