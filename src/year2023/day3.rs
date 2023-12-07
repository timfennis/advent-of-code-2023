use std::collections::HashMap;

use crate::prelude::*;
use crate::puzzle::Answerable;
use crate::{create_solution, Solution};

create_solution!(Day3, 2023, 3);
impl Solution for Day3 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let grid = Grid::from_string(input, |c| c != '.');

        let mut gears: HashMap<(i64, i64), Vec<_>> = HashMap::new();

        let mut sum = 0;
        let mut buf = String::new();

        let mut valid = false;
        let mut star = None;
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    c if c.is_ascii_digit() => {
                        buf.push(c);
                        for Vec2 { x: nx, y: ny } in grid.neighbours8((x, y)) {
                            let neighbour: Option<char> = grid.get_object(&(nx, ny).into());

                            match neighbour {
                                Some('.') => {}
                                Some(c) if c.is_ascii_digit() => {}
                                None => {}
                                Some(c) => {
                                    if c == '*' {
                                        star = Some((ny, nx));
                                    }
                                    //SYMBOL!
                                    valid = true;
                                }
                            }
                        }
                    }
                    _ => {
                        if buf.is_empty() {
                            continue;
                        }
                        // if the character is not a digit we are going to check if the number we just found is valid

                        if !buf.is_empty() && valid {
                            let current_number = buf.parse::<u32>().unwrap();
                            sum += current_number;
                            if let Some(coordinates) = star {
                                gears
                                    .entry(coordinates)
                                    .or_insert(Vec::new())
                                    .push(current_number)
                            }

                            valid = false;
                        }

                        star = None;
                        buf.clear();
                    }
                }
            }
        }

        self.submit_part1(sum);

        let mut p2 = 0;
        for (_, nums) in gears {
            assert!(nums.len() <= 2);
            if nums.len() == 2 {
                let product = nums[0] * nums[1];
                p2 += product;
            }
        }

        self.submit_part2(p2);
        Ok(())
    }
}
