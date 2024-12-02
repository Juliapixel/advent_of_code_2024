use std::{collections::HashMap, fmt::Display};

use advent_of_code_2024::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part_1(input: String) -> Box<dyn Display> {
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

    fn part_2(input: String) -> Box<dyn Display> {
        let (left, right): (Vec<i32>, Vec<i32>) = input
            .lines()
            .map(|l| l.split_once("   ").unwrap())
            .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
            .collect();

        let mut right_count = HashMap::<i32, i32>::new();
        for i in right.into_iter() {
            right_count.entry(i).and_modify(|e| *e += 1).or_insert(1);
        }

        let total: i32 = left
            .into_iter()
            .map(|v| v * right_count.get(&v).copied().unwrap_or_default())
            .sum();

        Box::new(total)
    }
}
