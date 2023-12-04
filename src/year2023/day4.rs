use std::collections::{HashMap, HashSet};
use itertools::Itertools;

use crate::prelude::*;
use crate::puzzle::Answerable;
use crate::{create_solution, Solution};

create_solution!(Day4, 2023, 4);
impl Solution for Day4 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let mut sum = 0;
        for line in input.lines() {
            let (game, data) = line.split_once(": ").unwrap();
            let (winning, my_numbers) = data.split_once(" | ").unwrap();

            // println!("{winning}, {my_numbers}");

            let winning = winning.trim().split(' ').filter(|n| !n.is_empty()).map(|n| n.trim().parse::<u32>().unwrap()).collect::<HashSet<_>>();
            let my_numbers = my_numbers.trim().split(' ').filter(|n| !n.is_empty()).map(|n| {
                // println!("{n}");

                n.trim().parse::<u32>().unwrap()
            }).collect::<HashSet<_>>();

            let mut foo = my_numbers.intersection(&winning).collect_vec().len();
            let  foo2 = my_numbers.intersection(&winning).collect_vec().len();

            let mut score = 0;
            if foo > 0 {
                score = 1;
                foo -= 1;
            }

            while foo > 0 {
                score *= 2;
                foo -= 1;
            }

            println!("{foo2} winning numbers is {score} points");

            sum += score;
        }

        self.submit_part1(sum);

        Ok(())
    }
}
