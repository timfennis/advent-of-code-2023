use crate::prelude::Vec2;
use std::fmt::{Display, Formatter, Write};
use std::ops::Range;
use std::slice::Iter;

type GridObject = (Vec2, char);

//TODO: no clone
#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub objects: Vec<GridObject>,
}

impl IntoIterator for Grid {
    type Item = GridObject;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.into_iter()
    }
}

#[allow(dead_code)]
impl Grid {
    #[allow(dead_code)]
    pub fn from_dimension(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            objects: Default::default(),
        }
    }

    pub fn from_string(input: &str, predicate: fn(char) -> bool) -> Self {
        let width = input
            .lines()
            .next()
            .expect("input does not have a first line")
            .len();

        let height = input.lines().count();

        assert!(
            input.lines().all(|line| line.len() == width),
            "all lines must be the same length"
        );

        let objects = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, char)| ((x, y).into(), char))
            })
            .filter(|(_pos, val)| predicate(*val))
            .collect();

        Grid {
            width,
            height,
            objects,
        }
    }

    /// Returns whether a given point is within the bounds of this grid
    pub fn in_bound(&self, point: Vec2) -> bool {
        point.y < 0
            || point.x < 0
            || point.x >= self.width() as i64
            || point.y >= self.height() as i64
    }
    #[allow(dead_code)]
    pub fn neighbours4(&self, pos: impl Into<Vec2>) -> Vec<Vec2> {
        let Vec2 { x, y } = pos.into();
        debug_assert!(x < self.width as i64);
        debug_assert!(y < self.height as i64);

        vec![
            Vec2 { x, y: y - 1 },
            Vec2 { x: x - 1, y },
            Vec2 { x: x + 1, y },
            Vec2 { x, y: y + 1 },
        ]
        .into_iter()
        .filter(|p| p.x < self.width as i64 && p.x >= 0 && p.y >= 0 && p.y < self.height as i64)
        .collect()
    }
    pub fn neighbours8(&self, pos: impl Into<Vec2>) -> Vec<Vec2> {
        let Vec2 { x, y } = pos.into();
        debug_assert!(x < self.width as i64);
        debug_assert!(y < self.height as i64);

        vec![
            Vec2 { x: x - 1, y: y - 1 },
            Vec2 { x, y: y - 1 },
            Vec2 { x: x + 1, y: y - 1 },
            Vec2 { x: x - 1, y },
            Vec2 { x: x + 1, y },
            Vec2 { x: x - 1, y: y + 1 },
            Vec2 { x, y: y + 1 },
            Vec2 { x: x + 1, y: y + 1 },
        ]
        .into_iter()
        .filter(|p| p.x < self.width as i64 && p.y < self.height as i64)
        .collect()
    }

    pub fn get_object(&self, pos: &Vec2) -> Option<char> {
        self.objects
            .iter()
            .find(|(op, _c)| *op == *pos)
            .map(|(_, c)| c)
            .copied()
    }
    pub fn object_at(&self, x: i64, y: i64) -> Option<char> {
        self.objects
            .iter()
            .find(|(op, _c)| op.x == x && op.y == y)
            .map(|(_, c)| c)
            .copied()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn x_range(&self) -> Range<usize> {
        0..self.width
    }
    pub fn y_range(&self) -> Range<usize> {
        0..self.height
    }

    pub fn iter_objects(&self) -> Iter<'_, GridObject> {
        self.objects.iter()
    }

    pub fn contains(&self, obj: GridObject) -> bool {
        self.objects.iter().any(|o| *o == obj)
    }

    pub fn remove(&mut self, obj: GridObject) {
        self.objects.retain(|o| *o != obj);
    }

    pub fn add(&mut self, obj: GridObject) {
        self.objects.push(obj);
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(ch) = self.object_at(x as i64, y as i64) {
                    f.write_char(ch)?
                } else {
                    f.write_char('.')?
                }
            }
            f.write_char('\n')?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_string() {
        let grid = Grid::from_string("x....\n.....\n..y..\n.....\n....z", |c| c != '.');

        assert_eq!(grid.width, 5);
        assert_eq!(grid.height, 5);

        assert!(grid.objects.contains(&(Vec2 { x: 0, y: 0 }, 'x')));
        println!("{:#?}", grid.objects);
        assert!(grid.objects.contains(&(Vec2 { x: 2, y: 2 }, 'y')));
        assert!(grid.objects.contains(&(Vec2 { x: 4, y: 4 }, 'z')));
    }

    #[test]
    fn neighbours8() {
        let grid = Grid::from_dimension(100, 100);

        let locs = grid.neighbours8((50, 50));
        assert_eq!(
            locs,
            vec![
                Vec2 { x: 49, y: 49 },
                Vec2 { x: 50, y: 49 },
                Vec2 { x: 51, y: 49 },
                Vec2 { x: 49, y: 50 },
                Vec2 { x: 51, y: 50 },
                Vec2 { x: 49, y: 51 },
                Vec2 { x: 50, y: 51 },
                Vec2 { x: 51, y: 51 },
            ]
        );

        let locs = grid.neighbours8((99, 99));

        assert_eq!(
            locs,
            vec![
                Vec2 { x: 98, y: 98 },
                Vec2 { x: 99, y: 98 },
                Vec2 { x: 98, y: 99 },
            ]
        );
    }
}
