use itertools::Itertools;
use std::cmp::min;
use std::collections::{HashMap, HashSet};

use crate::puzzle::Answerable;
use crate::{create_solution, Solution};
use crate::prelude::StringTools;

create_solution!(Day4, 2023, 4);
impl Solution for Day4 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let mut part1_answer = 0;

        let mut games: HashMap<u32, u32> = Default::default();
        for line in input.lines() {
            let (game, data) = line.split_once(": ").unwrap();
            let (winning, my_numbers) = data.split_once(" | ").unwrap();
            let (_, id) = game.split_once(' ').unwrap();
            let id = id.trim().parse::<u32>().unwrap();

            let winning = winning
                .nums()
                .into_iter()
                .collect::<HashSet<_>>();

            let my_numbers = my_numbers
                .nums()
                .into_iter()
                .collect::<HashSet<_>>();

            let win_count = my_numbers.intersection(&winning).count() as u32;

            part1_answer += calc_score(win_count);

            games.insert(id, win_count);
        }

        self.submit_part1(part1_answer);

        assert_eq!(part1_answer, 20117);

        let mut buf = (1..=198).collect_vec();

        let mut part2_answer = 0;
        while let Some(game_id) = buf.pop() {
            let win_count = games.get(&game_id).unwrap();

            part2_answer += 1;
            let new_range = (game_id + 1)..=min(game_id + win_count, 198);
            for n in new_range {
                buf.push(n);
            }
        }

        self.submit_part2(part2_answer);

        assert_eq!(part2_answer, 13768818);

        Ok(())
    }
}

fn calc_score(win_count: u32) -> u32 {
    match win_count {
        0 => 0,
        1 => 1,
        _ => 2u32.pow(win_count - 1),
    }
}
