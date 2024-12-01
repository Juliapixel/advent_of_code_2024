use std::fmt::Display;

use advent_of_code_2024::Solution;

pub struct Day1Part1;

impl Solution for Day1Part1 {
    fn solve(input: String) -> Box<dyn Display> {
        let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
            .lines()
            .map(|l| l.split_once("   ").unwrap())
            .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
            .collect();

        left.sort_unstable();
        right.sort_unstable();

        let total: i32 = left
            .into_iter()
            .zip(right)
            .map(|(l, r)| l.max(r) - l.min(r))
            .sum();

        Box::new(total)
    }
}
