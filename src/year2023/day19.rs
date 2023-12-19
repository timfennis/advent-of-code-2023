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

    fn split(&self, (x, m, a, s): PartRanges) -> (PartRanges, PartRanges) {
        fn less(range: Range<usize>, num: usize) -> (Range<usize>, Range<usize>) {
            if !range.contains(&num) {
                panic!("this is not supposed to happen");
            }

            // 300..700
            // split on <500
            // 300..500 && 500..700
            assert_eq!((range.start..num).len() + (num..range.end).len(), range.len());
            (range.start..num, num..range.end)
        }

        fn more(range: Range<usize>, num: usize) -> (Range<usize>, Range<usize>) {
            if !range.contains(&num) {
                panic!("this is not supposed to happen");
            }


            // 300..700
            // split on >500
            // 300..501 && 501..700
            let (a, b) = ((num + 1)..range.end, range.start..(num + 1));
            assert_eq!(a.len() + b.len(), range.len());
            (a, b)
        }

        match self {
            Condition::LessThan(i, v) => match i {
                Identifier::X => {
                    let (good, bad) = less(x.clone(), *v);
                    ((good, m.clone(), a.clone(), s.clone()), (bad, m, a, s))
                }
                Identifier::M => {
                    let (good, bad) = less(m.clone(), *v);
                    ((x.clone(), good, a.clone(), s.clone()), (x, bad, a, s))
                }
                Identifier::A => {
                    let (good, bad) = less(a.clone(), *v);
                    ((x.clone(), m.clone(), good, s.clone()), (x, m, bad, s))
                }
                Identifier::S => {
                    let (good, bad) = less(s.clone(), *v);
                    ((x.clone(), m.clone(), a.clone(), good), (x, m, a, bad))
                }
            },

            Condition::GreaterThan(i, v) => match i {
                Identifier::X => {
                    let (good, bad) = more(x.clone(), *v);
                    ((good, m.clone(), a.clone(), s.clone()), (bad, m, a, s))
                }
                Identifier::M => {
                    let (good, bad) = more(m.clone(), *v);
                    ((x.clone(), good, a.clone(), s.clone()), (x, bad, a, s))
                }
                Identifier::A => {
                    let (good, bad) = more(a.clone(), *v);
                    ((x.clone(), m.clone(), good, s.clone()), (x, m, bad, s))
                }
                Identifier::S => {
                    let (good, bad) = more(s.clone(), *v);
                    ((x.clone(), m.clone(), a.clone(), good), (x, m, a, bad))
                }
            },
            _ => unreachable!("RIGHT?!"),
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
    ) -> usize {
        if step == "A" {
            return ranges.0.len() * ranges.1.len() * ranges.2.len() * ranges.3.len();
        }

        if step == "R" {
            return 0;
        }

        let rules = workflows.get(step).unwrap();

        let mut ans = 0;
        let mut cur_ranges = ranges;
        for (condition, name) in rules {

            match condition {
                Condition::LessThan(_, _) | Condition::GreaterThan(_, _) => {
                    let (good, bad) = condition.split(cur_ranges.clone());

                    println!("{:?} SPLIT INTO:\n{:?} AND:\n{:?}", condition, good, bad);
                    ans += find_rec(good, name, workflows);
                    cur_ranges = bad;
                }
                Condition::Always => {
                    println!("ALWAYS {:?}", condition);
                    ans += find_rec(cur_ranges.clone(), name, workflows);
                }
            }
        }

        ans
    }

    let ans = find_rec((1..4001, 1..4001, 1..4001, 1..4001), "in", &workflows);

    // let ans = 0;
    (
        accepted_parts
            .iter()
            .map(|(x, m, a, s)| *x + *m + *a + *s)
            .sum(),
        ans
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
