use crate::create_solution;
use crate::prelude::{Grid, Vec2};
use crate::puzzle::{Answerable, Solution};
use ahash::AHashMap;
use itertools::Itertools;
use num::integer::gcd;
create_solution!(Day10, 2019, 10);

impl Solution for Day10 {
    fn handle_input(&mut self, _input: &str) -> anyhow::Result<()> {
        let input = ".#..#
.....
#####
....#
...##";
        println!("{input}");

        let grid = Grid::from_string(input, |c| c == '#');

        let mut result: AHashMap<Vec2, usize> = Default::default();
        for (from, _) in grid.objects.iter() {
            for (to, _) in grid.objects.iter() {
                if from == to {
                    continue;
                }
                let x_diff = to.x - from.x;
                let y_diff = to.y - from.y;

                let gcd = gcd(x_diff, y_diff);
                let x_step = if y_diff == 0 { 1 } else { x_diff / gcd };
                let y_step = if x_diff == 0 { 1 } else { y_diff / gcd };
                dbg!(x_step, y_step);

                let mut x = from.x;
                let mut y = from.y;
                // walk to the other astroid
                let mut obstructed = None;
                loop {
                    x += x_step;
                    y += y_step;

                    if x == to.x || y == to.y {
                        break;
                    }

                    println!("actually doing check!!!");

                    if grid.object_at(x, y).is_some() {
                        obstructed = Some((x, y));
                        break;
                    }
                }

                if obstructed.is_none() {
                    println!("{} to {} is not obstructed", from, to);
                    let count = result.entry(*from).or_insert(0);
                    *count += 1;
                } else {
                    println!(
                        "{} to {} IS obstructed by {},{}",
                        from,
                        to,
                        obstructed.unwrap().0,
                        obstructed.unwrap().1
                    );
                }
            }
        }

        self.submit_part1(result.values().max().unwrap());

        Ok(())
    }
}
