use crate::puzzle::{Answerable, Solution};
use download::Downloader;

mod download;
mod puzzle;
mod year2023;

fn main() -> anyhow::Result<()> {
    let downloader = Downloader::from_env()?;
    let mut day: year2023::Day3 = Default::default();

    let input = downloader.day(day.year, day.day)?;

    day.handle_input(&input)?;

    println!("Part 1: {}", day.answer().get_part1().unwrap_or("todo"));
    println!("Part 2: {}", day.answer().get_part2().unwrap_or("todo"));

    Ok(())
}
