#!/usr/bin/env bash

day="${1?}"
if ! [[ -f .cookie ]]; then
  >&2 echo 'no cookie'
exit 1
fi
cookie="$(cat .cookie)"
echo $cookie
directory="puzzles/$(printf 'day%02d' "$day")"
file="$directory/joshua.input"
mkdir -p "$directory"
curl --cookie "session=$cookie" "https://adventofcode.com/2019/day/$day/input" > "$file"
touch "$directory/joshua.output"

