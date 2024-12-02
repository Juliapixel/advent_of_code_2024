use std::fmt::Display;

use crate::Solution;

pub struct Day2;

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split_ascii_whitespace().map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect()
}

fn is_safe(vals: impl Iterator<Item = i32>) -> bool {
    let mut is_asc = None;
    let mut prev = None;

    for i in vals {
        if prev.is_none() {
            prev = Some(i);
            continue;
        }

        let dist = (i - prev.unwrap()).abs();
        if !(1..=3).contains(&dist) {
            return false;
        }

        match is_asc {
            Some(is_asc) => {
                if is_asc != (i > prev.unwrap()) {
                    return false;
                }
            },
            None => is_asc = Some(i > prev.unwrap())
        };
        prev = Some(i)
    }
    true
}

impl Solution for Day2 {
    fn part_1(input: &str) -> Box<dyn Display> {
        let reports = parse(input);
        let valids: i32 = reports
            .into_iter()
            .map(|r| is_safe(r.into_iter()) as i32)
            .sum();
        Box::new(valids)
    }

    fn part_2(input: &str) -> Box<dyn Display> {
        let reports = parse(input);
        let valids = reports
            .into_iter()
            .map(|r| {
                is_safe(r.iter().copied()) ||
                (0..r.len()).any(|i| {
                    is_safe(r.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, val)| *val))
                })
            })
            .map(i32::from)
            .sum::<i32>();

        Box::new(valids)
    }
}
