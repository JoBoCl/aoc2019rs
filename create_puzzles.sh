#!/usr/bin/env bash

cd "$(dirname "$0")"

./puzzles.sh $1

day="$(printf '%02d' "$1")"
directory="$(printf 'day%02d' "$1")"
echo "Creating day${day}.rs and setting up BUILD"
cargo new --lib day${day}
cargo add --path day${day}
cd day${day}
cargo add --git 'https://github.com/JoBoCl/solver'
cd ..
cp day00/src/lib.rs day${day}/src/lib.rs
sed -e "s/00/${day}/g" -i day${day}/src/lib.rs

PRE_SOLVERS=$(sed '1,/BEGIN_SOLVER_LIST$/!d' src/main.rs)
SOLVERS=$(sed '/BEGIN_SOLVER_LIST$/,/END_SOLVER_LIST$/!d' src/main.rs \
  | head -n-1 | tail -n+2 \
  | cat - <(echo "$1 => day${day}::Day${day}::try_create(input),") \
  | sort -u)
POST_SOLVERS=$(sed '/END_SOLVER_LIST$/,$!d' src/main.rs)

cat <<<$PRE_SOLVERS>src/main.rs
cat <<<$SOLVERS>>src/main.rs
cat <<<$POST_SOLVERS>>src/main.rs
cargo fmt
cargo build