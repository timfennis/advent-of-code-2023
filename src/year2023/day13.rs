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

        println!("{pattern}");

        let flips = find_flip(&grid);
        assert_eq!(flips.len(), 1);
        dbg!(&flips);
        let p1_flip = flips
            .first()
            .expect("guaranteed to have at least 1 flip here");
        let part_1_ans = match p1_flip {
            (Some(x), None) => *x + 1,
            (None, Some(y)) => 100 * (*y + 1),
            _ => unreachable!("right!?"),
        };

        part_1 += part_1_ans;

        println!("=====part2======");
        let new_grid = correct_smudge(&grid);

        let new_flips = find_flip(&new_grid);
        dbg!(&new_flips);
        let excl_new_flips = new_flips
            .iter()
            .filter(|new_flip| *new_flip != p1_flip)
            .collect_vec();
        assert_eq!(excl_new_flips.len(), 1);
        let flip = **excl_new_flips
            .first()
            .expect("guaranteed to have at least 1 flip here");
        match flip {
            (Some(x), None) => {
                part_2 += x + 1;
            }
            (None, Some(y)) => {
                part_2 += 100 * (y + 1);
            }
            _ => unreachable!("right!?"),
        }
        println!();
    }

    (part_1, part_2)
}

fn correct_smudge(grid: &Grid) -> Grid {
    let mut smudge = None;

    for mirror_x in 0..(grid.width() - 1) as i64 {
        // find all objects that would be flipped along x
        let new_objects = grid
            .iter_objects()
            .filter(|(object, _)| object.x > mirror_x)
            .map(|(object, _)| object.mirror_between_x((mirror_x, mirror_x + 1)))
            .filter(|obj| obj.x >= 0 && obj.y >= 0)
            .collect::<HashSet<_>>();

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

        let diff = diff_a.union(&diff_b).copied().collect::<HashSet<_>>();

        if diff.len() == 1 {
            assert_eq!(smudge, None);
            smudge = diff.iter().next().copied();
        }
    }
    // println!("--y--\n");

    for mirror_y in 0..(grid.height() - 1) as i64 {
        // find all objects that would be flipped along y
        let new_objects = grid
            .iter_objects()
            .filter(|(object, _)| object.y > mirror_y)
            .map(|(object, _)| object.mirror_between_y((mirror_y, mirror_y + 1)))
            .filter(|obj| obj.x >= 0 && obj.y >= 0)
            .collect_vec();

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

        let diff = diff_a.union(&diff_b).copied().collect::<HashSet<Vec2>>();

        if diff.len() == 1 {
            assert_eq!(smudge, None);
            smudge = diff.iter().next().copied();
        }
    }

    let smudge = smudge.expect("must have 1 smudge");
    let mut grid = grid.clone();

    // If it already contains the smudge we remove it, otherwise we add it
    if grid.contains((smudge, '#')) {
        grid.remove((smudge, '#'));
    } else {
        grid.add((smudge, '#'));
    }

    grid
}

fn find_flip(grid: &Grid) -> Vec<(Option<i64>, Option<i64>)> {
    let mut flips = Vec::new();

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
        if diff_a.is_empty() && diff_b.is_empty() {
            // Add one because the example is 1 indexed
            flips.push((Some(mirror_x), None));
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
        if diff_a.is_empty() && diff_b.is_empty() {
            // Add one because the example is 1 indexed
            flips.push((None, Some(mirror_y)));
        }
    }

    flips
}

#[cfg(test)]
mod test {
    use crate::prelude::Grid;
    use crate::year2023::day13::{correct_smudge, find_flip, solve_puzzle};

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
