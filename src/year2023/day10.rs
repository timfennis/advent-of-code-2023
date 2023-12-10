use crate::create_solution;
use crate::prelude::{Direction, Grid, Vec2};
use crate::puzzle::{Answerable, Solution};
use ahash::{AHashMap, AHashSet, HashSet};
use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::path::Component::ParentDir;
create_solution!(Day10, 2023, 10);

impl Solution for Day10 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let grid = Grid::from_string(input, |c| c != '.');

        let mut loop_to_dist: AHashMap<Vec2, u32> = Default::default();
        let mut queue = VecDeque::new();

        for (pos, char) in grid {
            if char == 'S' {
                loop_to_dist.insert(pos, 0u32);
                queue.push_front((pos, 0u32));
            }
        }

        // TODO: handle the general case by NOT replacing the S with | which is only correct for my input
        let grid = Grid::from_string(&input.replace('S', "|"), |c| c != '.');

        let directions = [
            Direction::Up,
            Direction::Left,
            Direction::Right,
            Direction::Down,
        ];

        while let Some((cur_pos, cur_dist)) = queue.pop_front() {
            for dir in directions {
                let next_pos = cur_pos.move_dir(dir);

                // Must be connected at all
                if let Some((me, next_char)) = grid
                    .get_object(&cur_pos)
                    .and_then(|c| grid.get_object(&next_pos).map(|n| (c, n)))
                {
                    if (is_connected(me, next_char, dir))
                        && *loop_to_dist.get(&next_pos).unwrap_or(&u32::MAX) > (cur_dist + 1u32)
                    {
                        // dbg!(&queue.len(), seen.len());
                        queue.push_back((next_pos, cur_dist + 1));
                        loop_to_dist.insert(next_pos, cur_dist + 1);
                    }
                }
            }
        }

        #[cfg(debug_assertions)]
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let pos = Vec2 {
                    x: x as i64,
                    y: y as i64,
                };
                if loop_to_dist.contains_key(&pos) {
                    match grid.get_object(&pos) {
                        None => {}
                        Some('F') => {
                            print!("┌");
                        }
                        Some('L') => {
                            print!("└");
                        }
                        Some('J') => {
                            print!("┘");
                        }
                        Some('7') => {
                            print!("┐");
                        }
                        Some(s) => {
                            print!("{}", s);
                        }
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }

        self.submit_part1(loop_to_dist.values().max().unwrap());

        let mut massive_grid: AHashMap<Vec2, char> = AHashMap::default();

        let shapes = AHashMap::from([
            ('7', "___\n##_\n_#_"),
            ('7', "___\n##_\n_#_"),
            ('J', "_#_\n##_\n___"),
            ('L', "_#_\n_##\n___"),
            ('F', "___\n_##\n_#_"),
            ('|', "_#_\n_#_\n_#_"),
            ('-', "___\n###\n___"),
        ]);

        for (point, _) in &loop_to_dist {
            let shape = *grid
                .get_object(point)
                .and_then(|char| shapes.get(&char))
                .expect("must have a char that's mapped to a shape");

            for (shape_pos, char) in Grid::from_string(shape, |_| true).objects {
                massive_grid.insert(
                    (point.x * 3 + shape_pos.x, point.y * 3 + shape_pos.y).into(),
                    char,
                );
            }
        }

        let start = Vec2::origin();
        let mut seen: AHashSet<Vec2> = Default::default();
        let mut queue = VecDeque::new();
        let mut empty_count = 1;

        seen.insert(start);
        queue.push_front(start);

        while let Some(cur) = queue.pop_front() {
            for dir in directions {
                let next = cur.move_dir(dir);

                if next.x >= (grid.width() as i64 * 3)
                    || next.x < 0
                    || next.y >= (grid.height() as i64 * 3)
                    || next.y < 0
                {
                    continue;
                }
                if seen.contains(&next) {
                    continue;
                }

                match massive_grid.get(&next).unwrap_or(&'.') {
                    '.' => {
                        empty_count += 1;
                        queue.push_front(next);
                        seen.insert(next);
                    }
                    '_' => {
                        queue.push_front(next);
                        seen.insert(next);
                    }
                    '#' => {
                        // we can't go here
                    }
                    _ => unreachable!("value cannot exist in grid"),
                }
            }
        }

        assert_eq!(empty_count % 9, 0);

        self.submit_part2(grid.height() * grid.width() - loop_to_dist.len() - (empty_count / 9));

        Ok(())
    }
}

fn is_connected(from: char, to: char, dir: Direction) -> bool {
    let connects_to_right = ['-', 'L', 'F'];
    let connects_to_up = ['|', 'L', 'J'];
    let connects_to_down = ['|', 'F', '7'];
    let connects_to_left = ['-', 'J', '7'];

    match dir {
        Direction::Right => connects_to_right.contains(&from) && connects_to_left.contains(&to),
        Direction::Down => connects_to_down.contains(&from) && connects_to_up.contains(&to),
        Direction::Left => connects_to_left.contains(&from) && connects_to_right.contains(&to),
        Direction::Up => connects_to_up.contains(&from) && connects_to_down.contains(&to),
    }
}
