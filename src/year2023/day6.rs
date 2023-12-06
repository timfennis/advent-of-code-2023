use crate::create_solution;
use crate::prelude::StringTools;
use crate::puzzle::{Answerable, Solution};
use itertools::Itertools;
create_solution!(Day6, 2023, 6);

impl Solution for Day6 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let mut lines = input.lines();
        let line1 = lines
            .next()
            .expect("file must contain 2 lines: missing first");
        let line2 = lines
            .next()
            .expect("file must contain 2 lines: missing second");

        let p1_input = line1
            .split_ascii_whitespace()
            .dropping(1)
            .map(|num| {
                num.parse::<u64>()
                    .unwrap_or_else(|_| panic!("{} is not a valid int", num))
            })
            .zip(
                line2
                    .split_ascii_whitespace()
                    .dropping(1)
                    .map(|num| num.parse::<u64>().unwrap()),
            );

        let mut p1_answer = 1;
        for (mt, md) in p1_input {
            p1_answer *= count_winning_races(mt, md);
        }

        self.submit_part1(p1_answer);

        let time = line1[10..].replace(' ', "").parse::<u64>().unwrap();
        let dist = line2[10..].replace(' ', "").parse::<u64>().unwrap();

        self.submit_part2(count_winning_races(time, dist));

        Ok(())
    }
}

fn count_winning_races(max_time: u64, max_dist: u64) -> u64 {
    let mut count = 0;
    for hold_for in 0..max_time {
        let go_for = max_time - hold_for;
        let distance = go_for * hold_for;
        if distance > max_dist {
            count += 1;
        }
    }

    count
}
