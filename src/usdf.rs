
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
    fn step_through(&self, steps: usize) -> Vec<PathPoint>;
    fn get_corners(&self) -> [PathPoint; 2];
}

// a.k.a Circular arc
pub struct CircleSegment {
    pub center: PathPoint,
    pub radius: f64,
    pub facing_angle: f64,
    pub angle_spread: f64,
    pub start_point: PathPoint,
    pub end_point: PathPoint,
}

impl CircleSegment {
    pub fn new(center: PathPoint, radius: f64, facing_angle: f64, angle_spread: f64) -> Self {
        // start point
        let arc_start_angle = facing_angle - angle_spread;
        let arc_start_vector = PathPoint::new(arc_start_angle.cos(), arc_start_angle.sin()) * radius;
        let arc_start_point = center + arc_start_vector;

        // end point
        let arc_end_angle = facing_angle + angle_spread;
        let arc_end_vector = PathPoint::new(arc_end_angle.cos(), arc_end_angle.sin()) * radius;
        let arc_end_point = center + arc_end_vector;

        Self {
            center,
            radius,
            facing_angle,
            angle_spread,
            start_point: arc_start_point,
            end_point: arc_end_point,
        }
    }
}

impl USDF for CircleSegment {
    fn distance(&self, point: PathPoint) -> f64 {
        let relative_point = point-self.center;
        let point_angle = relative_point.angle();
        let angle_diff = (self.facing_angle - point_angle + PI + TAU) % TAU - PI;
        if angle_diff > self.angle_spread {
            (self.start_point - point).len()
        } else if angle_diff < -self.angle_spread {
            (self.end_point - point).len()
        } else {
            (relative_point.len() - self.radius).abs()
        }
    }

    fn step_through(&self, steps: usize) -> Vec<PathPoint> {
        let mut ret = vec![];
        let actual_steps = (steps as f64 * self.angle_spread) as usize;
        for step in 1..actual_steps {
            let t = step as f64 / actual_steps as f64;
            let angle = self.facing_angle - self.angle_spread + (self.angle_spread*2.0*t);
            ret.push((PathPoint::from_angle(angle) * self.radius) + self.center);
        }
        ret
    }
    
    fn get_corners(&self) -> [PathPoint; 2] {
        [self.start_point, self.end_point]
    }
}

pub struct LineSegment {
    pub start_point: PathPoint,
    pub end_point: PathPoint,
}

impl LineSegment {
    pub fn new(start_point: PathPoint, end_point: PathPoint) -> Self {
        Self {
            start_point,
            end_point,
        }
    }
}

impl USDF for LineSegment {
    fn distance(&self, point: PathPoint) -> f64 {
        let ba = self.end_point - self.start_point;
        let pa = point - self.start_point;
        let h = (pa.dot(&ba) / ba.dot(&ba)).clamp(0.0, 1.0);
        (pa - ba*h).len()
    }

    fn step_through(&self, steps: usize) -> Vec<PathPoint> {
        let mut ret = vec![];
        for step in 1..steps as usize {
            let t = step as f64 / steps as f64;
            let vector = self.end_point - self.start_point;
            ret.push(vector * t + self.start_point);
        }
        ret
    }
    
    fn get_corners(&self) -> [PathPoint; 2] {
        [self.start_point, self.end_point]
    }
}
