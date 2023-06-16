
use crate::{usdf::USDF, point::PathPoint};

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
        let mut sum: f64 = 0.0;
        let mut count: f64 = 0.0;
        for usdf in &self.usdfs {
            sum += usdf.distance(point);
            count += 1.0;
        }
        sum / count
    }
}
