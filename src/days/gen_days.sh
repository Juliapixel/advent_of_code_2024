#!/usr/bin/env bash
set -euxo pipefail

interp() {
    file="use std::fmt::Display;

use advent_of_code_2024::Solution;

pub struct Day${1};

impl Solution for Day${1} {
    fn part_1(input: String) -> Box<dyn Display> {
        todo!(\"day ${1} part 1 not done!\")
    }

    fn part_2(input: String) -> Box<dyn Display> {
        todo!(\"day ${1} part 2 not done!\")
    }
}
"
    echo "$file"
}


for day in $(seq 2 25);
do
    interp $day > ./day_${day}.rs
done;
