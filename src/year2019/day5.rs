use crate::create_solution;
use crate::prelude::StringTools;
use crate::puzzle::{Answerable, Solution};
use crate::year2019::run_program;
use itertools::Itertools;
create_solution!(Day5, 2019, 5);

impl Solution for Day5 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let program = input
            .trim()
            .split(',')
            .map(|num| num.parse::<i64>().unwrap())
            .collect_vec();

        let output = run_program(&program, vec![1, 1]);
        self.submit_part1(output.last().unwrap());

        let output = run_program(&program, vec![5, 1]);
        self.submit_part2(output.first().unwrap());

        Ok(())
    }
}
