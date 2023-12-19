use crate::create_solution;
use crate::prelude::StringTools;
use crate::puzzle::{Answerable, Solution};
use ahash::HashMap;
use itertools::{min, Itertools};
use std::ops::Range;
use std::str::FromStr;
create_solution!(Day19, 2023, 19);

impl Solution for Day19 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let (p1, p2) = solve(input);
        self.submit_part1(p1);
        self.submit_part2(p2);
        Ok(())
    }
}
#[derive(Debug, Copy, Clone)]
enum Identifier {
    X,
    M,
    A,
    S,
}

impl FromStr for Identifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "m" {
            Ok(Self::M)
        } else if s == "x" {
            Ok(Self::X)
        } else if s == "a" {
            Ok(Self::A)
        } else if s == "s" {
            Ok(Self::S)
        } else {
            Err("invalid identifier".to_string())
        }
    }
}
#[derive(Debug, Copy, Clone)]
enum Condition {
    LessThan(Identifier, usize),
    GreaterThan(Identifier, usize),
    Always,
}

type PartRanges = (Range<usize>, Range<usize>, Range<usize>, Range<usize>);

impl Condition {
    fn matches(&self, (x, m, a, s): Part) -> bool {
        match self {
            Condition::LessThan(i, v) => match i {
                Identifier::X => x < *v,
                Identifier::M => m < *v,
                Identifier::A => a < *v,
                Identifier::S => s < *v,
            },

            Condition::GreaterThan(i, v) => match i {
                Identifier::X => x > *v,
                Identifier::M => m > *v,
                Identifier::A => a > *v,
                Identifier::S => s > *v,
            },
            Condition::Always => true,
        }
    }

    fn modify_range(&self, (x, m, a, s): PartRanges) -> PartRanges {
        fn less(range: Range<usize>, num: usize) -> Range<usize> {
            if num < range.start {
                println!("DESTROYED RANGE");
                return 0..0;
            }
            Range {
                start: std::cmp::min(range.start, num),
                end: std::cmp::min(range.end, num),
            }
        }

        fn more(range: Range<usize>, num: usize) -> Range<usize> {
            if num > range.end {
                println!("DESTROYED RANGE");
                return 0..0;
            }
            Range {
                start: std::cmp::max(range.start, num + 1),
                end: std::cmp::max(range.end, num),
            }
        }

        match self {
            Condition::LessThan(i, v) => match i {
                Identifier::X => (less(x, *v), m, a, s),
                Identifier::M => (x, less(m, *v), a, s),
                Identifier::A => (x, m, less(a, *v), s),
                Identifier::S => (x, m, a, less(s, *v)),
            },

            Condition::GreaterThan(i, v) => match i {
                Identifier::X => (more(x, *v), m, a, s),
                Identifier::M => (x, more(m, *v), a, s),
                Identifier::A => (x, m, more(a, *v), s),
                Identifier::S => (x, m, a, more(s, *v)),
            },
            Condition::Always => (x, m, a, s),
        }
    }
}
type Part = (usize, usize, usize, usize);

fn solve(input: &str) -> (usize, usize) {
    let mut all_parts: Vec<Part> = Default::default();

    let (rules, parts) = input
        .split_once("\n\n")
        .expect("must have a double linebreak");

    type WorkflowMap<'a> = HashMap<&'a str, Vec<(Condition, &'a str)>>;
    let mut workflows: WorkflowMap = Default::default();
    for rule in rules.lines() {
        let (name, rest) = rule.split_once('{').unwrap();
        let rest = rest.strip_suffix('}').unwrap();

        let mut parsed_rules = Vec::new();
        for instruction in rest.split(',') {
            match instruction.split_once(':') {
                Some((instruction, destination)) => {
                    if let Some((a, b)) = instruction.split_once('<') {
                        parsed_rules.push((
                            Condition::LessThan(
                                a.parse::<Identifier>().unwrap(),
                                b.parse::<usize>().unwrap(),
                            ),
                            destination,
                        ));
                    } else if let Some((a, b)) = instruction.split_once('>') {
                        parsed_rules.push((
                            Condition::GreaterThan(
                                a.parse::<Identifier>().unwrap(),
                                b.parse::<usize>().unwrap(),
                            ),
                            destination,
                        ));
                    }
                }
                None => {
                    let destination = instruction;
                    parsed_rules.push((Condition::Always, destination));
                }
            }
        }
        workflows.insert(name, parsed_rules);
    }

    for part in parts.lines() {
        let nums = part.nums::<usize>().collect_vec();
        match nums.as_slice() {
            [x, m, a, s] => {
                all_parts.push((*x, *m, *a, *s));
            }
            _ => panic!("invalid part"),
        }
    }

    let mut accepted_parts = Vec::new();
    for part in all_parts {
        let mut current_workflow = "in";
        'main: loop {
            if current_workflow == "A" {
                println!("Accepted: {:?}", part);
                accepted_parts.push(part);
                break;
            }

            if current_workflow == "R" {
                println!("Rejected: {:?}", part);
                break;
            }
            let workflow = workflows.get(current_workflow).unwrap();

            for (condition, destination) in workflow {
                if condition.matches(part) {
                    current_workflow = *destination;
                    continue 'main;
                }
            }

            unreachable!("This can never happen");
        }
    }

    // let mut current_condition = *c;

    fn find_rec(
        ranges: PartRanges,
        step: &str,
        workflows: &WorkflowMap,
    ) -> Option<(
        Vec<Range<usize>>,
        Vec<Range<usize>>,
        Vec<Range<usize>>,
        Vec<Range<usize>>,
    )> {
        if step == "A" {
            return Some((
                vec![ranges.0],
                vec![ranges.1],
                vec![ranges.2],
                vec![ranges.3],
            ));
        }

        if step == "R" {
            return None;
        }

        let rules = workflows.get(step).unwrap();

        let mut xr = vec![ranges.0.clone()];
        let mut mr = vec![ranges.1.clone()];
        let mut ar = vec![ranges.2.clone()];
        let mut sr = vec![ranges.3.clone()];
        for (condition, next) in rules {
            let new_ranges = condition.modify_range(ranges.clone());
            if let Some((xx, mm, aa, ss)) = find_rec(new_ranges, next, workflows) {
                for x in xx {
                    let mut done = false;
                    for er in xr.iter_mut() {
                        let merge = merge_range(er.clone(), x.clone());
                        if merge.is_ok() {
                            *er = merge.unwrap();
                            done = true;
                            break;
                        }
                    }
                    if !done {
                        xr.push(x);
                    }
                }

                for m in mm {
                    let mut done = false;
                    for er in mr.iter_mut() {
                        let merge = merge_range(er.clone(), m.clone());
                        if merge.is_ok() {
                            *er = merge.unwrap();
                            done = true;
                            break;
                        }
                    }

                    if done == false {
                        mr.push(m);
                    }
                }

                for a in aa {
                    let mut done = false;
                    for er in ar.iter_mut() {
                        let merge = merge_range(er.clone(), a.clone());
                        if merge.is_ok() {
                            *er = merge.unwrap();
                            done = true;
                            break;
                        }
                    }

                    if done == false {
                        ar.push(a);
                    }
                }

                for s in ss {
                    let mut done = false;
                    for er in sr.iter_mut() {
                        let merge = merge_range(er.clone(), s.clone());
                        if merge.is_ok() {
                            *er = merge.unwrap();
                            done = true;
                            break;
                        }
                    }

                    if done == false {
                        sr.push(s);
                    }
                }
            }
        }

        Some((xr, mr, ar, sr))
    }

    let (xx, mm, aa, ss) =
        find_rec((1..4001, 1..4001, 1..4001, 1..4001), "in", &workflows).unwrap();

    println!("{:?} {:?} {:?} {:?} ", xx, mm, aa, ss);

    let ans = xx.iter().map(|r| r.len()).sum::<usize>()
        * mm.iter().map(|r| r.len()).sum::<usize>()
        * aa.iter().map(|r| r.len()).sum::<usize>()
        * ss.iter().map(|r| r.len()).sum::<usize>();

    // let ans = 0;
    (
        accepted_parts
            .iter()
            .map(|(x, m, a, s)| *x + *m + *a + *s)
            .sum(),
        ans, // f.0.len() * f.1.len() * f.2.len() * f.3.len(),
    )
}
fn merge_range(
    r1: Range<usize>,
    r2: Range<usize>,
) -> Result<Range<usize>, (Range<usize>, Range<usize>)> {
    let ls = std::cmp::min(r1.start, r2.start);
    let hs = std::cmp::max(r1.start, r2.start);
    let le = std::cmp::min(r1.end, r2.end);
    let he = std::cmp::max(r1.end, r2.end);

    if hs > le {
        return Err((ls..le, hs..he));
    }

    return Ok(ls..he);
}

#[cfg(test)]
mod test {
    use crate::year2023::day19::solve;

    #[test]
    fn year_2023_day_19_example() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

        assert_eq!(solve(input), (19114, 167409079868000usize));
    }
}
