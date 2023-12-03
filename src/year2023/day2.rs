use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use std::cmp::max;

create_solution!(Day2, 2023, 2);

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
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for round in rounds {
                for set in round.split(',') {
                    let (num, color) = set.trim().split_once(' ').unwrap();

                    let color = color.trim();
                    let num = num.trim().parse::<u32>().expect("valid number of cubes");

                    match (color, num) {
                        ("red", num) => {
                            if num > 12 {
                                possible = false;
                            }
                            max_red = max(max_red, num);
                        }
                        ("green", num) => {
                            if num > 13 {
                                possible = false;
                            }
                            max_green = max(max_green, num);
                        }
                        ("blue", num) => {
                            if num > 14 {
                                possible = false;
                            }
                            max_blue = max(max_blue, num);
                        }
                        _ => {}
                    }
                }
            }

            if possible {
                part1_sum += id;
            }

            part2_sum += max_red * max_blue * max_green
        }
        self.submit_part1(part1_sum);
        self.submit_part2(part2_sum);

        Ok(())
    }
}
