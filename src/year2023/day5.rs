use crate::create_solution;
use crate::prelude::StringTools;
use crate::puzzle::{Answerable, Solution};
use itertools::Itertools;
use std::collections::HashMap;
use std::env::current_dir;
use std::ops::Range;
use std::thread::current;
create_solution!(Day5, 2023, 5);

impl Solution for Day5 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        //         let input = "seeds: 79 14 55 13
        //
        // seed-to-soil map:
        // 50 98 2
        // 52 50 48
        //
        // soil-to-fertilizer map:
        // 0 15 37
        // 37 52 2
        // 39 0 15
        //
        // fertilizer-to-water map:
        // 49 53 8
        // 0 11 42
        // 42 0 7
        // 57 7 4
        //
        // water-to-light map:
        // 88 18 7
        // 18 25 70
        //
        // light-to-temperature map:
        // 45 77 23
        // 81 45 19
        // 68 64 13
        //
        // temperature-to-humidity map:
        // 0 69 1
        // 1 0 69
        //
        // humidity-to-location map:
        // 60 56 37
        // 56 93 4";
        let mut groups = input.split("\n\n").into_iter();

        let seeds = groups.next().unwrap();
        let (_, seeds) = seeds.split_once(": ").unwrap();

        let seed_nums = seeds
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect_vec();

        let seed_ranges = seed_nums
            .iter()
            .tuples()
            .map(|(a, b)| *a..(*a + *b))
            .collect_vec();

        let mut mappings: HashMap<(String, String), HashMap<_, _>> = Default::default();

        while let Some(group) = groups.next() {
            let mut lines = group.lines().into_iter();
            let ident = lines.next().unwrap();
            let (from, to) = ident
                .strip_suffix(" map:")
                .unwrap()
                .split_once("-to-")
                .unwrap();

            let mut current_dict: HashMap<_, _> = Default::default();

            println!("{from} {to}");

            while let Some(mapping) = lines.next() {
                let nums = mapping
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect_vec();
                let dest_start = *nums.get(0).unwrap();
                let source_start = *nums.get(1).unwrap();
                let offset = *nums.get(2).unwrap();

                let source_range = source_start..(source_start + offset);
                dbg!(&mapping);
                let destination_range = dest_start..(dest_start + offset);
                current_dict.insert(source_range, destination_range);
                // nothing
            }

            mappings.insert((from.into(), to.into()), current_dict);
        }

        // fn rev_lookup(from: &str, to: &str, num: u64, mappings: &HashMap<(String, String), HashMap<Range<u64>, Range<u64>>>) -> u64 {
        //
        //     if let Some(mapping) = mappings.get(&(to.into(), from.into())) {
        //         mapping.iter().find(|(&&start, &&end)| start)
        //         0u64
        //     } else {
        //         num
        //     }
        // }

        let mut results = Vec::new();

        for seed_num in seed_nums {
            let mut cur_num = seed_num;
            let mut kind = "seed".to_owned();

            loop {
                let cur_map = mappings
                    .iter()
                    .find(|((from, _), _)| *from == kind)
                    .expect("one mapping must be found");

                println!("Current number: {cur_num}");
                println!("from {} to {}", kind, cur_map.0.clone().1);
                if let Some((s, e)) = cur_map.1.iter().find(|(start, _)| start.contains(&cur_num)) {
                    println!("{:#?} maps to {:#?}", s, e);
                    let diff = e.start as i128 - s.start as i128;
                    println!("diff is {diff}");
                    cur_num = ((cur_num as i128) + diff) as u64;
                } else {
                    println!("{} is exactly the same", cur_num);
                    // cur num stays the same
                }
                kind = cur_map.0.clone().1;

                if kind == "location" {
                    results.push(cur_num);
                    println!("\n\n----------------------------------\n\n");
                    break;
                }
            }
        }

        let p1 = results.into_iter().min().unwrap();
        assert_ne!(2045303401, p1);
        self.submit_part1(p1);

        for location in 0u64.. {
            let mut cur_num = location;
            let mut kind = "location".to_owned();

            loop {
                let cur_map = mappings
                    .iter()
                    .find(|((_, from), _)| *from == kind)
                    .expect("one mapping must be found");

                // println!("Current number: {cur_num}");
                // println!("from {} to {}", kind, cur_map.0.clone().0);
                if let Some((s, e)) = cur_map.1.iter().find(|(_, end)| end.contains(&cur_num)) {
                    // println!("{:#?} maps to {:#?}", e, s);
                    let diff = s.start as i128 - e.start as i128;
                    // println!("diff is {diff}");
                    cur_num = ((cur_num as i128) + diff) as u64;
                } else {
                    // println!("{} is exactly the same", cur_num);
                    // cur num stays the same
                }
                kind = cur_map.0.clone().0;

                if kind == "seed" {
                    for range in seed_ranges.iter() {
                        if range.contains(&cur_num) {
                            self.submit_part2(location);
                            return Ok(());
                        }
                    }

                    break;
                }
            }
        }

        Ok(())
    }
}
