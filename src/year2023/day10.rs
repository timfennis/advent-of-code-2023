use crate::create_solution;
use crate::prelude::{Direction, Grid, Vec2};
use crate::puzzle::{Answerable, Solution};
use ahash::{AHashMap, AHashSet, HashSet};
use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::path::Component::ParentDir;
create_solution!(Day10, 2023, 10);

#[derive(Debug, Clone, Copy)]
enum Side {
    A,
    B,
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Side::A => 'A',
                Side::B => 'B',
            }
        )
    }
}

impl Side {
    fn invert(&self) -> Self {
        match self {
            Side::A => Side::B,
            Side::B => Side::A,
        }
    }
}
impl Solution for Day10 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let grid = Grid::from_string(input, |c| c != '.');

        let mut seen: AHashMap<Vec2, u32> = Default::default();
        let mut queue = VecDeque::new();

        for (pos, char) in grid {
            if char == 'S' {
                seen.insert(pos, 0u32);
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
                        && *seen.get(&next_pos).unwrap_or(&u32::MAX) > (cur_dist + 1u32)
                    {
                        // dbg!(&queue.len(), seen.len());
                        queue.push_back((next_pos, cur_dist + 1));
                        seen.insert(next_pos, cur_dist + 1);
                    }
                }
            }
        }

        self.submit_part1(seen.values().max().unwrap());

        // Floodfill
        let mut seen_part2: AHashSet<Vec2> = Default::default();
        let mut outside: AHashSet<Vec2> = Default::default();
        let cur = Vec2::origin();
        let mut buf: VecDeque<(Vec2, Option<Side>)> = Default::default();

        outside.insert(cur);
        buf.push_front((cur, None));

        while let Some((pos, side)) = buf.pop_front() {
            for dir in directions {
                let next = pos.move_dir(dir);

                // if the next position is out of bounds we just skip
                if next.x < 0
                    || next.x >= grid.width() as i64
                    || next.y < 0
                    || next.y >= grid.height() as i64
                {
                    continue;
                }

                if outside.contains(&next) {
                    continue;
                }

                // this means we're bumping into part of the loop, whether we can continue walking
                // in this direction depends on the character we came from and the character we're going to
                // We need to know which part of the loop we came from to figure out what side of the loop we'll be on next
                let from = if seen.contains_key(&pos) {
                    grid.get_object(&pos)
                } else {
                    None
                };

                let to = if seen.contains_key(&next) {
                    grid.get_object(&next)
                } else {
                    None
                };

                match (from, to) {
                    (None, Some(to)) => {
                        let new_side = match (dir, to) {
                            // if we're moving right you always end up on the left side of an object
                            (Direction::Right, 'F') => Side::A,
                            (Direction::Right, '|') => Side::A,
                            (Direction::Right, 'L') => Side::A,

                            (Direction::Left, '7') => Side::A,
                            (Direction::Left, '|') => Side::B,
                            (Direction::Left, 'J') => Side::A,

                            (Direction::Up, 'L') => Side::A,
                            (Direction::Up, '-') => Side::B,
                            (Direction::Up, 'J') => Side::A,

                            (Direction::Down, 'F') => Side::A,
                            (Direction::Down, '-') => Side::A,
                            (Direction::Down, '7') => Side::A,
                            _ => panic!("going in direction {:#?} to a {:#?}", dir, to),
                        };

                        // We continue searching along this side of the loop
                        buf.push_back((next, Some(new_side)));
                    }

                    // If we've moving from nothing to nothing we're part of outside
                    (None, None) => {
                        assert!(!seen.contains_key(&next));
                        outside.insert(next);
                        buf.push_back((next, None));
                    }

                    // Figure out if we can exit the loop
                    (Some(from), None) => {
                        let legal = matches!(
                            (dir, from, side.expect("we must have a side here")),
                            (Direction::Left, '|', Side::A)
                                | (Direction::Right, '|', Side::B)
                                | (Direction::Up, '7', Side::A)
                                | (Direction::Right, '7', Side::A)
                                | (Direction::Right, 'J', Side::A)
                                | (Direction::Down, '7', Side::A)
                                | (Direction::Up, '-', Side::A)
                                | (Direction::Down, '-', Side::B)
                                | (Direction::Left, 'L', Side::A)
                                | (Direction::Down, 'L', Side::A)
                                | (Direction::Up, 'F', Side::A)
                                | (Direction::Left, 'F', Side::A)
                        );
                        if legal {
                            outside.insert(next);
                            buf.push_back((next, None));
                        }
                        // figure out if we're on the correct side to exit the loop in this direction
                    }
                    (Some(from), Some(to)) => {
                        // ???? use part 1 logic to figure out if we can keep walking (and update the side we're on)

                        if !is_connected(from, to, dir) {
                            continue;
                        }

                        let new_side = match (from, to, side.expect("there must be a side here")) {
                            ('7', '|', side) => side.invert(),
                            ('|', '7', side) => side.invert(),

                            ('|', 'J', side) => side.invert(),
                            ('J', '|', side) => side.invert(),

                            ('-', 'J', side) => side.invert(),
                            ('J', '-', side) => side.invert(),

                            ('-', 'L', side) => side.invert(),
                            ('L', '-', side) => side.invert(),

                            ('F', 'J', side) => side.invert(),
                            ('J', 'F', side) => side.invert(),

                            ('L', '7', side) => side.invert(),
                            ('7', 'L', side) => side.invert(),
                            (_, _, side) => side,
                        };

                        if !seen_part2.contains(&next) {
                            seen_part2.insert(next);
                            buf.push_back((next, Some(new_side)));
                        }
                    }
                }
            }
        }

        self.submit_part2(grid.width() * grid.height() - outside.len() - seen.len());

        #[cfg(debug_assertions)]
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let pos = Vec2 {
                    x: x as i64,
                    y: y as i64,
                };
                if outside.contains(&pos) {
                    print!(".");
                } else if seen.contains_key(&pos) {
                    print!(
                        "{}",
                        match grid.get_object(&pos).expect("cannot be none here") {
                            '-' => '─',
                            '|' => '│',
                            '7' => '┐',
                            'F' => '┌',
                            'L' => '└',
                            'J' => '┘',
                            c => c,
                        }
                    );
                } else {
                    print!("X");
                }
            }

            println!();
        }
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
