use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cartesian {
    pub x: i32,
    pub y: i32,
}

impl Cartesian {
    pub const fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
}

impl Add<Self> for Cartesian {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::Output {x: self.x + other.x, y: self.y + other.y}
    }
}

impl AddAssign<Self> for Cartesian {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub<Self> for Cartesian {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::Output {x: self.x - other.x, y: self.y - other.y}
    }
}

impl SubAssign<Self> for Cartesian {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<i32> for Cartesian {
    type Output = Self;
    fn mul(self, other: i32) -> Self::Output {
        Self::Output {x: self.x * other, y: self.y * other}
    }
}
