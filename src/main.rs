use crate::puzzle::{Answerable, Puzzle};
use crate::year2023::{Day1, Day2};
use download::Downloader;

mod download;
mod puzzle;
mod year2023;

fn main() -> anyhow::Result<()> {
    let downloader = Downloader::from_env()?;
    let input = downloader.day(2023, 2)?;

    let mut day: Day2 = Default::default();

    day.handle_input(&input)?;

    println!("Part 1: {}", day.answer().get_part1().unwrap_or("todo"));
    println!("Part 2: {}", day.answer().get_part2().unwrap_or("todo"));

    Ok(())
}
