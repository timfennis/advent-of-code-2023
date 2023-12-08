use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use ahash::AHashMap;
use itertools::Itertools;
use std::collections::HashMap;
create_solution!(Day8, 2023, 8);

impl Solution for Day8 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let (lr, input) = input.split_once("\n\n").unwrap();

        let mut adj: AHashMap<&str, (&str, &str)> = AHashMap::default();
        for line in input.lines() {
            let (from, to) = line.split_once(" = ").unwrap();
            let to = to.trim_start_matches('(').trim_end_matches(')');
            let (a, b) = to.split_once(", ").unwrap();

            adj.insert(from, (a, b));
        }

        self.submit_part1(solve("AAA", lr, &adj));

        let part2_ans = adj
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|start| solve(start, lr, &adj))
            .reduce(num::integer::lcm)
            .expect("the number of solutions must be > 0");

        self.submit_part2(part2_ans);
        Ok(())
    }
}

fn solve(start: &str, lr: &str, adj: &AHashMap<&str, (&str, &str)>) -> u64 {
    let mut cur = start;
    for (steps, ins) in lr.chars().cycle().enumerate() {
        let (nl, nr) = adj
            .get(cur)
            .unwrap_or_else(|| panic!("{cur} must exist in adjacency map"));
        cur = if ins == 'L' { nl } else { nr };

        if cur.ends_with('Z') {
            return steps as u64 + 1;
        }
    }

    unreachable!("the loop can never end");
}
