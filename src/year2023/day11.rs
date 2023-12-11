use crate::create_solution;
use crate::prelude::{Grid, Vec2};
use crate::puzzle::{Answerable, Solution};
use ahash::{HashMap, HashSet};
use itertools::Itertools;
create_solution!(Day11, 2023, 11);

impl Solution for Day11 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        self.submit_part1(
            get_positions(input, 2)
                .iter()
                .tuple_combinations()
                .map(|(a, b)| a.manhattan_distance(b))
                .sum::<u64>(),
        );
        self.submit_part2(
            get_positions(input, 1000000)
                .iter()
                .tuple_combinations()
                .map(|(a, b)| a.manhattan_distance(b))
                .sum::<u64>(),
        );

        Ok(())
    }
}

fn get_positions(input: &str, distance: i64) -> Vec<Vec2> {
    let grid = Grid::from_string(input, |c| c == '#');

    let mut empty_columns: HashSet<usize> = grid.x_range().collect();

    for col in grid.x_range() {
        for row in grid.y_range() {
            if grid.object_at(col as i64, row as i64) == Some('#') {
                empty_columns.remove(&col);
            }
        }
    }

    let mut cur_row = 0;
    let mut universe: Vec<Vec2> = Default::default();

    for (_, line) in input.lines().enumerate() {
        let mut cur_col = 0;
        let mut row_is_empty = true;
        for (col, ch) in line.chars().enumerate() {
            cur_col += if empty_columns.contains(&col) {
                distance
            } else {
                1
            };
            if ch == '#' {
                row_is_empty = false;
                universe.push(Vec2 {
                    x: cur_col,
                    y: cur_row,
                });
            }
        }

        cur_row += if row_is_empty { distance } else { 1 }
    }

    universe
}
