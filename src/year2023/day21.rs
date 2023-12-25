use crate::create_solution;
use crate::prelude::{Direction, Grid, Vec2};
use crate::puzzle::{Answerable, Solution};
use ahash::{HashMap, HashSet};
use itertools::Itertools;
use std::collections::{BTreeSet, VecDeque};
use std::ops::Add;
create_solution!(Day21, 2023, 21);

impl Solution for Day21 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        println!("{input}");

        self.submit_part1(solve(input, 64));

        let c_327 = solve(input, 327);
        let p2_327 = solve_part2(input, 327, 327);
        assert_eq!(p2_327, c_327);

        let c_458 = solve(input, 327 + 131 + 131);
        let p2_458 = solve_part2(input, 327, 327 + 131 + 131);
        assert_eq!(p2_458, c_458);

        let part_2_ans = solve_part2(input, 131 * 2 + 65, 26501365);
        assert!(part_2_ans < 631366825986889);
        assert!(part_2_ans < 631366825940621);
        self.submit_part2(part_2_ans);
        // self.submit_part1(solve_part2(input, 26501365));
        Ok(())
    }
}

fn solve_part2(input: &str, _probe_len: usize, steps: usize) -> usize {
    let grid = Grid::from_string(input, |ch| ch != '.');

    // let walls: HashSet<Vec2> = Default::default();
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

    assert_eq!(steps % grid.width(), start.x as usize);
    assert_eq!(steps % grid.height(), start.y as usize);
    queue.insert(start);

    let mut grid_of_grids: HashMap<Vec2, BTreeSet<Vec2>> = Default::default();

    for steps_taken in 0..327 {
        println!("{steps_taken}");
        // 26 501 365
        let mut next_queue: HashSet<Vec2> = Default::default();
        for pos in queue.iter() {
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

    // TEMP: lots of duplication with the code below
    for pos in queue {
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

    let possible_configurations = grid_of_grids.values().collect::<HashSet<_>>();
    dbg!(grid_of_grids.values().len());
    dbg!(possible_configurations.len());

    let get = |x: i64, y: i64| {
        grid_of_grids
            .get(&(x, y).into())
            .unwrap_or_else(|| panic!("{} {} has to be filled", x, y))
            .len()
    };

    // for (a, b) in &grid_of_grids {
    //     println!("{} {}", a, b.len());
    // }

    // assert_eq!(grid_of_grids.values().len(), 21);
    assert_eq!(possible_configurations.len(), 14);

    //       X
    //      XXX
    //     XXXXX
    //    XXXXXXX
    //     XXXXX
    //      XXX
    //       X

    //       *T*
    //      *%X%*
    //     *%XXX%*
    //     LXXXXXR
    //     *%XXX%*
    //      *%X%*
    //       *B*
    // If i'm moving n steps to the right
    let _n = steps;
    let total_blocks_per_direction = steps / grid.width();
    let full_blocks_per_direction = total_blocks_per_direction - 1;

    println!("dimension {}x{}", grid.width(), grid.height());
    println!("steps {steps}");
    println!("total_blocks_per_direction {total_blocks_per_direction}");
    println!("full_blocks_per_direction {full_blocks_per_direction}");

    let center_block_count = if total_blocks_per_direction % 2 == 0 {
        (total_blocks_per_direction - 1).pow(2)
    } else {
        (total_blocks_per_direction).pow(2)
    };

    let odd_block_count = if total_blocks_per_direction % 2 == 0 {
        (total_blocks_per_direction).pow(2)
    } else {
        (total_blocks_per_direction - 1).pow(2)
    };

    println!("center_block_count {center_block_count}");
    println!("odd_block_count {odd_block_count}");
    dbg!(
        // Middle part
        center_block_count * get(0, 0),
        odd_block_count * get(0, 1),
        // Top and stuff
        get(-2, 0),
        get(2, 0),
        get(0, 2),
        get(0, -2),
        // Small parts
        total_blocks_per_direction * get(-2, -1),
        total_blocks_per_direction * get(2, -1),
        total_blocks_per_direction * get(-2, 1),
        total_blocks_per_direction * get(2, 1),
        // Big parts
        full_blocks_per_direction * get(-1, -1),
        full_blocks_per_direction * get(-1, 1),
        full_blocks_per_direction * get(1, -1),
        full_blocks_per_direction * get(1, 1),
    );
    let parts = [
        // Middle part
        (center_block_count * get(0, 0)),
        (odd_block_count * get(0, 1)),
        // Top and stuff
        get(-2, 0),
        get(2, 0),
        get(0, 2),
        get(0, -2),
        // Small parts
        total_blocks_per_direction * get(-2, -1),
        total_blocks_per_direction * get(2, -1),
        total_blocks_per_direction * get(-2, 1),
        total_blocks_per_direction * get(2, 1),
        // Big parts
        full_blocks_per_direction * get(-1, -1),
        full_blocks_per_direction * get(-1, 1),
        full_blocks_per_direction * get(1, -1),
        full_blocks_per_direction * get(1, 1),
    ]
    .iter()
    .sum();
    parts
}

fn solve(input: &str, steps: usize) -> usize {
    let grid = Grid::from_string(input, |ch| ch != '.');

    let walls = grid
        .iter_objects()
        .filter_map(|(pos, ch)| (*ch == '#').then_some(pos))
        .collect::<HashSet<_>>();
    // let walls: HashSet<Vec2> = Default::default();

    let mut queue: HashSet<Vec2> = Default::default();

    let start = grid.iter_objects().find(|(_, ch)| *ch == 'S').unwrap().0;
    queue.insert(start);

    let mut lens = Vec::new();

    for _steps_taken in 0..steps {
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
    }

    queue.len()
}

#[cfg(test)]
mod test {
    use crate::year2023::day21::{solve, solve_part2};

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
        assert_eq!(solve_part2(input, 11 * 2 + 5, 5000), 16733044);
        // assert_eq!(solve(input, 50), 1594);
        // assert_eq!(solve(input, 100), 6536);
        // assert_eq!(solve(input, 500), 167004);
        // assert_eq!(solve(input, 5000), 16733044);
    }
}
