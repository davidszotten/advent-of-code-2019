use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(PartialEq, Eq, Default, Clone, Copy, Hash)]
pub struct Coor {
    pub x: i32,
    pub y: i32,
}

impl Coor {
    pub fn new(x: i32, y: i32) -> Self {
        Coor { x, y }
    }
}
impl fmt::Debug for Coor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Coor {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coor::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Coor {
    // type Output = Self;

    fn add_assign(&mut self, other: Self) {
        // Coor::new(self.x + other.x, self.y + other.y)
        *self = *self + other;
    }
}
