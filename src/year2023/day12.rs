use crate::create_solution;
use crate::prelude::StringTools;
use crate::puzzle::{Answerable, Solution};
use itertools::Itertools;
use std::collections::VecDeque;
use std::ops::Not;
use std::time::Instant;
// use crate::year2023::day12::ValidationResult::Valid;
create_solution!(Day12, 2023, 12);

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum State {
    Good,
    Bad,
    Unknown,
}

impl Solution for Day12 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let sum = run(input, 0);
        self.submit_part1(sum);

        let sum = run(input, 4);
        self.submit_part2(sum);

        Ok(())
    }
}


fn run(input: &str, expand: usize) -> usize {
    let mut sum = 0;

    for (line_nr, line) in input.lines().enumerate() {
        let (rec, nums) = line.split_once(' ').unwrap();
        let nums = nums.nums::<usize>().collect_vec();

        // Expand logic
        let mut new_rec = String::from(rec);
        let mut new_nums: Vec<usize> = nums.clone();

        for i in 0..expand {
            if i < expand {
                new_rec.push('?');
            }
            new_rec.push_str(rec);
            new_nums.extend_from_slice(&nums);
        }

        let states = new_rec
            .chars()
            .map(|ch| {
                if ch == '#' {
                    State::Bad
                } else if ch == '?' {
                    State::Unknown
                } else {
                    State::Good
                }
            })
            .collect_vec();

        let start = Instant::now();
        let t = solve_dfs(states, &new_nums);
        let duration = start.elapsed();
        sum += t;
        println!("END :: [{line_nr}] {line} :: {t} in {:#?}\n", duration);
    }

    sum
}

enum ValidationResult {
    Invalid,
    KeepSearching,
}

fn could_be_valid(rec: &[State], nums: &[usize]) -> ValidationResult {
    let num_of_bad = rec.iter().filter(|s| **s == State::Bad).count();
    let num_of_unknown = rec.iter().filter(|s| **s == State::Unknown).count();
    // let num_of_good = rec.iter().filter(|s| **s == State::Good).count();
    let total_bad = nums.iter().sum::<usize>();

    if num_of_bad > total_bad || num_of_bad + num_of_unknown < total_bad {
        return ValidationResult::Invalid;
    }

    // if num_of_bad == total_bad && num_of_unknown > 0 {
    //     // all unknowns must be good
    //
    //     // println!("all unknowns must be good: {}", num_of_unknown);
    //     // return ValidationResult::Valid(num_of_unknown);
    //     // return ValidationResult::KeepSearching;
    //     // return ValidationResult::Valid(1);
    //     return ValidationResult::KeepSearching;
    // }

    let mut nums = nums.into_iter();
    let mut streak = 0;
    let mut target_streak = *nums.next().unwrap();

    for start in 0..rec.len() {
        match rec[start] {
            State::Bad => {
                streak += 1;

                // If we produced a string that contains a streak that's too high we return false
                if streak > target_streak {
                    return ValidationResult::Invalid;
                }
            }
            State::Good => {
                // If we were on a streak, and that streak exactly matches the streak we were looking for
                if streak > 0 && streak == target_streak {
                    // if we still have streaks to find we can just continue
                    if let Some(n) = nums.next() {
                        target_streak = *n;
                    } else {
                        // If there are no more streaks wanted we do this
                        return if rec[start + 1..].iter().all(|s| *s != State::Bad) {
                            ValidationResult::KeepSearching
                        } else {
                            ValidationResult::Invalid
                        };
                    }

                    streak = 0;
                } else if streak > 0 && streak > target_streak {
                    panic!("error streak is too high");
                } else if streak == 0 {
                    // we're not on a streak so we just continue happily
                    // do nothing
                } else if streak > 0 && streak < target_streak {
                    // println!("STREAK INTERRUPTED: ABORT INVALID");
                    return ValidationResult::Invalid;
                } else {
                    panic!("this doesn't happen right?");
                }
            }
            State::Unknown => {
                if streak > 0 && streak < target_streak {
                    // must be #
                    streak += 1;
                } else if streak > 0 && streak == target_streak {
                    // must be .
                    streak = 0;
                    if let Some(n) = nums.next() {
                        target_streak = *n;
                    } else {
                        return if rec[start + 1..].iter().all(|s| *s != State::Bad) {
                            ValidationResult::KeepSearching
                        } else {
                            ValidationResult::Invalid
                        };
                    }
                } else {
                    assert_eq!(streak, 0);
                    // TODO this pruning might be a source of bugs
                    let nums = nums.collect_vec();
                    let needed = nums.iter().map(|n| **n + 1).sum::<usize>() + target_streak;
                    let remaining = rec.len() - start;

                    if needed > remaining {
                        return ValidationResult::Invalid;
                    }

                    // let rem = rec[start..].iter().group_by(|s| **s == State::Good).into_iter().filter_map(|(s, a)| s.not().then_some(a.count())).collect_vec();
                    //
                    // if nums.iter().any(|n| *n > rem.iter().max().unwrap()) {
                    //     return ValidationResult::Invalid;
                    // }
                    return ValidationResult::KeepSearching;
                }
            }
        }
    }

    if streak == 0 || streak == target_streak {
        return if nums.next().is_none() {
            // must be valid HERE right?
            ValidationResult::KeepSearching
        } else {
            ValidationResult::Invalid
        };
    } else {
        ValidationResult::Invalid
    }
}

fn is_valid(rec: &[State], nums: &[usize]) -> bool {
    let vals = rec.iter().group_by(|s| **s).into_iter().filter_map(|(a, b)| (a == State::Bad).then_some(b.count())).collect_vec();
    vals == nums
}

fn solve_dfs(rec: Vec<State>, nums: &[usize]) -> usize {
    let mut counter = 0;
    let mut queue = Vec::new();
    queue.push(rec);

    let mut ans = 0;
    'main: while let Some(rec) = queue.pop() {
        counter += 1;
        // if counter % 1000000 == 0 {
        //     println!("{} {}", to_string(&rec), nums.iter().join(","));
        // }
        for (idx, state) in rec.iter().enumerate() {
            if *state == State::Unknown {
                let mut new_bad = rec.to_vec();
                new_bad[idx] = State::Bad;

                match could_be_valid(&new_bad, &nums) {
                    ValidationResult::Invalid => {
                        // NON
                    }
                    ValidationResult::KeepSearching => {
                        queue.push(new_bad);
                    }
                }

                let mut new_good = rec.to_vec();
                new_good[idx] = State::Good;

                match could_be_valid(&new_good, &nums) {
                    ValidationResult::Invalid => {
                        // NON
                    }
                    ValidationResult::KeepSearching => {
                        queue.push(new_good);
                    }
                }

                continue 'main;
            }
        }

        // Everything is known

        if is_valid(&rec, &nums) {
            ans += 1;
        }
    }

    ans
}

fn to_string(rec: &[State]) -> String {
    let mut buf = String::with_capacity(rec.len());
    for s in rec {
        buf.push(match s {
            State::Good => '.',
            State::Bad => '#',
            State::Unknown => '?',
        });
    }
    buf
}

#[cfg(test)]
mod test {
    use crate::puzzle::Solution;
    use crate::year2023::Day12;
    use crate::year2023::day12::run;

    #[test]
    fn example_part_1() {
        assert_eq!(run("???.### 1,1,3", 0), 1);
        assert_eq!(run(".??..??...?##. 1,1,3", 0), 4);
        assert_eq!(run("?#?#?#?#?#?#?#? 1,3,1,6", 0), 1);
        assert_eq!(run("????.#...#... 4,1,1", 0), 1);
        assert_eq!(run("????.######..#####. 1,6,5", 0), 4);
        assert_eq!(run("?###???????? 3,2,1", 0), 10);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(run("???.### 1,1,3", 4), 1);
        assert_eq!(run(".??..??...?##. 1,1,3", 4), 16384);
        assert_eq!(run("?#?#?#?#?#?#?#? 1,3,1,6", 4), 1);
        assert_eq!(run("????.#...#... 4,1,1", 4), 16);
        assert_eq!(run("????.######..#####. 1,6,5", 4), 2500);
        assert_eq!(run("?###???????? 3,2,1", 4), 506250);
    }
}