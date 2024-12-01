#!/usr/bin/env bash
set -euxo pipefail

for day in $(seq 2 25);
do
    for part in $(seq 1 2);
    do
    file="use std::fmt::Display;

use advent_of_code_2024::Solution;

pub struct Day${day}Part${part};

impl Solution for Day${day}Part${part} {
    fn solve(input: String) -> Box<dyn Display> {
        todo!(\"day ${day} part ${part} not done!\")
    }
}
"
    echo "$file" > ./day_${day}_${part}.rs
    done;

done;
