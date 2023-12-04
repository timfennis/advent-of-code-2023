use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use itertools::Itertools;
create_solution!(Day4, 2019, 4);

impl Solution for Day4 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let (start, end) = input.split_once('-').unwrap();

        let start = start.trim().parse::<usize>().unwrap();
        let end = end.trim().parse::<usize>().unwrap();

        let valid_codes = (start..=end).filter(|num| {
            let s = format!("{num}")
                .chars()
                .map(|it| it.to_digit(10).unwrap())
                .collect_vec();

            if s.iter().tuple_windows().any(|(&a, &b)| b < a) {
                return false;
            }

            s.iter()
                .counts_by(|s| s)
                .iter()
                .any(|(_, count)| *count >= 2)
        });

        self.submit_part1(valid_codes.count());

        let valid_codes = (start..=end).filter(|num| {
            let s = format!("{num}")
                .chars()
                .map(|it| it.to_digit(10).unwrap())
                .collect_vec();

            if s.iter().tuple_windows().any(|(&a, &b)| b < a) {
                return false;
            }

            s.iter()
                .counts_by(|s| s)
                .iter()
                .any(|(_, count)| *count == 2)
        });

        self.submit_part2(valid_codes.count());

        Ok(())
    }
}
