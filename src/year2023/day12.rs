use crate::create_solution;
use crate::prelude::StringTools;
use crate::puzzle::{Answerable, Solution};
use itertools::Itertools;
use std::collections::VecDeque;
create_solution!(Day12, 2023, 12);

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum State {
    Good,
    Bad,
    Unknown,
}
impl Solution for Day12 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        //         let input = "???.### 1,1,3
        // .??..??...?##. 1,1,3
        // ?#?#?#?#?#?#?#? 1,3,1,6
        // ????.#...#... 4,1,1
        // ????.######..#####. 1,6,5
        // ?###???????? 3,2,1";
        let input = "?###???????? 3,2,1";

        let mut sum = 0;
        for (line_nr, line) in input.lines().enumerate() {
            println!("START :: [{line_nr}] {line}");

            let (rec, nums) = line.split_once(' ').unwrap();
            let nums = nums.nums::<usize>().collect_vec();

            // Expand logic
            // let mut new_rec = String::new();
            // let mut new_nums: Vec<usize> = Vec::new();
            // for i in 0..5 {
            //     new_rec.push_str(rec);
            //     new_nums.extend_from_slice(&nums);
            //     if i < 4 {
            //         new_rec.push('?');
            //     }
            // }
            // let nums = new_nums;

            let rec = rec
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

            let t = solve2(&rec, &nums);
            sum += t;
            println!("END :: [{line_nr}] {line} :: {t}\n");
        }

        self.submit_part1(sum);

        Ok(())
    }
}

fn solve3(rec: &[State], nums: &[usize]) -> usize {
    let queue = VecDeque::new();
    queue.push_front((rec, nums));

    while let Some((rec, nums)) = queue.pop_front() {}
}

fn is_valid(rec: &[State], nums: &[usize]) -> usize {}
fn solve2(rec: &[State], nums: &[usize]) -> usize {
    let mut streak = 0;

    let mut cur_num_idx = 0;

    if nums.is_empty() {
        return if rec.iter().all(|s| *s == State::Good) {
            println!("NUMS EXHAUSTED ABORT VALID");
            1
        } else {
            println!("NUMS EXHAUSTED ABORT INVALID");
            0
        };
    }

    for start in 0..rec.len() {
        match rec[start] {
            State::Bad => {
                streak += 1;

                // We got in to a bad situation, this arrangement is not valid
                if streak > nums[cur_num_idx] {
                    println!("ABORT INVALID");
                    return 0;
                }
            }
            State::Good => {
                // If we were on a streak, and that streak exactly matches the streak we were looking for
                if streak > 0 && streak == nums[cur_num_idx] {
                    // If we end a streak and run out of numbers we can just check to end
                    if nums[(cur_num_idx + 1)..].is_empty() {
                        return if rec[start..].iter().all(|s| *s == State::Good) {
                            1
                        } else {
                            0
                        };
                    }

                    // if we still have streaks to find we can just continue
                    cur_num_idx += 1;
                    streak = 0;
                } else if streak > 0 && streak > nums[cur_num_idx] {
                    panic!("error streak is too high");
                } else if streak == 0 {
                    // we're not on a streak so we just continue happily
                    // do nothing
                } else {
                    println!("STREAK INTERRUPTED: ABORT INVALID");
                    return 0;
                }
            }
            State::Unknown => {
                if streak > 0 && streak < nums[cur_num_idx] {
                    // This question mark must be bad otherwise it can't be valid just continue
                    streak += 1;
                } else if streak > 0 && streak == nums[cur_num_idx] {
                    // If we were on a streak and we've totally exhausted it then this must be good

                    // We're ending a streak here so if there are no more streaks we can just return if all the remaining states are good
                    if nums[(cur_num_idx + 1)..].is_empty() {
                        return if rec[start..].iter().all(|s| *s == State::Good) {
                            println!("NUMS EXHAUSTED ON ? ABORT VALID");
                            1
                        } else {
                            println!("NUMS EXHAUSTED ON ? ABORT INVALID");
                            0
                        };
                    }

                    // otherwise just continue
                    cur_num_idx += 1;
                    streak = 0;
                } else if streak == 0 {
                    // We're not on a streak but we need to form one so this could go two ways

                    // GOOD
                    let mut new_good = rec[start..].to_vec();
                    new_good[0] = State::Good;
                    println!(
                        "new_good: {:#?} {:#?}",
                        to_string(&new_good),
                        &nums[cur_num_idx..]
                    );
                    let good_count = solve2(&new_good, &nums[cur_num_idx..]);

                    // BAD
                    let mut new_bad = rec[start..].to_vec();
                    new_bad[0] = State::Bad;
                    println!("new_bad: {:#?}", to_string(&new_bad));
                    let bad_count = solve2(&new_bad, &nums[cur_num_idx..]);

                    println!(
                        "recursed on {} good_count: {} bad_count: {}",
                        rec.len(),
                        good_count,
                        bad_count
                    );

                    return good_count + bad_count;
                } else if streak > 0 {
                    println!(
                        "streak: {streak} cur_num: {} cur_num_idx: {cur_num_idx}",
                        nums[cur_num_idx]
                    );

                    panic!("what's going on");
                } else {
                    panic!("what's going on");
                }
            }
        }
    }

    // println!("RECORD EXHAUSTED");
    if (streak == 0 || streak == nums[cur_num_idx]) && nums[(cur_num_idx + 1)..].is_empty() {
        println!("RECORD EXHAUSTED ABORT VALID");
        1
    } else {
        println!(
            "RECORD EXHAUSTED ABORT INVALID {} {} {}",
            streak,
            nums[cur_num_idx],
            nums.len()
        );
        0
    }
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
fn solve(rec: &str, nums: &[usize]) -> usize {
    let indexes = rec
        .chars()
        .enumerate()
        .filter(|(_idx, ch)| *ch == '?')
        .map(|(idx, _ch)| idx)
        .collect_vec();

    let fill = nums.iter().sum::<usize>() - rec.chars().filter(|ch| *ch == '#').count();

    let mut buf = String::new();

    let mut sum = 0;

    for p in indexes.iter().combinations(fill) {
        buf.clear();
        for (oi, oc) in rec.chars().enumerate() {
            if oc == '?' {
                if p.contains(&&oi) {
                    buf.push('#');
                } else {
                    buf.push('.');
                }
            } else {
                buf.push(oc);
            }
        }

        let config = buf
            .chars()
            .group_by(|&c| c == '#')
            .into_iter()
            .filter_map(|(b, g)| b.then_some(g.count()))
            .collect_vec();

        if config == nums {
            // println!("VALID: {buf}");
            sum += 1;
        } else {
            // println!("INVALID: {buf}");
        }

        // break;
    }

    sum
}
