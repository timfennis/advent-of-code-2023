use crate::create_solution;
use crate::prelude::{Grid, Vec2};
use crate::puzzle::{Answerable, Solution};
use ahash::{HashMap, HashSet};
use itertools::Itertools;
create_solution!(Day13, 2023, 13);
impl Solution for Day13 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let (part_1, part_2) = solve_puzzle(input);

        assert_eq!(part_1, 28651);
        self.submit_part1(part_1);

        assert_ne!(part_2, 17548); // too low
        self.submit_part2(part_2);

        Ok(())
    }
}

fn solve_puzzle(input: &str) -> (i64, i64) {
    let mut part_1 = 0;
    let mut part_2 = 0;

    for pattern in input.split("\n\n") {
        let grid = Grid::from_string(pattern, |c| c == '#');

        let fold = find_fold(&grid, 0);
        let part_1_ans = match fold {
            (Some(x), None) => x + 1,
            (None, Some(y)) => 100 * (y + 1),
            _ => unreachable!("there must be only 1 mirror possible"),
        };

        part_1 += part_1_ans;

        match find_fold(&grid, 1) {
            (Some(x), None) => {
                part_2 += x + 1;
            }
            (None, Some(y)) => {
                part_2 += 100 * (y + 1);
            }
            (None, None) => {
                println!("{}", grid);
                panic!("This should never happen");
            }
            _ => unreachable!("there must be only 1 mirror possible"),
        }
    }

    (part_1, part_2)
}

fn find_fold(grid: &Grid, error_margin: usize) -> (Option<i64>, Option<i64>) {
    for mirror_x in 0..(grid.width() - 1) as i64 {
        // find all objects that would be flipped along x
        let new_objects = grid
            .iter_objects()
            .filter_map(|(object, _)| {
                if object.x > mirror_x {
                    Some(Vec2 {
                        x: mirror_x - (object.x - mirror_x - 1),
                        y: object.y,
                    })
                } else {
                    None
                }
            })
            .filter(|obj| obj.x >= 0 && obj.y >= 0)
            .collect_vec();

        let new_xs = new_objects
            .iter()
            .map(|o| o.x)
            .dedup()
            .collect::<HashSet<_>>();

        // check if all mirrored objects exist on the other side
        let diff_a = new_objects
            .iter()
            .filter(|o| grid.object_at(o.x, o.y).is_none())
            .copied()
            .collect::<HashSet<Vec2>>();

        let diff_b = grid
            .iter_objects()
            .filter(|(o, _)| new_xs.contains(&o.x) && !new_objects.contains(o))
            .map(|(o, _)| *o)
            .collect::<HashSet<Vec2>>();

        // check if all mirrored objects exist on the other side
        if diff_a.len() + diff_b.len() == error_margin {
            return (Some(mirror_x), None);
        }
    }

    for mirror_y in 0..(grid.height() - 1) as i64 {
        // find all objects that would be flipped along y
        let new_objects = grid
            .iter_objects()
            .filter_map(|(object, _)| {
                if object.y > mirror_y {
                    Some(Vec2 {
                        x: object.x,
                        y: mirror_y - (object.y - mirror_y - 1),
                    })
                } else {
                    None
                }
            })
            .filter(|obj| obj.x >= 0 && obj.y >= 0)
            .collect_vec();

        // Check that the original grid does not have more objects than the mirrored grid
        let new_ys = new_objects
            .iter()
            .map(|o| o.y)
            .dedup()
            .collect::<HashSet<_>>();

        // check if all mirrored objects exist on the other side
        let diff_a = new_objects
            .iter()
            .filter(|o| grid.object_at(o.x, o.y).is_none())
            .copied()
            .collect::<HashSet<Vec2>>();

        let diff_b = grid
            .iter_objects()
            .filter(|(o, _)| new_ys.contains(&o.y) && !new_objects.contains(o))
            .map(|(o, _)| *o)
            .collect::<HashSet<Vec2>>();

        // check if all mirrored objects exist on the other side
        if diff_a.len() + diff_b.len() == error_margin {
            // Add one because the example is 1 indexed
            return (None, Some(mirror_y));
        }
    }

    // This case is not needed for our puzzle input
    (None, None)
}

#[cfg(test)]
mod test {
    use crate::prelude::Grid;
    use crate::year2023::day13::{find_fold, solve_puzzle};

    #[test]
    fn test_year_2023_day_13_example() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";

        let (part_1, part_2) = solve_puzzle(input);

        assert_eq!(405, part_1);
        assert_eq!(400, part_2);
    }

    #[test]
    fn test_year_2023_day_13_broken_input() {
        let input = ".##.#.#.####.#.
....#...#..#...
####..#.####.#.
####.#...##...#
######.#.##.#.#
#########..####
#..###..####...
#..#.#.##..##.#
#..####......##
.....##......##
.....##########";

        let (part_1, part_2) = solve_puzzle(input);

        assert_eq!(2, part_1);
        assert_eq!(10, part_2);
    }
}
