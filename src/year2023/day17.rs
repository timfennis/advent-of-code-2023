use crate::create_solution;
use crate::prelude::{Direction, Grid, Vec2};
use crate::puzzle::{Answerable, Solution};
use ahash::{AHashMap, HashMap};
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, VecDeque};
create_solution!(Day17, 2023, 17);

impl Solution for Day17 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        self.submit_part1(find_shortest_path(input, 1, 3));
        self.submit_part2(find_shortest_path(input, 4, 10));

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct BfsEntry {
    pos: Vec2,
    direction: Direction,
    heat_loss: usize,
}

impl PartialOrd<Self> for BfsEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BfsEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        //TODO this is hardcoded to be some point far away from start
        let self_dist = self.pos.manhattan_distance(&(150, 150).into());
        let other_dist = other.pos.manhattan_distance(&(150, 150).into());

        self.heat_loss
            .cmp(&other.heat_loss)
            .then(self_dist.cmp(&other_dist))
            .then(self.direction.cmp(&other.direction))
            .then(self.pos.cmp(&other.pos))
    }
}
fn find_shortest_path(input: &str, min_travel: usize, max_travel: usize) -> usize {
    assert!(min_travel <= max_travel);
    let grid = Grid::from_string(input, |_| true);
    let mut queue: BTreeSet<BfsEntry> = Default::default();

    queue.insert(BfsEntry {
        pos: Vec2::origin(),
        direction: Direction::Right,
        heat_loss: 0,
    });
    queue.insert(BfsEntry {
        pos: Vec2::origin(),
        direction: Direction::Down,
        heat_loss: 0,
    });

    let end: Vec2 = (grid.width() - 1, grid.height() - 1).into();

    let mut seen: AHashMap<(Vec2, Direction), usize> = Default::default();

    while let Some(BfsEntry {
        pos,
        direction: last_dir,
        heat_loss,
    }) = queue.pop_first()
    {
        'dir: for new_dir in [last_dir.turn_left(), last_dir.turn_right()] {
            let mut new_pos = pos;
            let mut hl = heat_loss;

            'offset: for offset in 1..=max_travel {
                new_pos = new_pos.move_dir(new_dir);

                if !grid.in_bound(new_pos) {
                    continue 'dir;
                }

                hl += grid.get_object(&new_pos).unwrap().to_digit(10).unwrap() as usize;

                if offset < min_travel {
                    continue 'offset;
                }

                let best_heat_level_at_location =
                    *seen.get(&(new_pos, new_dir)).unwrap_or(&usize::MAX);

                if hl < best_heat_level_at_location {
                    queue.insert(BfsEntry {
                        pos: new_pos,
                        direction: new_dir,
                        heat_loss: hl,
                    });
                    seen.insert((new_pos, new_dir), hl);
                }
            }
        }
    }

    seen.iter()
        .filter_map(|((pos, _), heat_loss)| (*pos == end).then_some(*heat_loss))
        .min()
        .unwrap()
}
#[cfg(test)]
mod test {
    use crate::year2023::day17::*;

    #[test]
    fn year_2023_day_17_example() {
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
        assert_eq!(find_shortest_path(input, 1, 3), 102);
        assert_eq!(find_shortest_path(input, 4, 10), 94);
    }
}
