use crate::create_solution;
use crate::prelude::StringTools;
use crate::puzzle::{Answerable, Solution};
use crate::year2019::run_program;
use itertools::Itertools;
create_solution!(Day5, 2019, 5);

impl Solution for Day5 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let mut program = [3i64, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let output = run_program(&mut program, &[1]);
        assert_eq!(output.len(), 1);

        let codes = input
            .trim()
            .split(',')
            .map(|num| num.parse::<i64>().unwrap())
            .collect_vec();

        let mut p1_state = codes.clone();
        let output = run_program(&mut p1_state, &[1i64; 1]);
        self.submit_part1(output.last().unwrap());

        let mut p2_state = codes.clone();
        let output = run_program(&mut p2_state, &[5i64, 1]);
        assert_eq!(output.len(), 1);
        self.submit_part2(output.first().unwrap());

        Ok(())
    }
}
