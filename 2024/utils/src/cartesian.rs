use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
pub const CARDINALS: [Cartesian; 4] = [
    Cartesian::UP,
    Cartesian::RIGHT,
    Cartesian::DOWN,
    Cartesian::LEFT
];
pub const DIRECTIONS: [Cartesian; 8] = [
    Cartesian::UP,
    Cartesian::UP_RIGHT,
    Cartesian::RIGHT,
    Cartesian::DOWN_RIGHT,
    Cartesian::DOWN,
    Cartesian::DOWN_LEFT,
    Cartesian::LEFT,
    Cartesian::UP_LEFT
];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cartesian {
    pub x: i32,
    pub y: i32,
}

impl Cartesian {

    pub const UP: Cartesian = Cartesian::new(-1, 0);
    pub const RIGHT: Cartesian = Cartesian::new(0, 1);
    pub const DOWN: Cartesian = Cartesian::new(1, 0);
    pub const LEFT: Cartesian = Cartesian::new(0, -1);
    pub const UP_LEFT: Cartesian = Cartesian::new(-1, -1);
    pub const UP_RIGHT: Cartesian = Cartesian::new(-1, 1);
    pub const DOWN_LEFT: Cartesian = Cartesian::new(1, -1);
    pub const DOWN_RIGHT: Cartesian = Cartesian::new(1, 1);
    pub const fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
    pub fn quarter_turn(self, counter_clockwise: bool) -> Cartesian {
        match (self, counter_clockwise) {
            (Cartesian::UP, false) => Cartesian::RIGHT,
            (Cartesian::UP, true) => Cartesian::LEFT,
            (Cartesian::RIGHT, false) => Cartesian::DOWN,
            (Cartesian::RIGHT, true) => Cartesian::UP,
            (Cartesian::DOWN, false) => Cartesian::LEFT,
            (Cartesian::DOWN, true) => Cartesian::RIGHT,
            (Cartesian::LEFT, false) => Cartesian::UP,
            (Cartesian::LEFT, true) => Cartesian::DOWN,
            (dir, _) => dir
        }
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
