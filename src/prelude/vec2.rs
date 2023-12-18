use std::fmt::{Display, Formatter};

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Debug, Hash, Copy)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

#[allow(dead_code)]
impl Vec2 {
    pub fn from(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn manhattan_distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    /// Create a new Vec2 moved in the given direction
    /// Down is considered to be a positive y value
    pub fn move_dir(&self, dir: Direction) -> Self {
        self.move_dir_dist(dir, 1)
    }

    #[inline]
    pub fn move_dir_dist(&self, dir: Direction, dist: i64) -> Self {
        match dir {
            Direction::Right => Vec2 {
                x: self.x + dist,
                y: self.y,
            },
            Direction::Down => Vec2 {
                x: self.x,
                y: self.y + dist,
            },
            Direction::Left => Vec2 {
                x: self.x - dist,
                y: self.y,
            },
            Direction::Up => Vec2 {
                x: self.x,
                y: self.y - dist,
            },
        }
    }

    pub fn mirror_between_x(&self, (x1, x2): (i64, i64)) -> Self {
        Self {
            x: (x1 + x2) - self.x,
            y: self.y,
        }
    }
    pub fn mirror_between_y(&self, (y1, y2): (i64, i64)) -> Self {
        Self {
            y: (y1 + y2) - self.y,
            x: self.x,
        }
    }
    /// Mirror over an imaginary axis that sits at the end of the value of x_axis
    pub fn mirror_x(&self, x_axis: i64) -> Self {
        self.mirror_between_x((x_axis, x_axis))
    }

    /// Mirror over an imaginary axis that sits at the end of the value of y_axis
    pub fn mirror_y(&self, y_axis: i64) -> Self {
        self.mirror_between_x((y_axis, y_axis))
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }

    pub fn turn_right(&self) -> Direction {
        self.turn_left().turn_left().turn_left()
    }
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

    #[test]
    fn mirror() {
        assert_eq!(
            Vec2::from(4, 10),
            Vec2::from(5, 10).mirror_between_x((4, 5))
        );
        assert_eq!(
            Vec2::from(3, 10),
            Vec2::from(6, 10).mirror_between_x((4, 5))
        );
        assert_eq!(
            Vec2::from(2, 10),
            Vec2::from(7, 10).mirror_between_x((4, 5))
        );

        assert_eq!(
            Vec2::from(4, 10).mirror_between_x((4, 5)),
            Vec2::from(5, 10)
        );
        assert_eq!(
            Vec2::from(3, 10).mirror_between_x((4, 5)),
            Vec2::from(6, 10)
        );
        assert_eq!(
            Vec2::from(2, 10).mirror_between_x((4, 5)),
            Vec2::from(7, 10)
        );
    }
}
