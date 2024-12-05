use std::{collections::HashMap, fmt::Display};

use crate::Solution;

pub struct Day5;

impl Solution for Day5 {
    fn part_1(input: &str) -> Box<dyn Display> {
        let mut happens_before: HashMap<u8, Vec<u8>> = HashMap::new();
        let mut updates: Vec<Vec<u8>> = vec![];
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            if let Some((l, r)) = line.split_once('|') {
                let l = l.parse().unwrap();
                let r = r.parse().unwrap();
                happens_before.entry(l).or_default().push(r);
                continue;
            }
            updates.push(line.split(',').map(|p| p.parse().unwrap()).collect());
        }
        let sum = updates
            .into_iter()
            .filter(|m| {
                m.iter().enumerate().all(|(i, p)| {
                    happens_before
                        .get(p)
                        .map(|c| m[0..i].iter().all(|pp| !c.contains(pp)))
                        .unwrap_or(true)
                })
            })
            .fold(0u32, |acc, m| acc + m[m.len() / 2] as u32);

        Box::new(sum)
    }

    fn part_2(input: &str) -> Box<dyn Display> {
        let mut happens_before: HashMap<u8, Vec<u8>> = HashMap::new();
        let mut updates: Vec<Vec<u8>> = vec![];
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            if let Some((l, r)) = line.split_once('|') {
                let l = l.parse().unwrap();
                let r = r.parse().unwrap();
                happens_before.entry(l).or_default().push(r);
                continue;
            }
            updates.push(line.split(',').map(|p| p.parse().unwrap()).collect());
        }
        let sum = updates
            .into_iter()
            .filter(|u| {
                u.iter().enumerate().any(|(i, p)| {
                    happens_before
                        .get(p)
                        .map(|c| u[0..i].iter().any(|pp| c.contains(pp)))
                        .unwrap_or(false)
                })
            })
            .fold(0u32, |acc, mut u| {
                u.sort_unstable_by(|a, b| {
                    happens_before
                        .get(a)
                        .map(|i| i.contains(b))
                        .unwrap_or(false)
                        .cmp(&true)
                });
                acc + u[u.len() / 2] as u32
            });

        Box::new(sum)
    }
}

#[test]
fn test() {
    const INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13


75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    assert_eq!(Day5::part_1(INPUT).to_string(), "143");
    assert_eq!(Day5::part_2(INPUT).to_string(), "123");
}
