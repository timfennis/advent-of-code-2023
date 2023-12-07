use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
create_solution!(Day7, 2023, 7);

#[derive(Debug)]
struct Hand {
    cards: String,
}

impl Hand {
    fn tally(&self) -> [u8; 13] {
        let mut tally = [0u8; 13];
        for c in self.cards.chars() {
            let idx = match c {
                '2' => 0,
                '3' => 1,
                '4' => 2,
                '5' => 3,
                '6' => 4,
                '7' => 5,
                '8' => 6,
                '9' => 7,
                'T' => 8,
                'J' => 9,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
                _ => unreachable!(),
            };
            tally[idx] += 1;
        }

        tally
    }
    fn score_part1(&self) -> u32 {
        let values = self.tally();

        Self::score_tally(&values)
    }

    fn score_part2(&self) -> u32 {
        let mut values = self.tally();
        let joker_count = values[9];

        // set jokers to 0
        values[9] = 0;

        // sort to find the max value at index 0
        values.sort();

        // Add the joker to the best score now at the bottom
        values[12] += joker_count;

        Self::score_tally(&values)
    }

    fn score_tally(values: &[u8]) -> u32 {

        // Figure out the ordering of the hand
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
    fn cmp_part_1(&self, other: &Self) -> Ordering {
        let card_value_order = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];

        self.score_part1()
            .cmp(&other.score_part1())
            .then(self.cmp_by_card_value(other, &card_value_order))
    }

    fn cmp_part_2(&self, other: &Self) -> Ordering {
        let card_value_order = [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];

        self.score_part2()
            .cmp(&other.score_part2())
            .then(self.cmp_by_card_value(other, &card_value_order))
    }

    fn cmp_by_card_value(&self, other: &Self, card_value_order: &[char]) -> Ordering {
        let mut ordering = Ordering::Equal;
        for (a, b) in self.cards.chars().zip(other.cards.chars()) {
            let a = card_value_order.iter().position(|p| *p == a).unwrap();
            let b = card_value_order.iter().position(|p| *p == b).unwrap();

            ordering = ordering.then(a.cmp(&b));
        }

        ordering
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
        let mut hands = input
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(' ').unwrap();
                (left.parse::<Hand>().unwrap(), right.parse::<u32>().unwrap())
            })
            .collect_vec();

        hands.sort_by(|a, b| a.0.cmp_part_1(&b.0));

        let part_1_ans: u64 = hands
            .iter()
            .enumerate()
            .map(|(pos, (_hand, bid))| (pos as u64 + 1) * (*bid as u64))
            .sum();

        self.submit_part1(part_1_ans);

        hands.sort_by(|a, b| a.0.cmp_part_2(&b.0));

        let part_2_ans: u64 = hands
            .iter()
            .enumerate()
            .map(|(pos, (_hand, bid))| (pos as u64 + 1) * (*bid as u64))
            .sum();

        self.submit_part2(part_2_ans);

        assert_eq!(part_1_ans, 250946742);
        assert_eq!(part_2_ans, 251824095);

        Ok(())
    }
}
