use crate::create_solution;
use crate::prelude::{Direction, Grid, Vec2};
use crate::puzzle::{Answerable, Solution};
use ahash::{HashMap, HashSet};
use cached::instant::Instant;
use itertools::Itertools;

create_solution!(Day14, 2023, 14);

impl Solution for Day14 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        // println!("{input}");

        let grid = Grid::from_string(input, |ch| ch != '.');
        let m = tilt(&grid.clone(), Direction::Up);
        self.submit_part1(calc_load(&m));

        let mut grid = grid;
        let mut num = 0usize;
        let mut cycle: HashMap<i64, Vec<usize>> = Default::default();
        loop {
            for dir in [
                Direction::Up,
                Direction::Left,
                Direction::Down,
                Direction::Right,
            ] {
                grid = tilt(&grid, dir);
            }

            num += 1;

            let load = calc_load(&grid);

            if let Some(seen_before) = cycle.get(&load) {
                // This number 4 has been chosen kind of arbitrarily because it works for my input
                // not sure how to improve this
                if seen_before.len() >= 4 {
                    let diffs = diffs(seen_before);
                    let head = *diffs.first().expect("must exist");

                    if diffs.iter().all(|v| *v == head) {
                        num += ((1000000000 - num) / head) * head;
                    }
                }
            }

            if num == 1000000000 {
                self.submit_part2(load);
                break;
            }

            cycle.entry(load).or_insert(Vec::new()).push(num);
        }

        Ok(())
    }
}

fn diffs(list: &[usize]) -> Vec<usize> {
    let mut out = Vec::new();
    for (a, b) in list.iter().tuples() {
        out.push(b - a)
    }
    out
}

fn tilt(grid: &Grid, direction: Direction) -> Grid {
    let mut new_objects: HashSet<(Vec2, char)> = Default::default();

    let walls = grid
        .iter_objects()
        .filter_map(|(o, ch)| (*ch == '#').then_some(o))
        .collect::<HashSet<_>>();

    // We move the objects in a specific order to ensure that they cannot collide with themselves
    let objects_to_move = grid
        .iter_objects()
        .filter_map(|(pos, ch)| (*ch == 'O').then_some(pos))
        .sorted_by(|&&a, &&b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));

    // If we're moving right or down we have to iterate the objects in reverse order to prevent self collisions
    let objects_to_move: Box<dyn Iterator<Item = _>> =
        if direction == Direction::Down || direction == Direction::Right {
            Box::new(objects_to_move.rev())
        } else {
            Box::new(objects_to_move)
        };

    for o in objects_to_move {
        let mut best = *o;

        loop {
            let new = best.move_dir(direction);

            if walls.contains(&new) || new_objects.contains(&(new, 'O')) || grid.not_in_bound(new) {
                // can't move here
                break;
            } else {
                best = new
            }
        }

        new_objects.insert((best, 'O'));
    }

    Grid {
        width: grid.width(),
        height: grid.height(),
        objects: new_objects
            .into_iter()
            .chain(grid.iter_objects().filter(|(_, ch)| *ch == '#').copied())
            .collect_vec(),
    }
}

fn calc_load(grid: &Grid) -> i64 {
    let mut sum = 0;
    for (o, ch) in grid.iter_objects() {
        if *ch == 'O' {
            sum += grid.height() as i64 - o.y
        }
    }

    sum
}
