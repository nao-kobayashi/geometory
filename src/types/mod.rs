pub mod priority_queue;

#[derive(Clone, Debug)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventType {
    SegmentStart,
    SegmentEnd,
    InterSection,
}
