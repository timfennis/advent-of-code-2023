use std::fmt::{Display, Formatter};

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Debug, Hash, Copy)]
pub struct Vec2 {
    pub y: i64,
    pub x: i64,
}

#[allow(dead_code)]
impl Vec2 {
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn manhattan_distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    /// Create a new Vec2 moved in the given direction
    /// Down is considered to be a positive y value
    pub fn move_dir(&self, dir: Direction) -> Self {
        match dir {
            Direction::Right => Vec2 {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Vec2 {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Vec2 {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Up => Vec2 {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl From<(i64, i64)> for Vec2 {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Direction, Vec2};

    #[test]
    fn from_tuple() {
        assert_eq!(Vec2 { x: 1, y: 2 }, (1, 2).into());
        assert_eq!(Vec2 { x: 1, y: 2 }, (1usize, 2usize).into());
        assert_eq!(Vec2 { x: 1, y: 2 }, (1i64, 2i64).into());
    }
    #[test]
    fn manhattan_distance() {
        let a: Vec2 = (10, 10).into();
        let b: Vec2 = (100, 100).into();

        assert_eq!(a.manhattan_distance(&b), 180);
    }

    #[test]
    fn move_dir() {
        let o = Vec2::origin();
        assert_eq!(o.move_dir(Direction::Down), Vec2 { x: 0, y: 1 });
    }
}
