use crate::line_segment::LineSegment;
use crate::types::EventType;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
pub struct Event<'a> {
    event_type: EventType,
    x: f64,
    y: f64,
    segment1: &'a LineSegment,
    segment2: Option<&'a LineSegment>,
}

impl<'a> Event<'a> {
    pub fn new(
        event_type: EventType,
        x: f64,
        y: f64,
        segment1: &'a LineSegment,
        segment2: Option<&'a LineSegment>,
    ) -> Self {
        Self {
            event_type,
            x,
            y,
            segment1,
            segment2,
        }
    }
}

fn compare(y1: f64, y2: f64, x1: f64, x2: f64) -> Option<Ordering> {
    let mut ord = y1.partial_cmp(&y2);
    if ord.is_none() || ord.unwrap_or(Ordering::Equal) == Ordering::Equal {
        ord = x1.partial_cmp(&x2);
    }
    ord
}

impl<'a> PartialOrd for Event<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        compare(self.y, other.y, self.x, other.x)
    }
}

impl<'a> Ord for Event<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ord) = compare(self.y, other.y, self.x, other.x) {
            ord
        } else {
            Ordering::Equal
        }
    }
}

impl<'a> Eq for Event<'a> {}

#[cfg(test)]
mod test {
    use crate::event::Event;
    use crate::intersection::{BruteForceInterSectionDirector, IntersectionDirector};
    use crate::line_segment::LineSegment;
    use crate::types::priority_queue::MinPriorityQueue;
    use crate::types::EventType;

    #[test]
    fn queue_test() {
        let segment1 = LineSegment::new(0.0, 1.0, 6.0, 4.0);
        let segment2 = LineSegment::new(-3.0, 4.0, 6.0, 1.0);
        let segment3 = LineSegment::new(0.0, 4.0, 6.0, 1.0);
        let segment4 = LineSegment::new(0.0, 10.0, 12.0, 20.0);
        let segment5 = LineSegment::new(20.0, 1.0, 12.0, 4.0);
        let segment6 = LineSegment::new(12.0, 4.0, 20.0, 1.0);

        let mut queue: MinPriorityQueue<Event> = MinPriorityQueue::new(usize::MAX);
        let segments = vec![segment1, segment2, segment3, segment4, segment5, segment6];
        for seg in segments.iter() {
            queue.append(Event::new(
                EventType::SegmentStart,
                seg.x1,
                seg.y1,
                &seg,
                None,
            ));
            queue.append(Event::new(
                EventType::SegmentEnd,
                seg.x2,
                seg.y2,
                &seg,
                None,
            ));
        }

        let director = BruteForceInterSectionDirector {};
        let intersections = director.execute(&segments);

        for intersect in intersections {
            if let Some(p) = intersect.get_intersection_point() {
                queue.append(Event::new(
                    EventType::InterSection,
                    p.x,
                    p.y,
                    intersect.segment1,
                    Some(intersect.segment2),
                ));
            }
        }

        let min = Event {
            event_type: EventType::SegmentStart,
            x: 0.0,
            y: 1.0,
            segment1: &LineSegment {
                x1: 0.0,
                y1: 1.0,
                x2: 6.0,
                y2: 4.0,
            },
            segment2: None,
        };
        let max = Event {
            event_type: EventType::SegmentEnd,
            x: 12.0,
            y: 20.0,
            segment1: &LineSegment {
                x1: 0.0,
                y1: 10.0,
                x2: 12.0,
                y2: 20.0,
            },
            segment2: None,
        };
        assert_eq!(queue.get_min_priority().unwrap(), &min);
        assert_eq!(queue.get_max_priority().unwrap(), &max);

        for elm in queue.get_min_value().iter() {
            println!("{:?}", elm);
        }
    }
}
