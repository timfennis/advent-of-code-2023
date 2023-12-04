use crate::create_solution;
use crate::puzzle::{Answerable, Solution};

create_solution!(Day1, 2019, 1);

impl Solution for Day1 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let sum: i32 = input
            .lines()
            .map(|it| it.parse::<i32>().expect("valid number"))
            .map(calc_p1)
            .sum();

        self.submit_part1(sum);

        let sum: i32 = input
            .lines()
            .map(|it| it.parse::<i32>().expect("valid number"))
            .map(calc_p2)
            .sum();

        self.submit_part2(sum);
        Ok(())
    }
}

fn calc_p1(n: i32) -> i32 {
    (n / 3) - 2
}
fn calc_p2(n: i32) -> i32 {
    let u = calc_p1(n);
    if u > 0 {
        u + calc_p2(u)
    } else {
        0
    }
}
