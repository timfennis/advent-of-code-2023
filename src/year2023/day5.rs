use crate::create_solution;
use crate::puzzle::Solution;
create_solution!(Day5, 2023, 5);

impl Solution for Day5 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        for line in input.lines() {
            println!("{}", line);
        }
        
        Ok(())
    }
}
