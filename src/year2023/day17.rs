use crate::create_solution;
use crate::prelude::{Direction, Grid, Vec2};
use crate::puzzle::{Answerable, Solution};
use ahash::HashMap;
use std::collections::VecDeque;
create_solution!(Day17, 2023, 17);

impl Solution for Day17 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        // println!("{input}");

        // self.submit_part1(solve_part_1(input));
        self.submit_part2(solve_part_2(input));
        Ok(())
    }
}

fn solve_part_2(input: &str) -> usize {
    let grid = Grid::from_string(input, |_| true);
    let mut queue = VecDeque::new();
    queue.push_front((Vec2::origin(), Direction::Right, 0, 0));

    let end: Vec2 = (grid.width() - 1, grid.height() - 1).into();

    let mut seen: HashMap<(Vec2, Direction, usize), usize> = Default::default();

    seen.insert((Vec2::origin(), Direction::Left, 0), 0);
    seen.insert((Vec2::origin(), Direction::Down, 0), 0);
    seen.insert((Vec2::origin(), Direction::Right, 0), 0);
    seen.insert((Vec2::origin(), Direction::Up, 0), 0);

    let mut counter = 0;
    'dfs: while let Some((pos, last_dir, last_dir_steps, heat_loss)) = queue.pop_front() {
        counter += 1;

        if counter % 1000000 == 0 {
            println!("{} {:?} {} {}", pos, last_dir, last_dir_steps, heat_loss);
        }

        'dir: for new_dir in [last_dir.turn_left(), last_dir.turn_right()] {
            // move 4 times now
            let mut new_pos = pos;
            let mut hl: usize = heat_loss;
            for _ in 0..4 {
                new_pos = new_pos.move_dir(new_dir);
                if !grid.in_bound(new_pos) {
                    continue 'dir;
                }
                hl += grid.get_object(&new_pos).unwrap().to_digit(10).unwrap() as usize;
            }

            if *seen
                .get(&(new_pos, new_dir, last_dir_steps))
                .unwrap_or(&usize::MAX)
                > hl
            {
                queue.push_back((new_pos, new_dir, 4, hl));
                seen.insert((new_pos, new_dir, 4), hl);
                if new_pos == end {
                    break 'dfs;
                }
            }
        }

        if last_dir_steps < 10 {
            let new_pos = pos.move_dir(last_dir);

            if !grid.in_bound(new_pos) {
                continue;
            }

            let hl =
                heat_loss + (grid.get_object(&new_pos).unwrap().to_digit(10).unwrap() as usize);

            if *seen
                .get(&(new_pos, last_dir, last_dir_steps))
                .unwrap_or(&usize::MAX)
                > hl
            {
                queue.push_back((new_pos, last_dir, last_dir_steps + 1, hl));
                seen.insert((new_pos, last_dir, last_dir_steps + 1), hl);
                if new_pos == end {
                    break 'dfs;
                }
            }
        }
    }

    seen.iter()
        .filter_map(|((pos, _, _), heat_loss)| (*pos == end).then_some(*heat_loss))
        .min()
        .unwrap()
}
fn solve_part_1(input: &str) -> usize {
    let grid = Grid::from_string(input, |_| true);
    let mut queue = VecDeque::new();
    queue.push_front((Vec2::origin(), Direction::Right, 0, 0));

    let end: Vec2 = (grid.width() - 1, grid.height() - 1).into();

    let mut seen: HashMap<(Vec2, Direction, usize), usize> = Default::default();

    seen.insert((Vec2::origin(), Direction::Left, 0), 0);
    seen.insert((Vec2::origin(), Direction::Down, 0), 0);
    seen.insert((Vec2::origin(), Direction::Right, 0), 0);
    seen.insert((Vec2::origin(), Direction::Up, 0), 0);

    let mut counter = 0;
    while let Some((pos, last_dir, last_dir_steps, heat_loss)) = queue.pop_front() {
        counter += 1;

        if counter % 1000000 == 0 {
            println!("{} {:?} {} {}", pos, last_dir, last_dir_steps, heat_loss);
        }

        for new_dir in [last_dir.turn_left(), last_dir.turn_right()] {
            let new_pos = pos.move_dir(new_dir);

            if !grid.in_bound(new_pos) {
                continue;
            }

            let hl = heat_loss
                + grid
                    .get_object(&new_pos)
                    .expect("must be a heat value at position")
                    .to_digit(10)
                    .expect("heat value must be foobar") as usize;

            if *seen
                .get(&(new_pos, new_dir, last_dir_steps))
                .unwrap_or(&usize::MAX)
                > hl
            {
                queue.push_back((new_pos, new_dir, 1, hl));
                seen.insert((new_pos, new_dir, 1), hl);
            }
        }

        if last_dir_steps < 3 {
            let new_pos = pos.move_dir(last_dir);

            if !grid.in_bound(new_pos) {
                continue;
            }

            let hl =
                heat_loss + (grid.get_object(&new_pos).unwrap().to_digit(10).unwrap() as usize);

            if *seen
                .get(&(new_pos, last_dir, last_dir_steps))
                .unwrap_or(&usize::MAX)
                > hl
            {
                queue.push_back((new_pos, last_dir, last_dir_steps + 1, hl));
                seen.insert((new_pos, last_dir, last_dir_steps + 1), hl);
            }
        }
    }

    seen.iter()
        .filter_map(|((pos, _, _), heat_loss)| (*pos == end).then_some(*heat_loss))
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::year2023::day17::*;

    #[test]
    fn shloobala() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        // assert_eq!(solve_part_1(input), 102);
        assert_eq!(solve_part_2(input), 94);
    }
}
