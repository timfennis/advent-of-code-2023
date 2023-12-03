use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use std::collections::HashMap;

create_solution!(Day2,2023,2);

impl Solution for Day2 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let mut part1_sum = 0;
        let mut part2_sum = 0;

        for line in input.lines() {
            let (game, cubes) = line.split_once(": ").expect("game id followed by colon");
            let (_, id) = game.split_once(' ').expect("Game followed by the id");
            let id = id.parse::<u32>().expect("Game ID to be valid u32");

            let rounds = cubes.trim().split(';');
            let mut possible = true;
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            for round in rounds {
                let mut map: HashMap<String, u32> = Default::default();
                for set in round.split(',') {
                    let (num, color) = set.trim().split_once(' ').unwrap();
                    let entry = map.entry(color.trim().into()).or_insert(0);
                    *entry += num.trim().parse::<u32>().expect("valid number of cubes");
                }

                if map.get("red") >= Some(&min_red) {
                    min_red = *map.get("red").unwrap_or(&0);
                }
                if map.get("blue") >= Some(&min_blue) {
                    min_blue = *map.get("blue").unwrap_or(&0);
                }
                if map.get("green") >= Some(&min_green) {
                    min_green = *map.get("green").unwrap_or(&0);
                }
                if map.get("red") <= Some(&12)
                    && map.get("green") <= Some(&13)
                    && map.get("blue") <= Some(&14)
                {
                } else {
                    possible = false
                }
            }

            if possible {
                part1_sum += id;
            }

            part2_sum += min_red * min_blue * min_green
        }
        self.submit_part1(part1_sum);
        self.submit_part2(part2_sum);

        Ok(())
    }
}
