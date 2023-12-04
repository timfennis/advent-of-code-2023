#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Debug)]
pub struct Vec2 {
    pub y: i64,
    pub x: i64,
}

#[allow(dead_code)]
impl Vec2 {
    pub fn manhattan_distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
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

#[cfg(test)]
mod tests {
    use crate::prelude::Vec2;

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
}
