
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
        let mut min = f64::MAX;
        for usdf in &self.usdfs {
            min = usdf.distance(point).sqrt().min(min);
        }
        min
    }
}
