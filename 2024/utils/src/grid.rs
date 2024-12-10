use crate::cartesian::Cartesian;

pub struct Grid<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn is_in_bounds(&self, coordinates: Cartesian) -> bool {
        coordinates.x >= 0 && coordinates.x < self.height as i32
        && coordinates.y >= 0 && coordinates.y < self.width as i32
    }
    pub fn get_data(&self) -> &Vec<Vec<T>> {
        &self.data
    }
    pub fn get_coordinate(&self, coordinates: Cartesian) -> Option<T>
    where T: Clone {
        if self.is_in_bounds(coordinates) {
            Some(self.data[coordinates.x as usize][coordinates.y as usize].clone())
        } else {
            None
        }
    }
    pub fn set_coordinate(&mut self, coordinates: Cartesian, value: T) {
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
        Self {data, width, height}
    }
}
