#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Debug)]
pub struct Vec2 {
    pub y: i64,
    pub x: i64,
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
