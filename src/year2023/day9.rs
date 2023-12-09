use crate::create_solution;
use crate::prelude::StringTools;
use crate::puzzle::{Answerable, Solution};
use itertools::Itertools;

create_solution!(Day9, 2023, 9);

impl Solution for Day9 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let mut part_1 = 0;
        let mut part_2 = 0;
        for line in input.lines() {
            let mut series = Vec::new();
            let mut cur = line
                .split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec();

            series.push(cur.clone());

            loop {
                let next = next(&cur);

                series.push(next.clone());
                cur = next;

                if cur.iter().all(|v| *v == 0) {
                    break;
                }
            }

            let mut first = *series.last().unwrap().first().unwrap();
            let mut last = *series.last().unwrap().last().unwrap();

            for (f, l) in series
                .into_iter()
                .map(|s| (*s.first().unwrap(), *s.last().unwrap()))
                .rev()
            {
                last += l;
                first = f - first;
            }

            part_1 += last;
            part_2 += first;
        }

        self.submit_part1(part_1);
        self.submit_part2(part_2);

        Ok(())
    }
}

fn next(series: &[i64]) -> Vec<i64> {
    let next = series
        .iter()
        .tuple_windows()
        .map(|(a, b)| *b - *a)
        .collect_vec();

    next
}
