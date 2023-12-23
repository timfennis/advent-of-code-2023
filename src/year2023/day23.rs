use crate::create_solution;
use crate::prelude::{Direction, Grid, Vec2};
use crate::puzzle::{Answerable, Solution};
use ahash::{HashMap, HashSet};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
create_solution!(Day23, 2023, 23);

impl Solution for Day23 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        // self.submit_part1(solve_part1(input));

        let part2 = herp_derp(input);
        assert_ne!(part2, 6051);
        assert_ne!(part2, 6050);
        assert_ne!(part2, 5815);
        self.submit_part2(part2);

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct QueueEntry {
    current_pos: Vec2,
    path: Vec<Vec2>,
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        let dist = self
            .current_pos
            .manhattan_distance(&(1000, 1000).into())
            .cmp(&other.current_pos.manhattan_distance(&(1000, 1000).into()));

        self.path
            .len()
            .cmp(&other.path.len())
            .then(dist)
            .then(self.current_pos.cmp(&other.current_pos))
            .then(self.path.cmp(&other.path))
    }
}

// fn solve_part2(input: &str) -> usize {
//     let grid = Grid::from_string(input, |ch| ch != '.');
//     let walls = grid
//         .iter_objects()
//         .map(|(pos, _ch)| *pos)
//         .collect::<HashSet<Vec2>>();
//
//     let mut queue = Vec::new();
//     queue.push(Vec2 { x: 1, y: 0 });
//
//     let mut graph: HashMap<Vec2, Vec<Vec2>> = Default::default();
//
//     while let Some(cur) = queue.pop() {
//         let from = cur;
//
//         let mut cur = cur;
//         loop {
//             let mut candidates = Vec::new();
//             for dir in Direction::all() {
//                 let next = cur.move_dir(dir);
//
//                 if walls.contains(&next) {
//                     continue;
//                 }
//
//                 if !grid.in_bound(next) && next.y != grid.height() as i64 {
//                     continue;
//                 }
//
//                 candidates.push(next);
//             }
//
//             if candidates.len() == 1 {
//                 // we continue moving
//                 cur = *candidates.get(0).unwrap();
//             } else {
//                 let entry = graph.entry(&cur).or_insert(Vec::new());
//                 for c in candidates {
//                     entry.push(c);
//                 }
//
//             }
//
//
//         }
//     }
//
//     0
// }
//

fn herp_derp(input: &str) -> usize {
    let grid = Grid::from_string(input, |ch| ch == '#');
    let walls = grid
        .iter_objects()
        .map(|(pos, _ch)| *pos)
        .collect::<HashSet<Vec2>>();

    let start = Vec2 { x: 1, y: 0 };

    let mut queue: VecDeque<QueueEntry> = Default::default();
    queue.push_front(QueueEntry {
        current_pos: start,
        path: vec![start],
    });

    let mut result = Vec::new();
    let mut max = 0usize;
    let mut counter = 0;

    while let Some(QueueEntry {
        current_pos: cur,
        path,
    }) = queue.pop_front()
    {
        counter += 1;

        if counter % 10_000 == 0 {
            println!(
                "QUEUE = {} RESULTS = {} BEST = {}",
                queue.len(),
                result.len(),
                max
            );
        }

        for dir in [
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::Up,
        ] {
            let next = cur.move_dir(dir);

            if next.y == (grid.height as i64) && grid.object_at(next.x, next.y).is_none() {
                // Reached the end
                result.push(path.clone());

                let l = path.len();

                if l > max {
                    max = l;
                    println!("Some result: {}", path.len());
                }
            } else if grid.in_bound(next) && !path.contains(&next) && !walls.contains(&next) {
                // Can continue
                let mut p = path.clone();
                p.push(next);
                queue.push_back(QueueEntry {
                    current_pos: next,
                    path: p,
                });
            }
        }
    }

    result.iter().map(|p| p.len()).max().unwrap() - 1
}
fn solve_part1(input: &str) -> usize {
    let grid = Grid::from_string(input, |ch| ch != '.');

    let start = Vec2 { x: 1, y: 0 };

    // assert_eq!(grid.object_at(1, 0), None);

    let mut queue: BinaryHeap<QueueEntry> = Default::default();
    queue.push(QueueEntry {
        current_pos: start,
        path: vec![start],
    });

    let mut result = Vec::new();
    let mut max = 0usize;
    let mut counter = 0;

    while let Some(QueueEntry {
        current_pos: cur,
        path,
    }) = queue.pop()
    {
        counter += 1;

        for dir in [
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::Up,
        ] {
            let next = cur.move_dir(dir);

            if next.y == (grid.height as i64) && grid.object_at(next.x, next.y).is_none() {
                // Reached the end
                result.push(path.clone());

                let l = path.len();

                if l > max {
                    max = l;
                    println!("Some result: {}", path.len());
                }
                continue;
            }

            if grid.in_bound(next) && !path.contains(&next) {
                let ob = grid.get_object(&next);

                match (ob, dir) {
                    (Some('>'), Direction::Right)
                    | (Some('<'), Direction::Left)
                    | (Some('^'), Direction::Up)
                    | (Some('v'), Direction::Down)
                    | (None, _) => {
                        // Can continue
                        let mut p = path.clone();
                        p.push(next);
                        queue.push(QueueEntry {
                            current_pos: next,
                            path: p,
                        });
                        // seen.insert(next, len + 1);
                    }
                    (Some('#'), _) | (Some(_), _) => {
                        // Can't continue
                    }
                }
            }
        }
    }

    result.iter().map(|p| p.len()).max().unwrap() - 1
}

#[test]
fn year_2023_day_23_example() {
    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    assert_eq!(solve_part1(input), 94);
    assert_eq!(herp_derp(input), 154);
}
