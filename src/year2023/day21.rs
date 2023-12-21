use crate::create_solution;
use crate::prelude::{Direction, Grid, Vec2};
use crate::puzzle::{Answerable, Solution};
use ahash::{HashMap, HashSet};
use itertools::Itertools;
use std::collections::{BTreeSet, VecDeque};
create_solution!(Day21, 2023, 21);

impl Solution for Day21 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        println!("{input}");

        self.submit_part1(solve(input, 64));
        self.submit_part2(solve_part2(input, 458));
        // self.submit_part1(solve_part2(input, 26501365));
        Ok(())
    }
}

fn solve_part2(input: &str, steps: usize) -> usize {
    let grid = Grid::from_string(input, |ch| ch != '.');

    let walls = grid
        .iter_objects()
        .filter_map(|(pos, ch)| (*ch == '#').then_some(pos))
        .collect::<HashSet<_>>();

    let start = *grid
        .iter_objects()
        .filter_map(|(pos, ch)| (*ch == 'S').then_some(pos))
        .next()
        .expect("there has to be one start pos");

    // TODO: calculate 589 using the start position and the dimensions of the grid

    let mut queue: HashSet<Vec2> = Default::default();

    let start = grid.iter_objects().find(|(_, ch)| *ch == 'S').unwrap().0;
    queue.insert(start);

    let mut grid_of_grids: HashMap<Vec2, BTreeSet<Vec2>> = Default::default();

    for steps_taken in 0..720 {
        println!("{steps_taken}");
        // 26 501 365
        let mut next_queue: HashSet<Vec2> = Default::default();
        for pos in queue.iter() {
            // TEMP: lots of duplication with the code below
            {
                let grid_position = Vec2 {
                    x: pos.x.div_euclid(grid.width() as i64),
                    y: pos.y.div_euclid(grid.height() as i64),
                };
                let gg = grid_of_grids
                    .entry(grid_position)
                    .or_insert(Default::default());

                let adjusted = Vec2 {
                    x: pos.x.rem_euclid(grid.width() as i64),
                    y: pos.y.rem_euclid(grid.height() as i64),
                };

                if !walls.contains(&adjusted) {
                    gg.insert(adjusted);
                }
            }

            // Original code
            for nd in [
                Direction::Right,
                Direction::Left,
                Direction::Up,
                Direction::Down,
            ] {
                let np = pos.move_dir(nd);
                let adjusted = Vec2 {
                    x: np.x.rem_euclid(grid.width() as i64),
                    y: np.y.rem_euclid(grid.height() as i64),
                };

                if !walls.contains(&adjusted) {
                    next_queue.insert(np);
                }
            }
        }

        queue = next_queue;
    }

    let possible_configurations = grid_of_grids.values().collect::<HashSet<_>>();
    dbg!(grid_of_grids.values().len());
    dbg!(possible_configurations.len());

    for (a, b) in &grid_of_grids {
        println!("{} {}", a, b.len());
    }

    // assert_eq!(grid_of_grids.values().len(), 21);
    assert_eq!(possible_configurations.len(), 13);

    // If i'm moving n steps to the right
    let n = steps;
    let n_blocks = steps / grid.width();
    let full_blocks = n_blocks - 1;
    let full_block_count = full_blocks.pow(2) + (full_blocks - 1).pow(2);

    0
}

fn solve(input: &str, steps: usize) -> usize {
    let grid = Grid::from_string(input, |ch| ch != '.');

    let walls: HashSet<Vec2> = Default::default();
    // let walls = grid
    //     .iter_objects()
    //     .filter_map(|(pos, ch)| (*ch == '#').then_some(pos))
    //     .collect::<HashSet<_>>();

    let mut queue: HashSet<Vec2> = Default::default();

    let start = grid.iter_objects().find(|(_, ch)| *ch == 'S').unwrap().0;
    queue.insert(start);

    let mut lens = Vec::new();

    for steps_taken in 0..steps {
        if steps_taken % 100_000 == 0 {
            // println!("{steps_taken}");
        }

        // 26 501 365
        let mut next_queue: HashSet<Vec2> = Default::default();
        for pos in queue.iter() {
            for nd in [
                Direction::Right,
                Direction::Left,
                Direction::Up,
                Direction::Down,
            ] {
                let np = pos.move_dir(nd);
                let adjusted = Vec2 {
                    x: np.x.rem_euclid(grid.width() as i64),
                    y: np.y.rem_euclid(grid.height() as i64),
                };
                if !walls.contains(&adjusted) {
                    next_queue.insert(np);
                }
            }
        }

        queue = next_queue;
        lens.push(queue.len());
        let diff = lens
            .iter()
            .tuple_windows()
            .last()
            .map(|(a, b)| b - a)
            .unwrap_or(0);

        let temp_grid = Grid {
            width: 200,
            height: 200,
            objects: queue.iter().map(|p| (*p, 'O')).collect_vec(),
        };
    }

    queue.len()
}

// fn calc_2(f: usize) -> usize {
//
//     (f * 2 + 1)
// }
fn calculate_occupied_square_count(f: usize) -> usize {
    let mut c = 0;
    let mut a = 1;
    let mut d = 1;
    while c < f {
        c += 1;
        d += 2;
        a += d;
    }

    a
}

#[cfg(test)]
mod test {
    use crate::year2023::day21::{calculate_occupied_square_count, solve};

    #[test]
    fn year_2023_day_21_square_count() {
        assert_eq!(calculate_occupied_square_count(1), 4);
        assert_eq!(calculate_occupied_square_count(9), 100);
        assert_eq!(calculate_occupied_square_count(121), 14884);
        assert_eq!(
            calculate_occupied_square_count(26501365),
            702_322_399_865_956
        );
    }
    #[test]
    fn year_2023_day_21_example_brute_force() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

        assert_eq!(solve(input, 6), 16);
        assert_eq!(solve(input, 10), 50);
        // assert_eq!(solve(input, 50), 1594);
        // assert_eq!(solve(input, 100), 6536);
        // assert_eq!(solve(input, 500), 167004);
        // assert_eq!(solve(input, 5000), 16733044);
    }
}
