use crate::create_solution;
use crate::prelude::Vec3;
use crate::puzzle::{Answerable, Solution};
use ahash::{AHashSet, HashSet};
use itertools::Itertools;
use std::thread::current;
create_solution!(Day22, 2023, 22);

impl Solution for Day22 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let (a, b) = solve(input);
        self.submit_part1(a);
        // -- 96384
        assert!(b < 96384); // was too high
        self.submit_part2(b);
        Ok(())
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut bricks = Vec::new();
    for line in input.lines() {
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

        bricks.push((start, end));
    }
    // let og_brick_count = bricks.len();

    dbg!(bricks.len());
    println!("Brick len: {}", bricks.len());
    // Settle the bricks
    let (bricks, _) = settle(bricks, false);

    dbg!(&bricks);

    let mut safe = 0;
    let mut sum = 0;
    for (idx, current_brick) in bricks.iter().enumerate() {
        let new_set = bricks
            .iter()
            .filter(|&b| b != current_brick)
            .copied()
            .collect_vec();

        let (_, changes) = settle(new_set, true);
        println!("{idx} has {changes}");
        sum += changes;
        println!("Subtotal: {sum}");
        if changes == 0 {
            safe += 1;
        }
    }

    (safe, sum)
}
type Brick = (Vec3, Vec3);

fn settle(bricks: Vec<Brick>, quick_abort: bool) -> (Vec<Brick>, usize) {
    let mut current_bricks = bricks;
    let mut total_changes = 0;
    loop {
        let mut new_bricks = Vec::with_capacity(1500);
        let pre = total_changes;
        for current_brick in &current_bricks {
            // Try to drop current_brick
            // we can drop the current brick 1 spot
            // First we check if the brick hasn't already reached the _bottom_
            let mut new_brick = *current_brick;
            let mut brick_changes = 0;
            'drop: loop {
                if new_brick.0.z == 1 || new_brick.1.z == 1 {
                    // continue with the next brick
                    // println!("SETTLED BOTTOM");
                    new_bricks.push(new_brick);
                    break 'drop;
                }

                // We drop the brick 1 slot down
                new_brick.0.z -= 1;
                new_brick.1.z -= 1;
                brick_changes += 1;

                for other_brick in &current_bricks {
                    // don't collide with self
                    if other_brick == current_brick {
                        continue;
                    }

                    if brick_intersect(new_brick, *other_brick) {
                        new_brick.0.z += 1;
                        new_brick.1.z += 1;
                        brick_changes -= 1;
                        // println!("SETTLED ON ANOTHER BRICK {}", total_changes);
                        new_bricks.push(new_brick);
                        break 'drop;
                    }
                }
            }

            if brick_changes > 0 {
                total_changes += 1;
            }
        }

        current_bricks = new_bricks;

        if total_changes == pre {
            return (current_bricks, total_changes);
        } else {
            // println!("AGAIN!!");
        }
    }
}
fn brick_intersect((s1, e1): Brick, (s2, e2): Brick) -> bool {
    for x in s1.x..=e1.x {
        for y in s1.y..=e1.y {
            for z in s1.z..=e1.z {
                for xx in s2.x..=e2.x {
                    for yy in s2.y..=e2.y {
                        for zz in s2.z..=e2.z {
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
fn enum_brick((start, end): (Vec3, Vec3)) -> AHashSet<Vec3> {
    let mut positions: AHashSet<Vec3> = Default::default();
    for x in start.x..=end.x {
        for y in start.y..=end.y {
            for z in start.z..=end.z {
                positions.insert(Vec3 { x, y, z });
            }
        }
    }

    positions
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
