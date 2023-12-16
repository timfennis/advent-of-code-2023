use crate::create_solution;
use crate::prelude::{Direction, Grid, Vec2};
use crate::puzzle::{Answerable, Solution};
use ahash::HashSet;
use itertools::Itertools;
create_solution!(Day16, 2023, 16);

impl Solution for Day16 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        self.submit_part1(solve(input, Vec2::origin(), Direction::Right));
        self.submit_part2(solve_p2(input));

        Ok(())
    }
}

pub fn solve_p2(input: &str) -> usize {
    let grid = Grid::from_string(input, |ch| ch != '.');
    let mut ans = Vec::new();
    for x in 0..grid.width() {
        let x = x as i64;
        ans.push(solve(input, Vec2 { x, y: 0 }, Direction::Down));
        ans.push(solve(
            input,
            Vec2 {
                x,
                y: grid.height() as i64 - 1,
            },
            Direction::Up,
        ));
    }

    for y in 0..grid.height() {
        let y = y as i64;

        ans.push(solve(input, Vec2 { x: 0, y }, Direction::Right));
        ans.push(solve(
            input,
            Vec2 {
                x: grid.width() as i64 - 1,
                y,
            },
            Direction::Left,
        ));
    }

    *ans.iter().max().unwrap()
}

pub fn solve(input: &str, start: Vec2, start_dir: Direction) -> usize {
    let grid = Grid::from_string(input, |ch| ch != '.');

    let mut queue: Vec<(Vec2, Direction)> = Vec::new();
    queue.push((start, start_dir));
    let mut seen: HashSet<(Vec2, Direction)> = Default::default();

    while let Some((pos, dir)) = queue.pop() {
        let obj = grid.get_object(&pos);

        if seen.contains(&(pos, dir)) {
            continue;
        } else {
            seen.insert((pos, dir));
        }

        // println!("TEST: {:?} {:?}", obj, dir);
        match (obj, dir) {
            (Some('\\'), _) => {
                let new_dir = match dir {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Left,
                };

                // println!("{:?}", new_dir);
                let new_pos = pos.move_dir(new_dir);
                if grid.in_bound(new_pos) {
                    queue.push((new_pos, new_dir));
                }
            }

            (Some('/'), _) => {
                let new_dir = match dir {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Right,
                };
                let new_pos = pos.move_dir(new_dir);
                if grid.in_bound(new_pos) {
                    queue.push((new_pos, new_dir));
                }
            }
            (Some('|'), Direction::Right | Direction::Left) => {
                let up = pos.move_dir(Direction::Up);
                if grid.in_bound(up) {
                    queue.push((up, Direction::Up));
                }

                let down = pos.move_dir(Direction::Down);
                if grid.in_bound(down) {
                    queue.push((down, Direction::Down));
                }
            }
            (Some('-'), Direction::Up | Direction::Down) => {
                let left = pos.move_dir(Direction::Left);
                if grid.in_bound(left) {
                    queue.push((left, Direction::Left));
                }

                let right = pos.move_dir(Direction::Right);
                if grid.in_bound(right) {
                    queue.push((right, Direction::Right));
                }
            }
            _ => {
                let new_pos = pos.move_dir(dir);
                if grid.in_bound(new_pos) {
                    queue.push((new_pos, dir));
                }
            }
        }
    }

    seen.iter()
        .map(|(pos, _)| *pos)
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod test {
    use crate::prelude::{Direction, Vec2};
    use crate::year2023::{solve, solve_p2};
    use std::ffi::c_short;

    #[test]
    fn year_2023_day_16_example() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        assert_eq!(solve(input, Vec2::origin(), Direction::Right), 46);
        assert_eq!(solve_p2(input), 51);
    }
}
