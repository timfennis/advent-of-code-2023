use crate::puzzle::{Answerable, Puzzle, Solution};
use download::Downloader;

mod download;
mod prelude;
mod puzzle;
mod year2019;
mod year2023;

fn main() -> anyhow::Result<()> {
    let downloader = Downloader::from_env()?;
    let mut day: year2023::Day7 = Default::default();

    execute(&mut day, &downloader)?;

    println!("Part 1: {}", day.answer().get_part1().unwrap_or("todo"));
    println!("Part 2: {}", day.answer().get_part2().unwrap_or("todo"));

    Ok(())
}

fn execute<T: Solution + Puzzle + Answerable>(
    solution: &mut T,
    downloader: &Downloader,
) -> anyhow::Result<()> {
    let input = downloader.day(solution.year(), solution.day())?;
    solution.handle_input(&input)
}
