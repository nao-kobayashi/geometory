use crate::line_segment::LineSegment;
use crate::types::Point2D;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    segment1: &'a LineSegment,
    segment2: &'a LineSegment,
}

impl<'a> Intersection<'a> {
    pub fn new(segment1: &'a LineSegment, segment2: &'a LineSegment) -> Self {
        Self { segment1, segment2 }
    }

    pub fn get_intersection_point(&self) -> Option<Point2D> {
        self.segment1
            .get_intersection_point_from_segment(&self.segment2)
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.segment1 == other.segment1 && self.segment2 == other.segment2 {
            return true;
        } else if self.segment1 == other.segment2 && self.segment2 == other.segment1 {
            return true;
        }

        return false;
    }
}

impl<'a> Eq for Intersection<'a> {}

impl<'a> Hash for Intersection<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.segment1.hash(state);
        self.segment2.hash(state);
    }
}

pub trait IntersectionDirector {
    fn execute(segments: &[LineSegment]) -> Vec<InterSection>;
}

#[cfg(test)]
mod tests {
    use crate::line::Line;
    use crate::line_segment::LineSegment;
    use std::collections::HashSet;
    use crate::intersection::Intersection;

    #[test]
    fn hash_test() {
        let mut hash = HashSet::new();
        let segment1 = LineSegment::new(0.0, 1.0, 6.0, 4.0);
        let segment2 = LineSegment::new(3.0, 6.0, 5.0, 0.0);
        let segment3 = LineSegment::new(0.0, 1.0, 6.0, 4.0);
        let segment4 = LineSegment::new(3.0, 6.0, 5.0, 0.0);

        let intersection1 = Intersection::new(&segment1, &segment2);
        let intersection2 = Intersection::new(&segment3, &segment4);

        if !hash.contains(&intersection1) {
            hash.insert(intersection1);
        }

        if !hash.contains(&intersection2) {
            hash.insert(intersection2);
        }

        assert_eq!(hash.len(), 1);
    }
}