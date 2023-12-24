use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use ahash::{AHashMap, AHashSet};
use itertools::Itertools;
use maths_rs::vec::Vec3;
use maths_rs::{line_vs_line, ray_vs_line_segment, Vec2d, Vec3d, Vec3f};
use std::collections::hash_set::IntoIter;
use std::ops::Range;
use std::str::FromStr;
use z3::ast::{Ast, Int};
create_solution!(Day24, 2023, 24);

impl Solution for Day24 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let (p1, p2) = solve(input, 200000000000000.0, 400000000000000.0);

        self.submit_part1(p1);
        self.submit_part2(p2);

        Ok(())
    }
}

#[derive(Debug)]
pub struct Hail<T> {
    pub orig: Vec3<T>,
    pub vec: Vec3<T>,
}

fn vec_from_str<T: FromStr>(input: &str) -> Vec3<T> {
    let mut coordinates = input.split(", ").map(|p| {
        p.trim()
            .parse::<T>()
            .unwrap_or_else(|_| panic!("cannot convert {p}"))
    });
    let v = Vec3 {
        x: coordinates.next().expect("must have x"),
        y: coordinates.next().expect("must have y"),
        z: coordinates.next().expect("must have z"),
    };

    v
}
fn parse<T: FromStr>(input: &str) -> Vec<Hail<T>> {
    let mut hail = Vec::new();
    for line in input.lines() {
        let (point, vector) = line
            .split_once(" @ ")
            .map(|(a, b)| (vec_from_str::<T>(a), vec_from_str::<T>(b)))
            .unwrap();

        hail.push(Hail {
            orig: point,
            vec: vector,
        });
    }

    hail
}

fn solve(input: &str, test_area_start: f64, test_area_end: f64) -> (usize, i64) {
    let p1 = count_collisions_in_test_area(&parse(input), test_area_start, test_area_end);
    let p2 = solve_p2(&parse(input));

    (p1, p2)
}

fn count_collisions_in_test_area(
    hail: &[Hail<f64>],
    test_area_start: f64,
    test_area_end: f64,
) -> usize {
    let mut count = 0;
    for (a, b) in hail.iter().tuple_combinations() {
        let a1 = Vec3d {
            x: a.orig.x,
            y: a.orig.y,
            z: 0.0,
        };

        let a2 = Vec3d {
            x: a.vec.x,
            y: a.vec.y,
            z: 0.0,
        };

        let b1 = Vec3d {
            x: b.orig.x,
            y: b.orig.y,
            z: 0.0,
        };

        let b2 = Vec3d {
            x: b.vec.x,
            y: b.vec.y,
            z: 0.0,
        };

        if let Some(intersection) = ray_vs_line_segment(a1, a2, b1, b1 + (b2 * 200000000000000.0)) {
            if intersection.x >= test_area_start
                && intersection.x <= test_area_end
                && intersection.y >= test_area_start
                && intersection.y <= test_area_end
            {
                count += 1;
            }
        }
    }

    count
}

// I 100% copied this solution from here:
// https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/24.rs
fn solve_p2(hail: &[Hail<i64>]) -> i64 {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let s = z3::Solver::new(&ctx);

    let (fx, fy, fz) = (
        Int::new_const(&ctx, "fx"),
        Int::new_const(&ctx, "fy"),
        Int::new_const(&ctx, "fz"),
    );
    let (fdx, fdy, fdz) = (
        Int::new_const(&ctx, "fdx"),
        Int::new_const(&ctx, "fdy"),
        Int::new_const(&ctx, "fdz"),
    );

    let zero = Int::from_i64(&ctx, 0);

    for (idx, h) in hail.iter().enumerate() {
        let (x, y, z) = (
            Int::from_i64(&ctx, h.orig.x),
            Int::from_i64(&ctx, h.orig.y),
            Int::from_i64(&ctx, h.orig.z),
        );
        let (dx, dy, dz) = (
            Int::from_i64(&ctx, h.vec.x),
            Int::from_i64(&ctx, h.vec.y),
            Int::from_i64(&ctx, h.vec.z),
        );

        let t = Int::new_const(&ctx, format!("t{idx}"));
        s.assert(&t.ge(&zero));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }

    assert_eq!(s.check(), z3::SatResult::Sat);
    let model = s.get_model().unwrap();

    let res = model.eval(&(&fx + &fy + &fz), true).unwrap();
    res.as_i64().unwrap()
}

#[cfg(test)]
mod test {
    use crate::year2023::day24::solve;

    #[test]
    fn year_2023_day_24_example() {
        let input = "19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3";

        let (p1, p2) = solve(input, 7.0, 27.0);
        assert_eq!(p1, 2);
        assert_eq!(p2, 47);
    }
}
