use crate::Puzzle;
use std::collections::HashMap;

#[derive(Default)]
pub struct Day2 {
    part1_sum: u32,
    part2_sum: u32,
}

impl Puzzle for Day2 {
    fn part1(&mut self, input: &str) -> anyhow::Result<String> {
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
                self.part1_sum += id;
            }

            self.part2_sum += min_red * min_blue * min_green
        }
        Ok(format!("{}", self.part1_sum))
    }

    fn part2(&mut self, _input: &str) -> anyhow::Result<String> {
        Ok(format!("{}", self.part2_sum))
    }
}
