//! Could probably optimize this to be even faster but meh, also too many allocations doid
//!
//! also i like implementing iterators

use std::fmt::Display;

use crate::Solution;

pub struct Day4;

struct VerticalStrIter<'a> {
    lines: core::str::Lines<'a>,
    column: usize,
}

impl<'a> VerticalStrIter<'a> {
    pub fn new(input: &'a str, column: usize) -> Self {
        Self {
            lines: input.lines(),
            column,
        }
    }
}

impl Iterator for VerticalStrIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        char::from_u32(*self.lines.next()?.as_bytes().get(self.column)? as u32)
    }
}

enum Direction {
    LeftToRight,
    RightToLeft
}

struct DiagonalStrIter<'a> {
    lines: core::str::Lines<'a>,
    column: usize,
    direction: Direction,
    done: bool,
}

impl<'a> DiagonalStrIter<'a> {
    pub fn new(input: &'a str, column: usize, start_line: usize, direction: Direction) -> Self {
        let mut lines = input.lines();
        for _ in 0..start_line {
            lines.next();
        }
        Self {
            lines,
            column,
            direction,
            done: false
        }
    }
}

impl Iterator for DiagonalStrIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let next = char::from_u32(*self.lines.next()?.as_bytes().get(self.column)? as u32);
        match self.direction {
            Direction::LeftToRight => self.column += 1,
            Direction::RightToLeft => {
                if let Some(subbed) = self.column.checked_sub(1) {
                    self.column = subbed
                } else {
                    self.done = true;
                }
            },
        };
        next
    }
}

impl Solution for Day4 {
    fn part_1(input: &str) -> Box<dyn Display> {
        let width = input.lines().next().unwrap().chars().count();
        let (height, finds) = input
            .lines()
            .fold(
                (0usize, 0usize),
                |(h, f), l| (h + 1, f + l.matches("XMAS").count() + l.matches("SAMX").count())
            );
        let mut count = finds;

        for i in 0..width {
            let vertical: String = VerticalStrIter::new(input, i).collect();
            count += vertical.matches("XMAS").count();
            count += vertical.matches("SAMX").count();

            // diagonals starting at the top row
            let diagonal_ltr: String = DiagonalStrIter::new(
                input,
                i,
                0,
                Direction::LeftToRight
            ).collect();
            count += diagonal_ltr.matches("XMAS").count();
            count += diagonal_ltr.matches("SAMX").count();

            let diagonal_rtl: String = DiagonalStrIter::new(
                input,
                i,
                0,
                Direction::RightToLeft
            ).collect();
            count += diagonal_rtl.matches("XMAS").count();
            count += diagonal_rtl.matches("SAMX").count();
        }

        // diagonals starting in left and right edges
        for i in 1..height {
            let diagonal_ltr: String = DiagonalStrIter::new(
                input,
                0,
                i,
                Direction::LeftToRight
            ).collect();
            count += diagonal_ltr.matches("XMAS").count();
            count += diagonal_ltr.matches("SAMX").count();

            let diagonal_rtl: String = DiagonalStrIter::new(
                input,
                width-1,
                i,
                Direction::RightToLeft
            ).collect();
            count += diagonal_rtl.matches("XMAS").count();
            count += diagonal_rtl.matches("SAMX").count();
        }

        Box::new(count)
    }

    fn part_2(input: &str) -> Box<dyn Display> {
        let width = input.lines().next().unwrap().chars().count();
        let height = input.lines().count();
        let mut count = 0;


        for i in 0..width {
            let diagonal_ltr: String = DiagonalStrIter::new(input, i, 0, Direction::LeftToRight).collect();
            for (j, _) in diagonal_ltr.match_indices("MAS").chain(diagonal_ltr.match_indices("SAM")) {
                let maybe: String = DiagonalStrIter::new(input, i + j + 2, j, Direction::RightToLeft).take(3).collect();
                if maybe == "MAS" || maybe == "SAM" {
                    count += 1;
                }
            }
        }

        for i in 1..height {
            let diagonal_ltr: String = DiagonalStrIter::new(input, 0, i, Direction::LeftToRight).collect();
            for (j, _) in diagonal_ltr.match_indices("MAS").chain(diagonal_ltr.match_indices("SAM")) {
                let maybe: String = DiagonalStrIter::new(input, j + 2, i + j, Direction::RightToLeft).take(3).collect();
                if maybe == "MAS" || maybe == "SAM" {
                    count += 1;
                }
            }
        }

        Box::new(count)
    }
}

#[cfg(test)]
const TEST_DATA: &str =
r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[test]
fn test() {
    assert_eq!(Day4::part_1(TEST_DATA).to_string(), "18");
    assert_eq!(Day4::part_2(TEST_DATA).to_string(), "9");
}
