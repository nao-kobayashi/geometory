use crate::line_segment::LineSegment;
use crate::types::Point2D;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct InterSection<'a> {
    pub segment1: &'a LineSegment,
    pub segment2: &'a LineSegment,
}

impl<'a> InterSection<'a> {
    pub fn new(segment1: &'a LineSegment, segment2: &'a LineSegment) -> Self {
        Self { segment1, segment2 }
    }

    pub fn get_intersection_point(&self) -> Option<Point2D> {
        self.segment1
            .get_intersection_point_from_segment(&self.segment2)
    }
}

impl<'a> PartialEq for InterSection<'a> {
    fn eq(&self, other: &Self) -> bool {
        //println!("{:?} {:?}", &self, &other);
        if self.segment1 == other.segment1 && self.segment2 == other.segment2 {
            return true;
        } else if self.segment1 == other.segment2 && self.segment2 == other.segment1 {
            return true;
        }

        return false;
    }
}

impl<'a> Eq for InterSection<'a> {}

impl<'a> Hash for InterSection<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.segment1.hash(state);
        self.segment2.hash(state);
    }
}

pub trait IntersectionDirector {
    fn execute<'a>(&self, segments: &'a [LineSegment]) -> Vec<InterSection<'a>>;
}

pub struct BruteForceInterSectionDirector {}
impl IntersectionDirector for BruteForceInterSectionDirector {
    fn execute<'a>(&self, segments: &'a [LineSegment]) -> Vec<InterSection<'a>> {
        let mut intersections = Vec::new();
        for i in 0..segments.len() {
            let seg1 = &segments[i];
            for j in i + 1..segments.len() {
                let seg2 = &segments[j];
                if seg1.intersects_segment(seg2) {
                    //println!("i {} j {}", i, j);
                    intersections.push(InterSection::new(seg1, seg2));
                }
            }
        }

        intersections
    }
}

#[cfg(test)]
mod tests {
    use crate::intersection::{BruteForceInterSectionDirector, InterSection, IntersectionDirector};
    use crate::line_segment::LineSegment;
    use std::collections::HashSet;

    #[test]
    fn bruteforce_test() {
        let segment1 = LineSegment::new(0.0, 1.0, 6.0, 4.0);
        let segment2 = LineSegment::new(-3.0, 4.0, 6.0, 1.0);
        let segment3 = LineSegment::new(0.0, 4.0, 6.0, 1.0);
        let segment4 = LineSegment::new(0.0, 10.0, 12.0, 20.0);
        let segment5 = LineSegment::new(20.0, 1.0, 12.0, 4.0);
        let segment6 = LineSegment::new(12.0, 4.0, 20.0, 1.0);

        let segments = vec![segment1, segment2, segment3, segment4, segment5, segment6];
        let director = BruteForceInterSectionDirector {};
        let intersections = director.execute(&segments);
        assert_eq!(intersections.len(), 4);

        assert_eq!(intersections[0].segment1, &segments[0]);
        assert_eq!(intersections[0].segment2, &segments[1]);
        assert_eq!(intersections[1].segment1, &segments[0]);
        assert_eq!(intersections[1].segment2, &segments[2]);
        assert_eq!(intersections[2].segment1, &segments[1]);
        assert_eq!(intersections[2].segment2, &segments[2]);
        assert_eq!(intersections[3].segment1, &segments[4]);
        assert_eq!(intersections[3].segment2, &segments[5]);
    }

    #[test]
    fn hash_test() {
        let mut hash = HashSet::new();
        let segment1 = LineSegment::new(0.0, 1.0, 6.0, 4.0);
        let segment2 = LineSegment::new(3.0, 6.0, 5.0, 0.0);
        let segment3 = LineSegment::new(0.0, 1.0, 6.0, 4.0);
        let segment4 = LineSegment::new(3.0, 6.0, 5.0, 0.0);
        let segment5 = LineSegment::new(3.0, 6.0, 5.0, 0.0);
        let segment6 = LineSegment::new(0.0, 1.0, 6.0, 4.0);

        let intersection1 = InterSection::new(&segment1, &segment2);
        let intersection2 = InterSection::new(&segment3, &segment4);
        let intersection3 = InterSection::new(&segment5, &segment6);

        assert_eq!(intersection1, intersection3);

        if !hash.contains(&intersection1) {
            hash.insert(intersection1);
        }

        if !hash.contains(&intersection2) {
            hash.insert(intersection2);
        }

        if !hash.contains(&intersection3) {
            hash.insert(intersection3);
        }

        assert_eq!(hash.len(), 2);
    }
}
