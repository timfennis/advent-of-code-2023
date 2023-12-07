use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
create_solution!(Day7, 2023, 7);

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: String,
}

impl Hand {
    fn score(&self) -> u32 {
        // 7 score types
        // 13 card types

        let crd = vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];

        let mut map = HashMap::new();
        // println!("{}", self.cards);
        for kind in crd {
            for c in self.cards.chars() {
                if c == kind {
                    *map.entry(kind).or_insert(0u32) += 1;
                }
            }
        }

        let mut values = map.iter().map(|(k, v)| *v).collect_vec();
        values.sort();

        // dbg!(&values);

        if values.contains(&5) {
            return 7;
        }

        if values.contains(&4) {
            return 6;
        }

        if values.contains(&3) {
            return if values.contains(&2) { 5 } else { 4 };
        }
        if values.iter().filter(|v| **v == 2).count() == 2 {
            return 3;
        }

        if values.contains(&2) {
            return 2;
        }

        1
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let crd = vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];
        if self.score().cmp(&other.score()) != Ordering::Equal {
            return self.score().cmp(&other.score());
        }

        for (a, b) in self.cards.chars().zip(other.cards.chars()) {
            let a = crd.iter().position(|p| *p == a).unwrap();
            let b = crd.iter().position(|p| *p == b).unwrap();

            if a.cmp(&b) != Ordering::Equal {
                return a.cmp(&b);
            }
        }

        Ordering::Equal
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cards: s.to_owned(),
        })
    }
}
impl Solution for Day7 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        //         let input = "32T3K 765
        // T55J5 684
        // KK677 28
        // KTJJT 220
        // QQQJA 483";
        let mut hands = input
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(' ').unwrap();
                (left.parse::<Hand>().unwrap(), right.parse::<u32>().unwrap())
            })
            .collect_vec();

        hands.sort_by(|a, b| a.0.cmp(&b.0));
        // hands.reverse();

        let mut ans = 0;
        for (pos, (hand, bid)) in hands.iter().enumerate() {
            ans += (pos as u64 + 1) * (*bid as u64);
        }

        // dbg!(hands);

        self.submit_part1(ans);

        // dbg!(&hands);
        Ok(())
    }
}
