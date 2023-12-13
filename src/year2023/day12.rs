use crate::create_solution;
use crate::prelude::StringTools;
use crate::puzzle::{Answerable, Solution};
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::VecDeque;
use std::ops::Not;
use std::time::Instant;
create_solution!(Day12, 2023, 12);

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
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

        assert_ne!(sum, 4310203286787);

        Ok(())
    }
}

fn run(input: &str, expand: usize) -> usize {
    let mut sum = 0;

    for (_line_nr, line) in input.lines().enumerate() {
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
        let t = solve_dfs_recursive(&states, None, &new_nums);
        let _duration = start.elapsed();
        sum += t;
    }

    sum
}

fn key(rec: &[State], current_group_size: Option<usize>, nums: &[usize]) -> String {
    format!("{:?}{:?}{:?}", rec, current_group_size, nums)
}
#[cached(key = "String", convert = "{key(rec, current_group_size, nums)}")]
fn solve_dfs_recursive(rec: &[State], current_group_size: Option<usize>, nums: &[usize]) -> usize {
    if rec.is_empty() {
        return if let Some(size) = current_group_size {
            if nums.len() == 1 && size == nums[0] {
                1
            } else {
                0
            }
        } else if nums.is_empty() {
            1
        } else {
            0
        };
    }

    let head = rec[0];
    let tail = &rec[1..];

    match (head, current_group_size) {
        (State::Good, None) => {
            // If we encounter a . and we're not in a group we can just continue walking
            solve_dfs_recursive(tail, None, nums)
        }
        (State::Good, Some(size)) => {
            // If we encounter a . and we're in a group we must end the group
            if !nums.is_empty() && size != nums[0] {
                return 0;
            }

            if nums.is_empty() {
                return 0;
            }

            solve_dfs_recursive(tail, None, &nums[1..])
        }
        (State::Bad, None) => {
            // If we encounter bad and we're not in a group can enter group 0
            solve_dfs_recursive(tail, Some(1), nums)
        }
        (State::Bad, Some(size)) => {
            // If we encounter bad and we're in a group we continue the same group
            if nums.is_empty() || size > nums[0] {
                return 0;
            }
            solve_dfs_recursive(tail, Some(size + 1), nums)
        }
        (State::Unknown, None) => {
            let bad = solve_dfs_recursive(tail, Some(1), nums);
            let good = solve_dfs_recursive(tail, None, nums);

            bad + good
        }
        (State::Unknown, Some(size)) => {
            if nums.is_empty() {
                return 0;
            }

            let bad = solve_dfs_recursive(tail, Some(size + 1), nums);

            let good = if size == nums[0] {
                solve_dfs_recursive(tail, None, &nums[1..])
            } else {
                0
            };

            bad + good
        }
    }
}

#[allow(dead_code)]
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
    use crate::year2023::day12::run;
    use crate::year2023::Day12;

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
