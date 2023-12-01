use download::Downloader;

mod download;
mod year2023;

fn main() -> anyhow::Result<()> {
    let downloader = Downloader::from_env()?;
    let input = downloader.day(2023, 1)?;

    let part_1_output = year2023::Day1::part1(&input)?;

    println!("Part 1: {}", part_1_output);

    let part_2_output = year2023::Day1::part1(&input)?;

    println!("Part 2: {}", part_2_output);

    Ok(())
}

pub trait Puzzle {
    fn part1(input: &str) -> anyhow::Result<String> {
        Ok(String::from("TODO"))
    }
    fn part2(input: &str) -> anyhow::Result<String> {
        Ok(String::from("TODO"))
    }
}
