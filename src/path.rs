
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

    pub fn reverse_score(&self, shape: &Shape) -> f64 {
        let mut count = 0.0;
        let mut sum = 0.0;
        for shape_point in shape.step_through(10) {
            let mut min = f64::MAX;
            for point in &self.points {
                min = min.min((shape_point - *point).len().sqrt());
            }
            count += 1.0;
            sum += min;
        }
        sum / count
    }

    pub fn full_score(&self, shape: &Shape) -> f64 {
        self.score(shape) + self.reverse_score(shape)
    }
}
