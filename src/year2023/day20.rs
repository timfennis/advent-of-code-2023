use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use ahash::HashMap;
use itertools::Itertools;
use num::integer::lcm;
use std::collections::{HashSet, VecDeque};
use std::fmt::Write;
use std::io::stdin;
create_solution!(Day20, 2023, 20);

impl Solution for Day20 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        // GB FX FZ ZG ZQ PQ MJ SQ CD
        // 1  2  4  6  7  9  10 11 12
        // 11_1  _11_  1111

        // HT VP MQ GZ BD LJ CX XR XD
        // 1  2  5  6  8  9  10 11 12
        // 11__  11_1  1111

        // VK MP LT XF DB DZ GD CV
        // 1  2  6  8  9  10 11 12
        // 11__  _1_1  1111

        // ZZ SH BF GG XS DX BM
        // 1  5  7  9  10 11 12
        // 1___  1_1_  1111

        self.submit_part1(solve(input));

        // 195397365072     WRONG
        // 225450741686403  WRONG
        // 240853834793347  ?????
        // 240853834793347

        // 4198764531195 is not correct
        self.submit_part2(solvep2(input));

        Ok(())
    }
}

fn solve(input: &str) -> usize {
    let mut flip_flop_modules: HashMap<&str, bool> = Default::default();
    let mut conjunction_modules: HashMap<&str, HashMap<&str, bool>> = Default::default();
    let mut connections: HashMap<&str, Vec<&str>> = Default::default();

    for line in input.lines() {
        let (mut from, to) = line.split_once(" -> ").unwrap();
        let to = to.split(", ");

        if from.starts_with('%') {
            from = from.strip_prefix('%').unwrap();
            flip_flop_modules.insert(from, false);
        } else if from.starts_with('&') {
            from = from.strip_prefix('&').unwrap();
            conjunction_modules.insert(from, Default::default());
        } else if from == "broadcaster" {
        } else {
            panic!("invalid module");
        }

        connections.insert(from, to.collect_vec());
    }

    for (from, to) in &connections {
        for t in to {
            if let Some(m) = conjunction_modules.get_mut(t) {
                m.insert(from, false);
            }
        }
    }

    let mut low = 0;
    let mut high = 0;

    let mut queue: VecDeque<(&str, &str, bool)> = Default::default();

    for _i in 0..1000 {
        queue.push_front(("YOU!", "broadcaster", false));
        // println!("------ ITERATION {i} ------");
        // dbg!(&flip_flop_modules, &conjunction_modules);
        // println!("---------");

        while let Some((prev_module, current_module, pulse)) = queue.pop_front() {
            if pulse {
                high += 1;
            } else {
                low += 1;
            }

            let Some(targets) = connections.get(current_module) else {
                continue;
            };

            if let Some(&state) = flip_flop_modules.get(current_module) {
                if pulse {
                    // do nothing
                } else {
                    let new_state = !state;
                    flip_flop_modules.insert(current_module, new_state);

                    for target in targets {
                        queue.push_back((current_module, *target, new_state));
                    }
                }
            } else if let Some(states) = conjunction_modules.get_mut(current_module) {
                // println!("GOT {pulse} FROM {prev_module} in {current_module}");
                states.insert(prev_module, pulse);

                // dbg!(&states);
                if states.iter().all(|(_, b)| *b) {
                    // all high
                    for target in targets {
                        queue.push_back((current_module, *target, false))
                    }
                } else {
                    for target in targets {
                        queue.push_back((current_module, *target, true))
                    }
                }
            } else {
                assert_eq!(current_module, "broadcaster");
                for target in targets {
                    queue.push_back((current_module, *target, pulse));
                }
            }
        }
    }

    low * high
}

fn solvep2(input: &str) -> usize {
    let mut flip_flop_modules: HashMap<&str, bool> = Default::default();
    let mut conjunction_modules: HashMap<&str, HashMap<&str, bool>> = Default::default();
    let mut connections: HashMap<&str, Vec<&str>> = Default::default();

    for line in input.lines() {
        let (mut from, to) = line.split_once(" -> ").unwrap();
        let to = to.split(", ");

        if from.starts_with('%') {
            from = from.strip_prefix('%').unwrap();
            flip_flop_modules.insert(from, false);
        } else if from.starts_with('&') {
            from = from.strip_prefix('&').unwrap();
            conjunction_modules.insert(from, Default::default());
        } else if from == "broadcaster" {
        } else {
            panic!("invalid module");
        }

        connections.insert(from, to.collect_vec());
    }

    for (from, to) in &connections {
        for t in to {
            if let Some(m) = conjunction_modules.get_mut(t) {
                m.insert(from, false);
            }
        }
    }

    let mut queue: VecDeque<(&str, &str, bool)> = Default::default();

    let _has_been_true: HashSet<&str> = Default::default();
    println!("FLIP FLOPS: {}", flip_flop_modules.len());
    println!("CONJUNCTION: {}", conjunction_modules.len());

    // dbg!(&conjunction_modules);

    #[allow(dead_code)]
    fn print(m: &HashMap<&str, bool>) {
        // MOST SIGNIFICANT BIT IS ON THE RIGHT SIDE
        let cells = [
            [
                "GB", "FX", "BN", "FZ", "BK", "ZG", "ZQ", "TT", "PQ", "MJ", "SQ", "CD",
            ],
            [
                "HT", "VP", "QJ", "HJ", "MQ", "GZ", "DS", "BD", "LJ", "CX", "XR", "XD",
            ],
            [
                "VK", "MP", "BQ", "PR", "QL", "LT", "VD", "XF", "DB", "DZ", "GD", "CV",
            ],
            [
                "ZZ", "RZ", "ZC", "DH", "SH", "MR", "BF", "GG", "PJ", "XS", "DX", "BM",
            ],
            // ["ZZ", "RZ", "QJ", "PR", "SH", "MR", "BF", "GG", "PJ", "XS", "DX", "BM"],
            // ["VK", "MP", "BQ", "DH", "BK", "LT", "VD", "XF", "DB", "DZ", "GD", "CV"],
            // ["HT", "VP", "BN", "HJ", "MQ", "GZ", "DS", "BD", "LJ", "CX", "XR", "XD"],
            // ["GB", "FX", "ZC", "FZ", "QL", "ZG", "ZQ", "TT", "PQ", "MJ", "SQ", "CD"],
        ];
        let mut buf = String::new();
        for cc in cells {
            for c in cc.iter().rev() {
                let bit = if *m.get(c.to_ascii_lowercase().as_str()).unwrap() {
                    1
                } else {
                    0
                };
                buf.push_str(&format!("{}", bit));
            }
            buf.push(' ');
            // print!(" ");
        }

        println!("{buf}");
    }

    fn all_ones(m: &HashMap<&str, bool>, cells: &[&str]) -> bool {
        // println!("---------");
        for c in cells {
            let val = *m.get(c.to_ascii_lowercase().as_str()).unwrap();
            // println!("{c} {val}");
            if !val {
                return false;
            }
        }

        true
    }

    // let v = connections.iter().sorted().collect_vec();
    // dbg!(&v);
    // wait();

    let mut vals: [HashSet<usize>; 4] = [
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
    ];

    for presses in 1..4096 {
        queue.push_front(("YOU!", "broadcaster", false));

        // dbg!(
        //     conjunction_modules.get("ff").unwrap(),
        //     conjunction_modules.get("th").unwrap(),
        //     conjunction_modules.get("zs").unwrap(),
        //     conjunction_modules.get("nt").unwrap(),
        // );
        //

        // 111101101010

        // if presses >= 3947 {
        //     print!("{presses} ");
        //     print(&flip_flop_modules);
        //     wait();
        // }

        // GB FX FZ ZG ZQ PQ MJ SQ CD
        // 1  2  4  6  7  9  10 11 12
        // 11_1  _11_  1111

        // for (&name, state) in &flip_flop_modules {
        //     if *state && !has_been_true.contains(name) {
        //         has_been_true.insert(name);
        //         println!("{presses} {name} was true for the first time");
        //     }
        // }

        while let Some((prev_module, current_module, pulse)) = queue.pop_front() {
            // println!("{current_module} {pulse}");

            // ["GB", "FX", "BN", "FZ", "BK", "ZG", "ZQ", "TT", "PQ", "MJ", "SQ", "CD"]
            // 0b111101101011
            if all_ones(
                &flip_flop_modules,
                &["GB", "FX", "FZ", "ZG", "ZQ", "PQ", "MJ", "SQ", "CD"],
            ) {
                vals[0].insert(presses);
                dbg!(&vals);
            }

            // HT VP MQ GZ BD LJ CX XR XD
            // 1  2  5  6  8  9  10 11 12
            // 11__  11_1  1111

            if all_ones(
                &flip_flop_modules,
                &["HT", "VP", "MQ", "GZ", "BD", "LJ", "CX", "XR", "XD"],
            ) {
                vals[1].insert(presses);
                dbg!(&vals);
            }
            // VK MP LT XF DB DZ GD CV
            // 1  2  6  8  9  10 11 12
            // 11__  _1_1  1111

            if all_ones(
                &flip_flop_modules,
                &["VK", "MP", "LT", "XF", "BD", "DZ", "GD", "CV"],
            ) {
                vals[2].insert(presses);
                dbg!(&vals);
            }
            // ZZ SH BF GG XS DX BM
            // 1  5  7  9  10 11 12
            // 1___  1_1_  1111

            if all_ones(
                &flip_flop_modules,
                &["ZZ", "SH", "BF", "GG", "XS", "DX", "BM"],
            ) {
                vals[3].insert(presses);
                dbg!(&vals);
            }

            if current_module == "rx" && !pulse {
                return presses;
            }

            let Some(targets) = connections.get(current_module) else {
                continue;
            };

            if let Some(&state) = flip_flop_modules.get(current_module) {
                if pulse {
                    // do nothing
                } else {
                    let new_state = !state;
                    flip_flop_modules.insert(current_module, new_state);

                    for target in targets {
                        queue.push_back((current_module, *target, new_state));
                    }
                }
            } else if let Some(states) = conjunction_modules.get_mut(current_module) {
                // println!("GOT {pulse} FROM {prev_module} in {current_module}");
                states.insert(prev_module, pulse);

                // dbg!(&states);
                if states.iter().all(|(_, b)| *b) {
                    // all high
                    for target in targets {
                        queue.push_back((current_module, *target, false))
                    }
                } else {
                    for target in targets {
                        queue.push_back((current_module, *target, true))
                    }
                }
            } else {
                assert_eq!(current_module, "broadcaster");
                for target in targets {
                    queue.push_back((current_module, *target, pulse));
                }
            }
        }
    }

    // let mut ans = usize::MAX;
    dbg!(&vals);
    let mut ans = usize::MAX;
    let l = lcm(
        *vals[0].iter().max().unwrap(),
        *vals[1].iter().max().unwrap(),
    );
    let l = lcm(l, *vals[2].iter().max().unwrap());
    let l = lcm(l, *vals[3].iter().max().unwrap());
    ans = std::cmp::min(l, ans);
    ans
}

#[cfg(test)]
mod test {
    use crate::year2023::day20::solve;

    #[test]
    fn year_2023_day_20_example() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        assert_eq!(solve(input), 11687500);
    }
}
