use std::cmp::min;
use std::collections::HashSet;

use crate::prelude::StringTools;
use crate::puzzle::Answerable;
use crate::{create_solution, Solution};

create_solution!(Day4, 2023, 4);
impl Solution for Day4 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let mut games: Vec<usize> = Vec::with_capacity(input.lines().count());

        let mut part1_answer = 0;
        for line in input.lines() {
            let (_game, data) = line.split_once(": ").expect("line must contain a ':'");
            let (winning, my_numbers) = data.split_once(" | ").expect("line must contain '|'");

            let winning = winning.nums().into_iter().collect::<HashSet<_>>();
            let my_numbers = my_numbers.nums().into_iter().collect::<HashSet<_>>();

            let win_count = my_numbers.intersection(&winning).count();

            part1_answer += match win_count {
                0 => 0,
                _ => 2u32.pow((win_count - 1) as u32),
            };

            games.push(win_count);
        }

        self.submit_part1(part1_answer);

        assert_eq!(part1_answer, 20117);

        let mut cards = [1usize; 198];

        for (game_id, win_count) in games.into_iter().enumerate() {
            let new_range = (game_id + 1)..=min(game_id + win_count, 198);

            for n in new_range {
                cards[n] += cards[game_id];
            }
        }

        let part2_answer = cards.into_iter().sum::<usize>();

        self.submit_part2(part2_answer);

        assert_eq!(part2_answer, 13768818);

        Ok(())
    }
}
