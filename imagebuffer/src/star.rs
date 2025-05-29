use std::f64::consts::PI;

/// Represents a star shape configuration
#[derive(Debug, Clone)]
pub struct Star {
    /// Center x-coordinate of the star
    pub x: f64,
    /// Center y-coordinate of the star
    pub y: f64,
    /// Outer radius of the star
    pub outer_radius: f64,
    /// Inner radius of the star
    pub inner_radius: f64,
    /// Number of points in the star
    pub points: usize,
    /// Rotation angle in radians
    pub rotation: f64,
}

impl Star {
    /// Creates a new star with the specified parameters
    pub fn new(x: f64, y: f64, outer_radius: f64, inner_radius: f64, points: usize) -> Self {
        Self {
            x,
            y,
            outer_radius,
            inner_radius,
            points,
            rotation: 0.0,
        }
    }

    /// Sets the rotation angle of the star in radians
    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }

    /// Calculates the vertices of the star
    pub fn calculate_vertices(&self) -> Vec<(f64, f64)> {
        let mut vertices = Vec::with_capacity(self.points * 2);
        let angle_step = PI / self.points as f64;

        for i in 0..self.points * 2 {
            let radius = if i % 2 == 0 {
                self.outer_radius
            } else {
                self.inner_radius
            };

            let angle = angle_step * i as f64 + self.rotation;
            let x = self.x + radius * angle.cos();
            let y = self.y + radius * angle.sin();
            vertices.push((x, y));
        }

        vertices
    }

    /// Returns an iterator over the star's edges
    pub fn edges(&self) -> StarEdgeIterator {
        StarEdgeIterator {
            vertices: self.calculate_vertices(),
            current: 0,
        }
    }
}

/// Iterator over the edges of a star
pub struct StarEdgeIterator {
    vertices: Vec<(f64, f64)>,
    current: usize,
}

impl Iterator for StarEdgeIterator {
    type Item = ((f64, f64), (f64, f64));

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.vertices.len() {
            return None;
        }

        let start = self.vertices[self.current];
        let end = if self.current == self.vertices.len() - 1 {
            self.vertices[0]
        } else {
            self.vertices[self.current + 1]
        };

        self.current += 1;
        Some((start, end))
    }
}

/// Utility functions for star-related calculations
pub mod utils {
    use super::*;

    /// Calculates the area of a star
    pub fn calculate_area(star: &Star) -> f64 {
        let vertices = star.calculate_vertices();
        let mut area = 0.0;

        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            area += vertices[i].0 * vertices[j].1;
            area -= vertices[j].0 * vertices[i].1;
        }

        area.abs() / 2.0
    }

    /// Checks if a point is inside the star
    pub fn contains_point(star: &Star, point: (f64, f64)) -> bool {
        let vertices = star.calculate_vertices();
        let mut inside = false;
        let mut j = vertices.len() - 1;

        for i in 0..vertices.len() {
            if ((vertices[i].1 > point.1) != (vertices[j].1 > point.1))
                && (point.0
                    < (vertices[j].0 - vertices[i].0) * (point.1 - vertices[i].1)
                        / (vertices[j].1 - vertices[i].1)
                        + vertices[i].0)
            {
                inside = !inside;
            }
            j = i;
        }

        inside
    }

    /// Calculates the perimeter of the star
    pub fn calculate_perimeter(star: &Star) -> f64 {
        star.edges()
            .map(|(start, end)| {
                let dx = end.0 - start.0;
                let dy = end.1 - start.1;
                (dx * dx + dy * dy).sqrt()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn test_star_creation() {
        let star = Star::new(0.0, 0.0, 10.0, 5.0, 5);
        assert_eq!(star.x, 0.0);
        assert_eq!(star.y, 0.0);
        assert_eq!(star.outer_radius, 10.0);
        assert_eq!(star.inner_radius, 5.0);
        assert_eq!(star.points, 5);
        assert_eq!(star.rotation, 0.0);
    }

    #[test]
    fn test_star_rotation() {
        let star = Star::new(0.0, 0.0, 10.0, 5.0, 5).with_rotation(FRAC_PI_2);
        assert_eq!(star.rotation, FRAC_PI_2);
    }

    #[test]
    fn test_vertices_count() {
        let star = Star::new(0.0, 0.0, 10.0, 5.0, 5);
        let vertices = star.calculate_vertices();
        assert_eq!(vertices.len(), 10); // 5 points * 2 vertices per point
    }

    #[test]
    fn test_edge_iterator() {
        let star = Star::new(0.0, 0.0, 10.0, 5.0, 5);
        let edges: Vec<_> = star.edges().collect();
        assert_eq!(edges.len(), 10);
    }

    #[test]
    fn test_point_containment() {
        let star = Star::new(0.0, 0.0, 10.0, 5.0, 5);
        assert!(utils::contains_point(&star, (0.0, 0.0))); // Center point
        assert!(!utils::contains_point(&star, (20.0, 20.0))); // Far outside
    }
}
