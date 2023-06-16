
// will only implement simple arguemnt based circle and line segments
// in order to make serialization easy

use std::f64::consts::{PI, TAU};

use crate::point::PathPoint;

// sources:
// https://iquilezles.org/articles/distfunctions2d/
// https://stackoverflow.com/questions/12234574/calculating-if-an-angle-is-between-two-angles

// unsigned distance function
pub trait USDF {
    fn distance(&self, point: PathPoint) -> f64;
}

// a.k.a Circular arc
pub struct CircleSegment {
    center: PathPoint,
    radius: f64,
    facing_angle: f64,
    angle_spread: f64,
}

impl USDF for CircleSegment {
    fn distance(&self, point: PathPoint) -> f64 {
        let relative_point = point-self.center;
        let point_angle = relative_point.angle();
        let angle_diff = (self.facing_angle - point_angle + PI + TAU) % TAU - PI;
        if angle_diff < -self.angle_spread {
            let arc_start_angle = self.facing_angle - self.angle_spread;
            let mut arc_start_vector = PathPoint{x: arc_start_angle.cos() , y: arc_start_angle.sin()};
            arc_start_vector *= self.radius;
            let arc_start_point = self.center + arc_start_vector;
            (arc_start_point - point).len()
        } else if angle_diff > self.angle_spread {
            let arc_end_angle = self.facing_angle + self.angle_spread;
            let mut arc_end_vector = PathPoint{x: arc_end_angle.cos() , y: arc_end_angle.sin()};
            arc_end_vector *= self.radius;
            let arc_end_point = self.center + arc_end_vector;
            (arc_end_point - point).len()
        } else {
            (relative_point.len() - self.radius).abs()
        }
    }
}

pub struct LineSegment {
    start_point: PathPoint,
    end_point: PathPoint,
}

impl USDF for LineSegment {
    fn distance(&self, point: PathPoint) -> f64 {
        let pa = point - self.start_point;
        let ba = self.end_point - self.start_point;
        let h = ((pa * ba) / (ba * ba)).clamp(0.0, 1.0);
        (pa - ba*h).len()
    }
}
