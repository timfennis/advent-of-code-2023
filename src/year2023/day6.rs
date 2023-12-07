use crate::create_solution;
use crate::prelude::StringTools;
use crate::puzzle::{Answerable, Solution};
use itertools::{fold, Itertools};
use std::ops::{Add, Div, Neg, Sub};
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

        let p1_input = line1.nums::<u64>().zip(line2.nums::<u64>());

        let p1_answer = p1_input
            .map(|(time, dist)| solve(time, dist))
            .product::<u64>();

        self.submit_part1(p1_answer);

        let time = line1[10..].replace(' ', "").parse::<u64>().unwrap();
        let dist = line2[10..].replace(' ', "").parse::<u64>().unwrap();

        let p2_answer = solve(time, dist);
        self.submit_part2(p2_answer);

        Ok(())
    }
}

fn solve(max_time: u64, max_dist: u64) -> u64 {
    let max_time = max_time as f64;
    let max_dist = max_dist as f64;

    let upper = (max_time + (max_time.neg().powi(2) - 4.0 * max_dist).sqrt()).div(2.0);
    let lower = (max_time - (max_time.neg().powi(2) - 4.0 * max_dist).sqrt()).div(2.0);
    (upper.ceil() - lower.ceil()) as u64
}
