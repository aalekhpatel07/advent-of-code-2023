pub type Coord = (isize, isize);


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polygon {
    // Assume they are listed in clockwise order.
    coordinates: Vec<Coord>
}


impl Polygon {
    pub fn new(coordinates: impl Iterator<Item=Coord>) -> Self {
        Self {
            coordinates: coordinates.collect()
        }
    }

    pub fn area(&self) -> usize {

        let mut coords_cycle = self.coordinates.clone();
        coords_cycle.push(coords_cycle[0]);

        let mut left = 0;
        let mut right = 0;

        for i in 0..self.coordinates.len() {
            let prev_coord = coords_cycle[i];
            let current_coord = coords_cycle[i+1];

            left += prev_coord.0 * current_coord.1;
            right += prev_coord.1 * current_coord.0;
        }

        left.abs_diff(right) / 2
    }
}

#[cfg(test)]
pub mod tests {
    use super::Polygon;

    #[test]
    fn test_area_of_triangle() {
        let vertices = vec![(0, 0), (0, 5), (6, 0)];
        let triangle = Polygon::new(vertices.into_iter());
        assert_eq!(triangle.area(), 15);
    }

    #[test]
    fn test_area_of_square() {
        let vertices = vec![(0, 0), (0, 5), (5, 5), (5, 0)];
        let triangle = Polygon::new(vertices.into_iter());
        assert_eq!(triangle.area(), 25);
    }
}