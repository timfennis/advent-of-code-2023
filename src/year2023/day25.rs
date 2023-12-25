use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use ahash::HashSet;
use itertools::Itertools;
create_solution!(Day25, 2023, 25);

impl Solution for Day25 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let mut edges: HashSet<(&str, &str)> = Default::default();
        for line in input.lines() {
            let (from, to) = line.split_once(": ").unwrap();
            for to in to.split(' ') {
                edges.insert((from, to));
                // println!("{from} -- {to}");
            }
        }

        // Note: I found these edges using Graphviz, I don't even know in which direction they
        // occur in my input so I'm cutting all of them. In the future figure out how to implement
        // the Stoer–Wagner algorithm to find these edges automatically.
        // https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
        edges.remove(&("xnn", "txf"));
        edges.remove(&("txf", "xnn"));

        edges.remove(&("nhg", "jjn"));
        edges.remove(&("jjn", "nhg"));

        edges.remove(&("tmc", "lms"));
        edges.remove(&("lms", "tmc"));

        // I wanted to be super sure the answer is correct
        let ans1 = calculate_graph_size(&edges, "xnn") * calculate_graph_size(&edges, "txf");
        let ans2 = calculate_graph_size(&edges, "nhg") * calculate_graph_size(&edges, "jjn");
        let ans3 = calculate_graph_size(&edges, "tmc") * calculate_graph_size(&edges, "lms");

        assert_eq!(ans1, ans2);
        assert_eq!(ans2, ans3);

        self.submit_part1(ans1);
        self.submit_part2("MERRY XMAS");
        Ok(())
    }
}

fn calculate_graph_size(edges: &HashSet<(&str, &str)>, start: &str) -> usize {
    let mut queue = Vec::new();
    queue.push(start);
    let mut seen: HashSet<&str> = Default::default();

    while let Some(cur) = queue.pop() {
        seen.insert(cur);
        for &(a, b) in edges.iter().filter(|(a, b)| *a == cur || *b == cur) {
            let next = if a == cur { b } else { a };

            if !seen.contains(next) {
                queue.push(next);
            }
        }
    }

    println!("Size for subgraph starting at {start} is {}", seen.len());

    seen.len()
}
