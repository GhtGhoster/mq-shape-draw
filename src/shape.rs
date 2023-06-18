
use std::f64::consts::{PI, TAU};

use crate::{usdf::{USDF, LineSegment, CircleSegment}, point::PathPoint};

pub struct Shape {
    pub usdfs: Vec<Box<dyn USDF>>,
    pub domain: Option<(PathPoint, PathPoint)>,
}

impl Shape {
    pub fn score(&self, point: PathPoint) -> f64 {
        let actual_point = if let Some(old_domain) = self.domain {
            point.lerp_from_normalized_domain(old_domain)
        } else {
            point
        };
        let mut min = f64::MAX;
        for usdf in &self.usdfs {
            // todo: choose a proper smoothing function (sqrt works pretty well so far)
            min = usdf.distance(actual_point).sqrt().min(min);
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
            domain: Some((PathPoint::new(0.3, 0.1), PathPoint::new(0.7, 0.9))),
        }
    }

    pub fn shape_devil() -> Self {
        let circle_usdf: CircleSegment = CircleSegment::new(
            PathPoint::new(0.5, 0.5),
            0.5,
            0.0,
            PI,
        );

        let left_top = PathPoint::from_angle((PI/2.0) + (TAU*0.4));
        let right_top = PathPoint::from_angle((PI/2.0) + (TAU*0.6));
        let left_bottom = PathPoint::from_angle((PI/2.0) + (TAU*0.2));
        let right_bottom = PathPoint::from_angle((PI/2.0) - (TAU*0.2));

        let left_tall_line_usdf: LineSegment = LineSegment::new(
            PathPoint::new(0.5, 1.0),
            left_top,
        );
        let right_tall_line_usdf: LineSegment = LineSegment::new(
            PathPoint::new(0.5, 1.0),
            right_top,
        );
        let left_wide_line_usdf: LineSegment = LineSegment::new(
            left_top,
            right_bottom,
        );
        let right_wide_line_usdf: LineSegment = LineSegment::new(
            right_top,
            left_bottom,
        );
        let bottom_line_usdf: LineSegment = LineSegment::new(
            left_bottom,
            right_bottom,
        );
        Self {
            usdfs: vec![
                Box::new(circle_usdf),
                Box::new(left_tall_line_usdf),
                Box::new(right_tall_line_usdf),
                Box::new(left_wide_line_usdf),
                Box::new(right_wide_line_usdf),
                Box::new(bottom_line_usdf)
            ],
            domain: None,
        }
    }

    pub fn shape_water() -> Self {
        let left_line_usdf: LineSegment = LineSegment::new(
            PathPoint::new(0.0, 0.0),
            PathPoint::new(0.5, 1.0),
        );
        let right_line_usdf: LineSegment = LineSegment::new(
            PathPoint::new(1.0, 0.0),
            PathPoint::new(0.5, 1.0),
        );
        let top_line_usdf: LineSegment = LineSegment::new(
            PathPoint::new(1.0, 0.0),
            PathPoint::new(0.0, 0.0),
        );
        Self {
            usdfs: vec![
                Box::new(left_line_usdf),
                Box::new(right_line_usdf),
                Box::new(top_line_usdf),
            ],
            domain: None,
        }
    }

    pub fn shape_fire() -> Self {
        let left_line_usdf: LineSegment = LineSegment::new(
            PathPoint::new(0.0, 1.0),
            PathPoint::new(0.5, 0.0),
        );
        let right_line_usdf: LineSegment = LineSegment::new(
            PathPoint::new(1.0, 1.0),
            PathPoint::new(0.5, 0.0),
        );
        let bottom_line_usdf: LineSegment = LineSegment::new(
            PathPoint::new(0.0, 1.0),
            PathPoint::new(1.0, 1.0),
        );
        Self {
            usdfs: vec![
                Box::new(left_line_usdf),
                Box::new(right_line_usdf),
                Box::new(bottom_line_usdf),
            ],
            domain: None,
        }
    }

    pub fn shape_earth() -> Self {
        let mut tmp = Self::shape_water();
        tmp.usdfs.push(Box::new(LineSegment::new(
            PathPoint::new(0.0, 0.6),
            PathPoint::new(1.0, 0.6),
        )));
        tmp
    }

    pub fn shape_air() -> Self {
        let mut tmp = Self::shape_fire();
        tmp.usdfs.push(Box::new(LineSegment::new(
            PathPoint::new(0.0, 0.4),
            PathPoint::new(1.0, 0.4),
        )));
        tmp
    }
}
