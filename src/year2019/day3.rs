use crate::create_solution;
use crate::prelude::{Direction, Vec2};
use crate::puzzle::{Answerable, Solution};
use anyhow::anyhow;
use itertools::Itertools;
use std::collections::HashSet;

create_solution!(Day3, 2019, 3);

impl Solution for Day3 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let (w1, w2) = input.split_once('\n').expect("two lines");

        let w1_points = sim(&parse(w1));
        let w2_points = sim(&parse(w2));

        let ans = w1_points
            .intersection(&w2_points)
            .map(|i| i.manhattan_distance(&Vec2 { x: 0, y: 0 }))
            .filter(|i| *i != 0)
            .min()
            .unwrap();
        self.submit_part1(ans);

        let mut fun = Vec::new();
        for intersection in w1_points.intersection(&w2_points) {
            if *intersection == Vec2::origin() {
                continue;
            }
            let d1 = sim2(&parse(w1))
                .iter()
                .enumerate()
                .filter(|(_d, p)| **p == *intersection)
                .min_by_key(|t| t.0)
                .unwrap()
                .0;

            let d2 = sim2(&parse(w2))
                .iter()
                .enumerate()
                .filter(|(_d, p)| **p == *intersection)
                .min_by_key(|t| t.0)
                .unwrap()
                .0;

            println!("{:#?} {} + {} = {}", intersection, d1, d2, d1 + d2);
            fun.push(d1 + d2);
        }

        self.submit_part2(fun.into_iter().min().unwrap());

        Ok(())
    }
}

fn parse(str: &str) -> Vec<(Direction, u32)> {
    let mut num = 0;
    let mut instructions = Vec::new();
    let mut direction = None;
    for c in str.trim().chars() {
        match c {
            c if c.is_ascii_digit() => {
                num *= 10;
                num += c.to_digit(10).unwrap();
            }
            'U' => {
                if let Some(d) = direction {
                    instructions.push((d, num));
                }
                direction = Some(Direction::Up);
                num = 0;
            }
            'D' => {
                if let Some(d) = direction {
                    instructions.push((d, num));
                }
                direction = Some(Direction::Down);
                num = 0;
            }
            'L' => {
                if let Some(d) = direction {
                    instructions.push((d, num));
                }
                direction = Some(Direction::Left);
                num = 0;
            }
            'R' => {
                if let Some(d) = direction {
                    instructions.push((d, num));
                }
                direction = Some(Direction::Right);
                num = 0;
            }
            ',' => {}
            c => panic!("unexpected token '{}'", c),
        }
    }

    instructions.push((direction.unwrap(), num));

    instructions
}

fn sim(instructions: &[(Direction, u32)]) -> HashSet<Vec2> {
    let mut cur = Vec2::origin();
    let mut points = HashSet::new();
    points.insert(cur.clone());

    for (dir, steps) in instructions {
        for _ in 0..(*steps) {
            cur = cur.move_dir(*dir);
            points.insert(cur.clone());
        }
    }

    points
}

fn sim2(instructions: &[(Direction, u32)]) -> Vec<Vec2> {
    let mut points = Vec::new();
    points.push(Vec2::origin());

    for (dir, steps) in instructions {
        for _ in 0..(*steps) {
            points.push(points.last().unwrap().move_dir(*dir));
        }
    }

    points
}

#[cfg(test)]
mod test {
    use crate::year2019::day3::*;

    #[test]
    fn simple() {
        let p = sim(&parse("U5"));

        assert_eq!(
            p,
            HashSet::from_iter(
                vec![
                    (0, 0).into(),
                    (0, 1).into(),
                    (0, 2).into(),
                    (0, 3).into(),
                    (0, 4).into(),
                    (0, 5).into(),
                ]
                .into_iter()
            )
        )
    }
}
