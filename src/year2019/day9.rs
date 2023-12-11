use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use crate::year2019::IntCodeComputer;
use itertools::Itertools;
create_solution!(Day9, 2019, 9);
impl Solution for Day9 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let program = input
            .split(',')
            .map(|n| n.parse::<i64>().unwrap())
            .collect_vec();

        let mut computer = IntCodeComputer::from(&program);
        let output = computer.run_until_halt(vec![1]);

        self.submit_part1(output.last().unwrap());

        let mut computer = IntCodeComputer::from(&program);
        let output = computer.run_until_halt(vec![2]);

        self.submit_part2(output.last().unwrap());
        Ok(())
    }
}
