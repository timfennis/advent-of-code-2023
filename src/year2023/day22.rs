use crate::create_solution;
use crate::prelude::Vec3;
use crate::puzzle::{Answerable, Solution};
use ahash::{AHashSet, HashSet};
use itertools::Itertools;
use std::thread::current;
create_solution!(Day22, 2023, 22);

#[derive(Debug, Eq, PartialEq, Clone)]
struct Brick {
    id: usize,
    start: Vec3,
    end: Vec3,
}

impl Brick {
    fn intersects(&self, other: &Brick) -> bool {
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                for z in self.start.z..=self.end.z {
                    for xx in other.start.x..=other.end.x {
                        for yy in other.start.y..=other.end.y {
                            for zz in other.start.z..=other.end.z {
                                if xx == x && yy == y && zz == z {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }
}

impl Solution for Day22 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let (a, b) = solve(input);
        self.submit_part1(a);
        // -- 96384
        assert!(b < 96384); // was too high
        assert!(b < 71987);
        self.submit_part2(b);
        Ok(())
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut bricks = Vec::new();
    for (idx, line) in input.lines().enumerate() {
        let (start, end) = line.split_once('~').expect("line has to contain ~");
        let start = match start
            .split(',')
            .map(|n| n.parse::<i64>().expect("must be valid number"))
            .collect_vec()
            .as_slice()
        {
            [x, y, z] => Vec3::from(*x, *y, *z),
            _ => panic!("kapot"),
        };
        let end = match end
            .split(',')
            .map(|n| n.parse::<i64>().expect("must be valid number"))
            .collect_vec()
            .as_slice()
        {
            [x, y, z] => Vec3::from(*x, *y, *z),
            _ => panic!("kapot"),
        };

        bricks.push(Brick {
            id: idx + 1,
            start,
            end,
        });
    }

    // Settle the bricks
    let _ = settle(&mut bricks);

    let mut safe = 0;
    let mut sum = 0;
    for current_brick in &bricks {
        let mut new_set = bricks
            .iter()
            .filter(|&b| b.id != current_brick.id)
            .cloned()
            .collect_vec();

        let changed_bricks = settle(&mut new_set);
        sum += changed_bricks;
        if changed_bricks == 0 {
            safe += 1;
        }
    }

    (safe, sum)
}
fn settle(bricks: &mut Vec<Brick>) -> usize {
    let mut changed_bricks: HashSet<usize> = Default::default();
    loop {
        // let bricks_len = bricks.len();
        let mut changes = false;
        for idx in 0..bricks.len() {
            let mut current_brick = bricks.get(idx).unwrap().clone();

            // Try to drop current_brick
            // we can drop the current brick 1 spot
            // First we check if the brick hasn't already reached the _bottom_
            loop {
                if current_brick.start.z == 1 || current_brick.end.z == 1 {
                    break;
                }

                // We drop the brick 1 slot down
                current_brick.start.z -= 1;
                current_brick.end.z -= 1;

                if bricks
                    .iter()
                    .filter(|b| b.id != current_brick.id)
                    .any(|b| b.intersects(&current_brick))
                {
                    // There is a collision making the current mutation invalid
                    break;
                } else {
                    // println!("{} was moved down", current_brick.id);
                    changes = true;
                    bricks[idx] = current_brick.clone();
                    changed_bricks.insert(current_brick.id);
                }
            }
        }

        if !changes {
            return changed_bricks.len();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::year2023::day22::solve;

    #[test]
    fn year_2023_day_22_example() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        assert_eq!(solve(input), (5, 7));
    }
}
