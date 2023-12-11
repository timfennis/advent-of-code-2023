use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use ahash::{HashMap, HashSet};
use std::cmp::Ordering;
create_solution!(Day6, 2019, 6);

impl Solution for Day6 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let mut adj: HashMap<&str, Vec<&str>> = Default::default();

        let mut map: HashMap<&str, usize> = Default::default();
        map.insert("COM", 0);

        loop {
            let mut some_fail = false;
            for line in input.lines() {
                let (b, a) = line.split_once(')').expect("line must contain )");

                if let Some(dist) = map.get(b) {
                    map.insert(a, dist + 1);
                } else {
                    some_fail = true;
                }

                adj.entry(a).or_insert(Vec::new()).push(b);
                adj.entry(b).or_insert(Vec::new()).push(a);
            }

            if !some_fail {
                self.submit_part1(map.values().sum::<usize>());
                break;
            }
        }

        let mut seen: HashSet<&str> = Default::default();
        let mut buf = Vec::new();
        buf.push(("YOU", 0));

        while let Some((pos, dist)) = buf.pop() {
            if pos == "SAN" {
                self.submit_part2(dist - 2);
            }

            for &next in adj.get(pos).expect("must have adjacent things") {
                if !seen.contains(next) {
                    seen.insert(next);
                    buf.push((next, dist + 1));
                    println!("{} connects to {}", pos, next);
                }
            }
        }

        Ok(())
    }
}
