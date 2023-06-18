
use crate::{point::PathPoint, shape::Shape};

pub struct DrawPath {
    pub points: Vec<PathPoint>,
    pub domain: Option<(PathPoint, PathPoint)>,
}

impl DrawPath {
    pub fn new() -> Self {
        Self {
            points: vec![],
            domain: None,
        }
    }

    pub fn push(&mut self, point: PathPoint) {
        if !self.points.contains(&point) {
            self.points.push(point);
        }

        // track lowest and highest x and y values
        self.domain = if let Some((min, max)) = self.domain {
            Some((point.min(&min), point.max(&max)))
        } else {
            Some((point, point))
        };
    }

    pub fn score(&self, shape: &Shape) -> f64 {
        let mut count = 0.0;
        let mut sum = 0.0;
        for point in &self.points {
            count += 1.0;
            sum += shape.score(*point);
        }
        sum / count
    }
}
