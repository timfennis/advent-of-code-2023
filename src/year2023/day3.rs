use std::collections::{HashMap, HashSet};
use crate::{create_solution, Solution};
use crate::puzzle::Answerable;

create_solution!(Day3, 2023, 3);
impl Solution for Day3 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
//         let input = "467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..";


        let all = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let width = all.iter().next().unwrap().len();
        println!("width: {width}");
        // for line in input.lines() {
        //     println!("{line}");
        // }

        let mut gears: HashMap<(i32, i32), Vec<_>> = HashMap::new();

        let mut sum = 0;
        let mut buf = String::new();
        let mut last_symbol = '.';
        for (idx, char) in input.chars().enumerate() {
            match char {
                c if c.is_ascii_digit() => {
                    buf.push(c);
                }
                c => {
                    if buf.is_empty() {
                        continue;
                    }

                    let x = (idx % (width+1)) as i32;
                    let y = (idx / (width+1)) as i32;
                    let x = x - 1;

                    // println!("----------idx:{idx} x:{x},y:{y} {buf}");
                    let mut valid = false;

                    let offsets = vec![
                        vec![],
                        vec![
                            (-1, -1), (-1, 0), (-1, 1),
                            (0, -1), (0, 1),
                            (1, -1), (1, 0), (1, 1),
                        ],
                        vec![
                            (-1, -2), (-1, -1), (-1, 0), (-1, 1),
                            (0, -2), (0, 1),
                            (1, -2), (1, -1), (1, 0), (1, 1),
                        ],
                        vec![
                            (-1, -3), (-1, -2), (-1, -1), (-1, 0), (-1, 1),
                            (0, -3), (0, 1),
                            (1, -3), (1, -2), (1, -1), (1, 0), (1, 1),
                        ],
                    ];
                    let mut might_be_gear = None;
                    for (dy, dx) in offsets.get(buf.len()).unwrap() {
                        let (ny, nx) = (y + dy, x + dx);
                        if ny < 0 || nx < 0 {
                            continue;
                        }
                        let neighbour: Option<char> = all.get(ny as usize)
                            .and_then(|r| r.get(nx as usize).copied());

                        println!("idx: {idx} buf:{buf} off:{dy},{dx} new:{ny},{nx} nb: {:#?}", neighbour);
                        match neighbour {
                            Some('.') => {}
                            Some('\n') => {}
                            Some(c) if c.is_ascii_digit() => {}
                            None => {}
                            Some(c) => {
                                if c == '*' {
                                    assert_eq!(might_be_gear, None);
                                    might_be_gear = Some((ny, nx));
                                }
                                //SYMBOL!
                                valid = true;
                            }
                        }
                    }
                    if !buf.is_empty() && valid {
                        println!("--- line: {y} columns: {x} valid: {buf}");
                        let current_number = buf.parse::<u32>().unwrap();
                        sum += current_number;
                        if let Some(coordinates) = might_be_gear  {
                            gears.entry(coordinates).or_insert(Vec::new()).push(current_number)
                        }

                    } else {
                        println!("--- line: {y} columns: {x} invalid: {buf}");
                    }

                    buf.clear();
                }
            }
        }

        self.submit_part1(sum);

        assert_ne!(sum, 541192);
        assert_eq!(sum, 537832);

        let mut p2 = 0;
        for (_, nums) in gears {
            assert!(nums.len() <= 2);
            if nums.len() == 2 {

                let product = nums[0] * nums[1];
                p2 += product;
            }
        }


        self.submit_part2(p2);

        Ok(())
    }
}
