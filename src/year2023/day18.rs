use crate::create_solution;
use crate::prelude::{Direction, Grid, Vec2};
use crate::puzzle::{Answerable, Solution};
use ahash::HashSet;
use itertools::Itertools;
use std::cmp::{max, min};
use std::thread::current;
create_solution!(Day18, 2023, 18);

impl Solution for Day18 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        self.submit_part1(solve_part1(input));
        self.submit_part2(solve_part2(input));

        Ok(())
    }
}

fn solve_part2(input: &str) -> usize {
    let mut pos = Vec2::origin();

    let mut points: Vec<Vec2> = Default::default();
    points.push(pos);

    for line in input.lines() {
        let hex = line
            .split(' ')
            .last()
            .expect("must have parts")
            .strip_suffix(')')
            .unwrap()
            .strip_prefix("(#")
            .unwrap();
        let mut num = 0;

        for c in hex.chars() {
            num *= 16;
            num += c.to_digit(16).expect("must be valid hex") as i64;
        }

        let dir = match num % 16 {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => unreachable!("invalid direction"),
        };
        let num = num / 16;

        pos = pos.move_dir_dist(dir, num);
        points.push(pos);
    }

    points.pop();

    solve_points(&points)
}

fn solve_points(points: &[Vec2]) -> usize {
    let boundary = points
        .iter()
        .circular_tuple_windows()
        .map(|(p1, p2)| p1.manhattan_distance(p2) as usize)
        .sum::<usize>();

    // Pick's Theorem says the interior area is the
    // Why is it +1 instead of -1 ?!?! I'm not 100% sure I get it
    shoelace(points) + (boundary / 2) + 1
}

fn shoelace(points: &[Vec2]) -> usize {
    let mut sum = 0;

    for (a, b) in points.iter().circular_tuple_windows() {
        sum += a.x * b.y - b.x * a.y;
    }

    (sum.abs() / 2) as usize
}
fn solve_part1(input: &str) -> usize {
    let mut pos = Vec2::origin();
    let mut all_pos: Vec<Vec2> = Default::default();
    for line in input.lines() {
        let parts = line.split(' ').collect_vec();
        let dir = match parts[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!("invalid direction"),
        };
        let dist = parts[1].parse::<i64>().unwrap();

        pos = pos.move_dir_dist(dir, dist);
        all_pos.push(pos);
    }

    solve_points(&all_pos)
}

#[cfg(test)]
mod test {
    use crate::prelude::Vec2;
    use crate::year2023::day18::{shoelace, solve_part1, solve_part2};

    #[test]
    fn test_shoelace() {
        let points: Vec<Vec2> = vec![
            (3, 4).into(),
            (5, 11).into(),
            (12, 8).into(),
            (9, 5).into(),
            (5, 6).into(),
        ];
        assert_eq!(shoelace(&points), 30);
    }

    #[test]
    fn my_sanity() {
        assert_eq!(solve_part1("U 3 _\nR 5 _\nD 3 _\nL 5 _"), 24);
    }

    #[test]
    fn year_2023_day_18_example() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        assert_eq!(solve_part1(input), 62);
        assert_eq!(solve_part2(input), 952408144115);
    }
}
