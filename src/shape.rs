
use crate::{usdf::{USDF, LineSegment, CircleSegment}, point::PathPoint};

pub struct Shape {
    pub usdfs: Vec<Box<dyn USDF>>,
}

impl Shape {
    pub fn new() -> Self {
        Self {
            usdfs: vec![],
        }
    }

    pub fn score(&self, point: PathPoint) -> f64 {
        let mut min = f64::MAX;
        for usdf in &self.usdfs {
            min = usdf.distance(point).sqrt().min(min);
        }
        min
    }
}

// shape generation functions
impl Shape {
    pub fn shape_lock() -> Self {
        let circle_usdf: CircleSegment = CircleSegment::new(
            PathPoint::new(0.5, 0.3),
            0.2,
            -90f64.to_radians(),
            145f64.to_radians(),
        );
        let left_line_usdf: LineSegment = LineSegment::new(
            circle_usdf.arc_start_point,
            PathPoint::new(0.3, 0.9),
        );
        let right_line_usdf: LineSegment = LineSegment::new(
            circle_usdf.arc_end_point,
            PathPoint::new(0.7, 0.9),
        );
        let bottom_line_usdf: LineSegment = LineSegment::new(
            PathPoint::new(0.3, 0.9),
            PathPoint::new(0.7, 0.9),
        );
        Self {
            usdfs: vec![Box::new(circle_usdf), Box::new(left_line_usdf), Box::new(right_line_usdf), Box::new(bottom_line_usdf)],
        }
    }
}
