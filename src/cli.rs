use std::sync::LazyLock;

use clap::{Parser, value_parser};

pub static ARGS: LazyLock<Args> = LazyLock::new(|| {
    let _ = dotenvy::dotenv();
    Args::parse()
});

#[derive(Debug, Clone, clap::Parser)]
pub struct Args {
    #[arg(value_parser(value_parser!(u8).range(1..=25)))]
    pub day: u8,
    #[arg(value_parser(value_parser!(u8).range(1..=2)))]
    pub part: u8,
    #[arg(long, env = "AOC_SESSION", hide_env_values(true))]
    pub session: String,
}
