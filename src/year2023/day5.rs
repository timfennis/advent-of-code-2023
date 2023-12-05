use itertools::Itertools;

use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use rayon::prelude::*;

create_solution!(Day5, 2023, 5);

impl Solution for Day5 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let mut groups = input.split("\n\n");

        let seeds = groups
            .next()
            .expect("file must have a first line containing the seeds");
        let (_, seeds) = seeds
            .split_once(": ")
            .expect("first line must contain a colon ':'");

        let seed_nums = seeds
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .expect("seed numbers must be valid 64 bit integers");

        let seed_ranges = seed_nums
            .iter()
            .tuples()
            .map(|(a, b)| *a..(*a + *b))
            .collect_vec();

        let mut mappings: Vec<(String, String, Vec<(_, _)>)> = Default::default();

        for group in groups {
            let mut lines = group.lines();
            let ident = lines.next().unwrap();
            let (from, to) = ident
                .strip_suffix(" map:")
                .unwrap()
                .split_once("-to-")
                .unwrap();

            let mut current_dict: Vec<(_, _)> = Default::default();

            for mapping in lines {
                let mut nums = mapping
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u64>().unwrap());

                let dest_start = nums.next().expect("mapping must have 3 numbers");
                let source_start = nums.next().expect("mapping must have 3 numbers");
                let offset = nums.next().expect("mapping must have 3 numbers");

                let source_range = source_start..(source_start + offset);
                let destination_range = dest_start..(dest_start + offset);
                current_dict.push((source_range, destination_range));
            }

            mappings.push((from.into(), to.into(), current_dict));
        }

        let mut results = Vec::new();

        for seed_num in seed_nums {
            let mut cur_num = seed_num;
            let mut kind = "seed";

            loop {
                let cur_map = mappings
                    .iter()
                    .find(|(from, _, _)| *from == kind)
                    .expect("one mapping must be found");

                if let Some((s, e)) = cur_map.2.iter().find(|(start, _)| start.contains(&cur_num)) {
                    let diff = e.start as i128 - s.start as i128;
                    cur_num = ((cur_num as i128) + diff) as u64;
                }
                kind = &cur_map.1;

                if kind == "location" {
                    results.push(cur_num);
                    break;
                }
            }
        }

        let p1 = results.into_iter().min().unwrap();
        self.submit_part1(p1);

        let mut sr = seed_ranges.clone();
        sr.sort_by(|a, b| a.start.cmp(&b.start));
        // sr.sort_by(|a,b| a.start.cmp(b.start));


        let res = sr.par_iter().map(|range| {
            let mut lowest = u64::MAX;
            println!("---------------------- {:#?} ----------------------", range);
            for seed_num in range.clone() {
                let mut cur_num = seed_num;
                let mut kind = "seed";

                loop {
                    let cur_map = mappings
                        .iter()
                        .find(|(from, _, _)| *from == kind)
                        .expect("one mapping must be found");

                    if let Some((s, e)) = cur_map.2.iter().find(|(start, _)| start.contains(&cur_num)) {
                        let diff = e.start as i128 - s.start as i128;
                        cur_num = ((cur_num as i128) + diff) as u64;
                    }
                    kind = &cur_map.1;

                    if kind == "location" {
                        lowest = std::cmp::min(lowest, cur_num);
                        // println!("New lowest: {lowest}");
                        break;
                    }
                }
            }

            lowest
        }).min().unwrap();

        self.submit_part2(res);


        Ok(())
    }
}
