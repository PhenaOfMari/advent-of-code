use std::fmt::Display;
use crate::cartesian::Cartesian;

pub struct Grid<T> {
    data: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    pub fn new(height: usize, width: usize, initial_value: T) -> Self
    where T: Clone {
        let data = vec![vec![initial_value; width]; height];
        Self {data, height, width}
    }
    pub fn data(&self) -> &Vec<Vec<T>> {
        &self.data
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn is_in_bounds(&self, coordinates: Cartesian) -> bool {
        coordinates.x >= 0 && coordinates.x < self.height as i32
        && coordinates.y >= 0 && coordinates.y < self.width as i32
    }
    pub fn get(&self, coordinates: Cartesian) -> Option<T>
    where T: Clone {
        if self.is_in_bounds(coordinates) {
            Some(self.data[coordinates.x as usize][coordinates.y as usize].clone())
        } else {
            None
        }
    }
    pub fn set(&mut self, coordinates: Cartesian, value: T) {
        if self.is_in_bounds(coordinates) {
            self.data[coordinates.x as usize][coordinates.y as usize] = value;
        }
    }
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item=Vec<T>>>(iter: I) -> Self {
        let mut data = Vec::new();
        for item in iter {
            data.push(item);
        }
        let height = data.len();
        let width = data[0].len();
        Self {data, height, width}
    }
}

impl<T> Display for Grid<T>
where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = Ok(());
        for i in 0..self.height {
            for j in 0..self.width {
                if result.is_ok() {
                    result = self.data[i][j].fmt(f);
                }
            }
            if result.is_ok() {
                result = write!(f, "\n");
            }
        }
        result
    }
}

