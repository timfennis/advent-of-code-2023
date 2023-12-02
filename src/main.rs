use crate::year2023::Day2;
use download::Downloader;

mod download;
mod year2023;

fn main() -> anyhow::Result<()> {
    let downloader = Downloader::from_env()?;
    let input = downloader.today()?;

    let mut day: Day2 = Default::default();

    println!("Part 1: {}", day.part1(&input)?);
    println!("Part 2: {}", day.part2(&input)?);

    Ok(())
}

pub trait Puzzle {
    fn part1(&mut self, _input: &str) -> anyhow::Result<String> {
        Ok(String::from("TODO"))
    }
    fn part2(&mut self, _input: &str) -> anyhow::Result<String> {
        Ok(String::from("TODO"))
    }
}
