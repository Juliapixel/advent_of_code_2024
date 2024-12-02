use std::{collections::HashMap, fmt::Display};

use crate::Solution;

#[inline]
fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut left, mut right) = (Vec::with_capacity(1000), Vec::with_capacity(1000));

    input
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
        .for_each(|(l, r)| {
            left.push(l);
            right.push(r)
        });

    (left, right)
}

pub struct Day1;

impl Solution for Day1 {
    fn part_1(input: String) -> Box<dyn Display> {
        let (mut left, mut right) = parse(&input);

        left.sort_unstable();
        right.sort_unstable();

        let total: i32 = left
            .into_iter()
            .zip(right)
            .fold(0, |sum, (l, r)| sum + l.max(r) - l.min(r));

        Box::new(total)
    }

    fn part_2(input: String) -> Box<dyn Display> {
        let (left, right) = parse(&input);

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
