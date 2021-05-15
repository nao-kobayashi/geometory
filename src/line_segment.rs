use crate::line::Line;
use crate::sweep_line::SweepLine;
use crate::types::Point2D;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct LineSegment {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    comparator: SweepLine,
}

impl LineSegment {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            comparator: SweepLine::new(),
        }
    }

    pub fn to_line(&self) -> Line {
        Line::from_points(self.x1, self.y1, self.x2, self.y2)
    }

    /// ax + by + c <= 0
    /// ax + by + c >= 0
    pub fn intersects(&self, line: &Line) -> bool {
        let t1 = line.a * self.x1 + line.b * self.y1 + line.c;
        let t2 = line.a * self.x2 + line.b * self.y2 + line.c;
        t1 * t2 <= 0.0
    }

    pub fn intersects_segment(&self, segment: &LineSegment) -> bool {
        self.intersects(&segment.to_line()) && segment.intersects(&self.to_line())
    }

    pub fn get_intersection_point(&self, line: &Line) -> Option<Point2D> {
        if !self.intersects(line) {
            return None;
        }

        line.get_intersection_point(&self.to_line())
    }

    pub fn get_intersection_point_from_segment(&self, segment: &LineSegment) -> Option<Point2D> {
        if !self.intersects(&segment.to_line()) {
            return None;
        }

        segment.to_line().get_intersection_point(&self.to_line())
    }

    pub fn set_sweep_line(&mut self, line_y: f64) {
        self.comparator.set_y(line_y);
    }

    fn compare_by_line(&self, other: &LineSegment, line: &Line) -> Option<Ordering> {
        let p1 = self.to_line().get_intersection_point(&line);
        let p2 = other.to_line().get_intersection_point(&line);

        let x1 = if let Some(p) = p1 {
            p.x
        } else {
            self.x1
        };

        let x2 = if let Some(p) = p2 {
            p.x
        } else {
            self.x1
        };

        x1.partial_cmp(&x2)
    }

}

impl Hash for LineSegment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        f64_to_bytes(self.x1).hash(state);
        f64_to_bytes(self.y1).hash(state);
        f64_to_bytes(self.x2).hash(state);
        f64_to_bytes(self.y2).hash(state);
    }
}

fn f64_to_bytes(x: f64) -> [u8; 8] {
    let p = &x as *const f64 as *const [u8; 8];
    let b: [u8; 8] = unsafe { *p };
    //println!("{} => {:?}", x, b);
    b
}

impl PartialOrd for LineSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut ord = self.compare_by_line(other, self.comparator.get_sweep_line());
        if ord.is_none() || ord.unwrap_or(Ordering::Equal) == Ordering::Equal {
            ord = self.compare_by_line(other, self.comparator.get_below_line());
        }
        ord
    }
}

impl Ord for LineSegment {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ord) = self.partial_cmp(other) {
            ord
        } else {
            Ordering::Equal
        }
    }
}

impl Eq for LineSegment {}


#[cfg(test)]
mod tests {
    use crate::line::Line;
    use crate::line_segment::LineSegment;

    #[test]
    fn intersects_test() {
        let segment = LineSegment::new(0.0, 1.0, 6.0, 4.0);
        let line = Line::from_points(3.0, 6.0, 5.0, 0.0);
        let segment2 = LineSegment::new(3.0, 6.0, 5.0, 0.0);

        assert!(segment.intersects(&line));
        assert!(segment.intersects_segment(&segment2));
    }

    #[test]
    fn get_intersection_point_test() {
        let segment = LineSegment::new(0.0, 1.0, 6.0, 4.0);
        let line = Line::from_points(3.0, 6.0, 5.0, 0.0);
        let segment2 = LineSegment::new(3.0, 6.0, 5.0, 0.0);
        let segment3 = LineSegment::new(3.0, -10.0, 5.0, 2.0);
        let segment4 = LineSegment::new(0.0, 2.0, 6.0, 5.0);

        if let Some(point) = segment.get_intersection_point(&line) {
            println!("{:?}", point);
            assert_eq!(point.x, 4.0);
            assert_eq!(point.y, 3.0);
        } else {
            assert!(false);
        }

        if let Some(point) = segment.get_intersection_point_from_segment(&segment2) {
            println!("{:?}", point);
            assert_eq!(point.x, 4.0);
            assert_eq!(point.y, 3.0);
        } else {
            assert!(false);
        }

        assert!(!segment.intersects_segment(&segment3));
        assert!(segment
            .get_intersection_point_from_segment(&segment3)
            .is_some());
        assert!(segment
            .get_intersection_point_from_segment(&segment4)
            .is_none());
    }
}
