use crate::create_solution;
use crate::puzzle::Solution;
use crate::year2019::IntCodeComputer;
use itertools::Itertools;
create_solution!(Day9, 2019, 9);
impl Solution for Day9 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let program = input
            .split(',')
            .map(|n| n.parse::<i64>().unwrap())
            .collect_vec();

        let _computer = IntCodeComputer::from(program);
        Ok(())
    }
}
