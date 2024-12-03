use std::fmt::Display;

use crate::Solution;

pub struct Day3;

impl Solution for Day3 {
    fn part_1(input: &str) -> Box<dyn Display> {
        let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let sum: i64 = re
            .captures_iter(input)
            .map(|c| c[1].parse::<i64>().unwrap() * c[2].parse::<i64>().unwrap())
            .sum();

        Box::new(sum)
    }

    fn part_2(input: &str) -> Box<dyn Display> {
        let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        let mut donts = input.match_indices("don't()").map(|(i, _)| i);
        let mut dos = input.match_indices("do()").map(|(i, _)| i);

        let mut sum: i64 = 0;
        let mut start = 0;
        let mut last_dont = 0;

        while let Some(next_dont) = donts.find(|d| *d > start) {
            last_dont = next_dont;
            sum += re
                .captures_iter(&input[start..next_dont])
                .map(|c| c[1].parse::<i64>().unwrap() * c[2].parse::<i64>().unwrap())
                .sum::<i64>();

            if let Some(next_start) = dos.find(|d| *d > next_dont) {
                start = next_start;
            } else {
                break
            }
        }

        if last_dont < start {
            sum += re
                .captures_iter(&input[start..])
                .map(|c| c[1].parse::<i64>().unwrap() * c[2].parse::<i64>().unwrap())
                .sum::<i64>();
        }

        Box::new(sum)
    }
}
