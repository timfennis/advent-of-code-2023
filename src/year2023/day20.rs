use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use ahash::HashMap;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::io::stdin;
create_solution!(Day20, 2023, 20);

impl Solution for Day20 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        println!("{input}");

        self.submit_part1(solve(input));

        // let mut new_input = String::from(input);
        // new_input.push_str(", rx");

        // println!("{new_input}");
        // return Ok(());

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

    for i in 0..1000 {
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

    let mut low = 0;
    let mut high = 0;

    let mut queue: VecDeque<(&str, &str, bool)> = Default::default();

    let mut has_been_true: HashSet<&str> = Default::default();
    println!("FLIP FLOPS: {}", flip_flop_modules.len());
    println!("CONJUNCTION: {}", conjunction_modules.len());

    // dbg!(&conjunction_modules);
    fn wait() {
        let mut string = String::new();
        stdin().read_line(&mut string).unwrap();
    }

    for presses in 1.. {
        queue.push_front(("YOU!", "broadcaster", false));

        // dbg!(
        //     conjunction_modules.get("ff").unwrap(),
        //     conjunction_modules.get("th").unwrap(),
        //     conjunction_modules.get("zs").unwrap(),
        //     conjunction_modules.get("nt").unwrap(),
        // );
        //
        if presses % 2048 == 0 {
            // println!("--------- {presses} -----------");
            // dbg!(&conjunction_modules);
            // wait();
        }

        if presses % 14970971 == 0 {
            println!("----- CYCLE {} -----", presses / 14970971);
        }

        // for (&name, state) in &flip_flop_modules {
        //     if *state && !has_been_true.contains(name) {
        //         has_been_true.insert(name);
        //         println!("{presses} {name} was true for the first time");
        //     }
        // }

        while let Some((prev_module, current_module, pulse)) = queue.pop_front() {
            // println!("{current_module} {pulse}");
            if pulse {
                high += 1;
            } else {
                low += 1;
            }
            // Every 14970971 presses there is a cycle
            // The cycle breaks in some way around: 209593593
            // jg 14970970 - 0 1 0 0
            // jg 15183378 - 1 0 0 0
            // jg 15244066 - 0 0 0 1
            // jm 15799840 - 0 1 0 0
            // rh 15862992 - 0 1 0 0
            // jm 16088056 - 0 0 0 1

            // jg 29941941 - 0 1 0 0
            // jg 30366757 - 1 0 0 0
            // jg 30488133 - 0 0 0 1
            // jm 31599681 - 0 1 0 0
            // rh 31725985 - 0 1 0 0
            // jm 32176113 - 0 0 0 1

            // jg 44912912 - 0 1 0 0
            // jg 45550136 - 1 0 0 0
            // jg 45732200 - 0 0 0 1
            // jm 47399522 - 0 1 0 0
            // rh 47588978 - 0 1 0 0
            // jm 48264170 - 0 0 0 1

            // jg 59883883 - 0 1 0 0
            // jg 60733515 - 1 0 0 0
            // jg 60976267 - 0 0 0 1
            // jm 63199363 - 0 1 0 0
            // rh 63451971 - 0 1 0 0
            // jm 64352227 - 0 0 0 1

            // jg 74854854 - 0 1 0 0
            // jg 75916894 - 1 0 0 0
            // jg 76220334 - 0 0 0 1
            // jm 78999204 - 0 1 0 0
            // rh 79314964 - 0 1 0 0
            // jm 80440284 - 0 0 0 1

            // JG JG JG JM RH JM
            // JG JG JG JM RH JM
            // JG JG JG JM RH
            if current_module == "mg" && pulse {
                let f = conjunction_modules.get("mg").unwrap();
                let jm = if *f.get("jm").unwrap() { 1 } else { 0 };
                let hf = if *f.get("hf").unwrap() { 1 } else { 0 };
                let jg = if *f.get("jg").unwrap() { 1 } else { 0 };
                let rh = if *f.get("rh").unwrap() { 1 } else { 0 };

                if jm > 0 || hf > 0 || jg > 0 || rh > 0 {
                    println!(
                        "{prev_module} {presses} {} - {} {} {} {}",
                        presses % 14970971,
                        jg,
                        jm,
                        rh,
                        hf,
                    );
                }
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

    low * high
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
