
pub type Coord = (isize, isize);


/// A special case of a polygon where the edges
/// are either horizontal or vertical but not slanted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LateralPolygon {
    /// The integer vertices of the polygon are listed in clockwise order.
    pub coordinates: Vec<Coord>,
}


impl LateralPolygon {
    pub fn new(coordinates: impl Iterator<Item=Coord>) -> Self {
        Self {
            coordinates: coordinates.collect(),
        }
    }

    pub fn bounding_box(&self) -> (Coord, Coord) {
        let x_min = self.coordinates.iter().map(|(x, _)| *x ).min().unwrap();
        let x_max = self.coordinates.iter().map(|(x, _)| *x ).max().unwrap();
        let y_min = self.coordinates.iter().map(|(_, y)| *y ).min().unwrap();
        let y_max = self.coordinates.iter().map(|(_, y)| *y ).max().unwrap();
        ((x_min, x_max), (y_min, y_max))
    }

    /// Return an iterator over the integer points that lie on the edges
    /// of the polygon but not at the vertices.
    pub fn edges(&self) -> impl Iterator<Item=Coord> + '_ {
        self.coordinates
        .as_slice()
        .windows(2)
        .flat_map(|window| {
            let (prev_x, prev_y) = window[0];
            let (curr_x, curr_y) = window[1];
            let range = {
                // Vertical line.
                if prev_x == curr_x {
                    // Don't include endpoints at all.
                    prev_y.min(curr_y)+1..prev_y.max(curr_y)
                }
                // Horizontal line.
                else if prev_y == curr_y {
                    // Don't include endpoints at all.
                    prev_x.min(curr_x)+1..prev_x.max(curr_x)
                } else {
                    panic!("not a standing/sleeping line");
                }
            };
            range
            .map(move |c| {
                if prev_x == curr_x {
                    (prev_x, c)
                } else {
                    (c, prev_y)
                }
            })
        })
    }

    /// Compute the area of the polygon using the [Shoelace formula].
    /// 
    /// [Shoelace formula]: https://www.theoremoftheday.org/GeometryAndTrigonometry/Shoelace/TotDShoelace.pdf
    pub fn shoelace_area(&self) -> usize {

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

    /// Get an iterator over all the unique integer points that form
    /// the boundary of the polygon.
    pub fn boundary(&self) -> impl Iterator<Item=Coord> + '_ {
        // Edges don't include endpoints so we gotta chain em.
        // This sweet optimization helps us avoid collecting everything
        // into a HashSet when we want to return a unique set of points
        // that form the boundary. In practise this changed my combined
        // runtime of day-18 from 19s to 1.9ms (i.e. a 10000x speed up).
        self
        .edges()
        .chain(self.coordinates.iter().copied())
    }
    /// Get the total number of unique integer points on the boundary of this polygon.
    pub fn perimeter(&self) -> usize {
        self.boundary().count()
    }

}

#[cfg(test)]
pub mod tests {
    use super::LateralPolygon;

    #[test]
    fn test_area_of_triangle() {
        let vertices = vec![(0, 0), (0, 5), (6, 0)];
        let triangle = LateralPolygon::new(vertices.into_iter());
        assert_eq!(triangle.shoelace_area(), 15);
    }

    #[test]
    fn test_area_of_square() {
        let vertices = vec![(0, 0), (0, 5), (5, 5), (5, 0)];
        let triangle = LateralPolygon::new(vertices.into_iter());
        assert_eq!(triangle.shoelace_area(), 25);
    }
}