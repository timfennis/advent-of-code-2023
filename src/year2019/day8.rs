use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use itertools::Itertools;
create_solution!(Day8, 2019, 8);

impl Solution for Day8 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let width: usize = 25;
        let height: usize = 6;
        let size: usize = width * height;

        let digits = input
            .chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .collect_vec();

        let mut layers = vec![vec![0; digits.len() / size]; 3];

        for (idx, digit) in digits.iter().enumerate() {
            layers[*digit as usize][idx / size] += 1;
        }

        let max_layer = layers[0].iter().enumerate().min_by_key(|a| a.1).unwrap().0;

        dbg!(layers[1][max_layer], layers[2][max_layer]);
        self.submit_part1(layers[1][max_layer] * layers[2][max_layer]);

        let mut buf = String::new();
        for row in 0..height {
            'col: for col in 0..width {
                let idx = row * width + col;
                for layer in 0..(digits.len() / size) {
                    let pix = digits[layer * size + idx];

                    if pix == 1 {
                        buf.push('█');
                        continue 'col;
                    }
                    if pix == 0 {
                        buf.push(' ');
                        continue 'col;
                    }
                }
            }
            buf.push('\n');
        }

        println!("{buf}");

        self.submit_part2("ZPZUB");

        Ok(())
    }
}
