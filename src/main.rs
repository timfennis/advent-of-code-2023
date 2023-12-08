use crate::puzzle::{Answerable, Puzzle, Solution};
use download::Downloader;

mod download;
mod prelude;
mod puzzle;
mod year2019;
mod year2023;

fn main() -> anyhow::Result<()> {
    let downloader = Downloader::from_env()?;
    let mut day: year2023::Day8 = Default::default();
    // let mut day: year2019::Day5 = Default::default();

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

#[cfg(test)]
pub mod test {
    use crate::download::Downloader;
    use crate::execute;
    use crate::puzzle::{Answerable, Puzzle, Solution};
    use std::fmt::Display;

    pub fn check<T: Solution + Puzzle + Answerable>(
        solution: &mut T,
        expected_part_1: impl Display,
        expected_part_2: impl Display,
    ) {
        let downloader = Downloader::from_env().expect("must be able to construct a downloader");
        execute(solution, &downloader).expect("solution returns OK");
        assert_eq!(
            solution.answer().part_1,
            Some(format!("{}", expected_part_1))
        );
        assert_eq!(
            solution.answer().part_2,
            Some(format!("{}", expected_part_2))
        );
    }
}
