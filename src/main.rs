#![feature(concat_idents)]

use advent_of_code_2024::cli::ARGS;
use advent_of_code_2024::get_input;

mod days;

use paste::paste;

macro_rules! day {
    ($result: ident, $day: literal) => {
        if ARGS.day == $day {
            if ARGS.part == 1 {
                paste! {
                    $result = Some(days::[< day_ $day _1 >]::solve(get_input($day)));
                }
            } else if ARGS.part == 2 {
                paste! {
                    $result = Some(days::[< day_ $day _2 >]::solve(get_input($day)));
                }
            }
        }
    }
}

macro_rules! match_days {
    ($result:ident, $($days:literal),+) => {
        $(day!($result, $days));+
    };
}

fn main() {
    let mut result = None;
    match_days!(result, 1);
    match result {
        Some(res) => {
            println!("result: {res}")
        },
        None => {
            println!("no result provided!")
        }
    }
}
