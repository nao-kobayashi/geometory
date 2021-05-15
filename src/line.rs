use crate::types::Point2D;

#[derive(Debug, Clone, PartialEq)]
/// ax + by + c = 0
pub struct Line {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Line {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    /// ax1 + by1 + c = 0
    /// ax2 + by2 + c = 0
    ///
    /// a = y2 - y1
    /// b = x1 - x2
    /// c = y1(x2 - x1) - x1(y2 - y1)
    pub fn from_points(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        let dx = x2 - x1;
        let dy = y2 - y1;
        Line::new(dy, -dx, (dx * y1) - (dy * x1))
    }

    /// a1x + b1y +c1 = 0
    /// a2x + b2y +c2 = 0
    ///
    /// x = (b1c2 - b2c1) / (a1b2 - a2b1)
    /// y = (a2c1 - a1c2) / (a1b2 - a2b1)
    pub fn get_intersection_point(&self, line: &Line) -> Option<Point2D> {
        let d = self.a * line.b - line.a * self.b;
        if d == 0.0 {
            return None;
        }

        let x = (self.b * line.c - line.b * self.c) / d;
        let y = (line.a * self.c - self.a * line.c) / d;

        Some(Point2D { x, y })
    }
}

#[cfg(test)]
mod tests {
    use crate::line::Line;

    #[test]
    fn get_intersection_point_test() {
        let l1 = Line::from_points(0.0, 1.0, 6.0, 4.0);
        let l2 = Line::from_points(3.0, 6.0, 5.0, 0.0);

        if let Some(point) = l1.get_intersection_point(&l2) {
            assert_eq!(point.x, 4.0);
            assert_eq!(point.y, 3.0);
        } else {
            assert!(false);
        }
    }
}
