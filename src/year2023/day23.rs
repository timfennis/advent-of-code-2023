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

        let (p1, p2) = solve(input);
        self.submit_part1(p1);
        self.submit_part2(p2);

        Ok(())
    }
}

fn solve(input: &str) -> (usize, usize) {
    let (p1_graph, start, end) = parse_graph(input, false);
    let p1_ans = dfs(&p1_graph, start, end);

    let (p2_graph, start, end) = parse_graph(input, true);
    let p2_graph = compress(p2_graph);
    let p2_ans = dfs(&p2_graph, start, end);
    // let p2_ans = 0;

    (p1_ans, p2_ans)
}

fn compress(graph: Graph) -> Graph {
    graph
}

fn parse_graph(input: &str, part_2: bool) -> (Graph, Vec2, Vec2) {
    let grid = Grid::from_string(
        input,
        if part_2 {
            |ch| ch == '#'
        } else {
            |ch| ch != '.'
        },
    );

    let mut start = None;
    let mut end = None;

    let end_y = grid.height() as i64 - 1;
    for x in 0..(grid.width() as i64) {
        if grid.object_at(x, 0).is_none() {
            debug_assert_eq!(start, None, "must only have a single start location");
            start = Some(Vec2 { x, y: 0 });
        }

        if grid.object_at(x, end_y).is_none() {
            debug_assert_eq!(end, None, "must only have a single end location");
            end = Some(Vec2 { x, y: end_y });
        }
    }

    let start = start.expect("must have a start");
    let end = end.expect("must have a end");

    #[cfg(debug_assertions)]
    println!("Found start {} and end {}", start, end);

    let mut graph: Graph = Default::default();
    for y in 0..grid.height() as i64 {
        for x in 0..grid.width() as i64 {
            let cur: Vec2 = (x, y).into();
            let all_directions = &Direction::all();
            let directions: &[Direction] = match grid.object_at(x, y) {
                None => all_directions,
                Some('>') => &[Direction::Right],
                Some('<') => &[Direction::Left],
                Some('^') => &[Direction::Up],
                Some('v') => &[Direction::Down],
                Some('#') => &[],
                _ => unreachable!("this can never happen"),
            };

            for dir in directions {
                let next = cur.move_dir(*dir);

                if grid.get_object(&next) != Some('#') && grid.in_bound(next) {
                    graph.entry(cur).or_default().push((next, 1));
                } else {
                    continue;
                }
            }
        }
    }

    (graph, start, end)
}

type Graph = HashMap<Vec2, Vec<(Vec2, usize)>>;

fn dfs(graph: &Graph, start: Vec2, end: Vec2) -> usize {
    type Path = Vec<Vec2>;
    let mut queue: Vec<(Vec2, Path, usize)> = Default::default();
    queue.push((start, vec![], 0));

    let mut best = 0;
    while let Some((cur, path, distance)) = queue.pop() {
        if cur == end {
            best = std::cmp::max(best, distance);
        }

        for (dest, weight) in graph
            .get(&cur)
            .unwrap_or_else(|| panic!("cur {cur} did not have an entry in graph"))
        {
            if path.contains(dest) {
                continue;
            } else {
                let mut new_path = path.clone();
                new_path.push(*dest);
                queue.push((*dest, new_path, distance + weight));
            }
        }
    }

    best
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

    let (p1, p2) = solve(input);
    assert_eq!(p1, 94);
    assert_eq!(p2, 154);
}
