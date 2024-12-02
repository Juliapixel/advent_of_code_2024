use std::fmt::Display;
use std::time::Duration;

use advent_of_code_2024::cli::ARGS;
use advent_of_code_2024::get_input;

use paste::paste;

macro_rules! day {
    ($result: ident, $time:ident, $day: literal) => {
        if ARGS.day == $day {
            let input = get_input($day, &ARGS.session);
            if ARGS.part == 1 {
                let start = ::std::time::Instant::now();
                paste! {
                    let _ = $result.insert(
                        <::advent_of_code_2024::days::[< Day $day >] as ::advent_of_code_2024::Solution>::part_1(&input)
                    );
                }
                let _ = $time.insert(start.elapsed());
            } else if ARGS.part == 2 {
                let start = ::std::time::Instant::now();
                paste! {
                    let _ = $result.insert(
                        <::advent_of_code_2024::days::[< Day $day >] as ::advent_of_code_2024::Solution>::part_2(&input)
                    );
                }
                let _ = $time.insert(start.elapsed());
            }
        }
    }
}

macro_rules! match_days {
    ($result:ident, $time:ident, [$($days:literal),+]) => {
        $(day!($result, $time, $days));+
    };
}

fn main() {
    let mut result: Option<Box<dyn Display>> = None;
    let mut time: Option<Duration> = None;
    match_days!(result, time, [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
    ]);
    match (result, time) {
        (Some(res), Some(time)) => {
            println!("result: {res}");
            println!("took: {:.06}ms", time.as_secs_f64() * 1000.0);
        }
        _ => {
            println!("oh noey, no result returned!");
        }
    }
}
