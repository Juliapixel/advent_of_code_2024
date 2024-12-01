use std::fmt::Display;

use cli::ARGS;

pub mod cli;

pub fn get_input(day: u8) -> String {
    get_input_year_day(2024, day)
}

fn get_input_year_day(year: u32, day: u8) -> String {
    assert!(year >= 2015);
    assert!(day > 0 && day <= 25);

    let mut req = reqwest::blocking::Request::new(
        reqwest::Method::GET,
        format!("https://adventofcode.com/{year}/day/{day}/input")
            .parse()
            .unwrap(),
    );

    req.headers_mut().insert(
        "cookie",
        format!("session={}", &ARGS.session).try_into().unwrap(),
    );

    let resp = reqwest::blocking::Client::new().execute(req);

    resp.unwrap().error_for_status().unwrap().text().unwrap()
}

pub trait Solution {
    fn solve(input: String) -> Box<dyn Display>;
}
