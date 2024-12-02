#!/usr/bin/env bash
set -euxo pipefail

interp() {
    file="use std::fmt::Display;

use crate::Solution;

pub struct Day${1};

impl Solution for Day${1} {
    fn part_1(input: &str) -> Box<dyn Display> {
        todo!(\"day ${1} part 1 not done!\")
    }

    fn part_2(input: &str) -> Box<dyn Display> {
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
