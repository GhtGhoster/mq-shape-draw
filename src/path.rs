
use std::collections::HashSet;

use crate::point::PathPoint;

pub struct DrawPath {
    pub points: HashSet<PathPoint>,
    pub min: Option<PathPoint>,
    pub max: Option<PathPoint>,
}

impl DrawPath {
    pub fn new() -> Self {
        Self {
            points: HashSet::new(),
            min: None,
            max: None,
        }
    }

    pub fn push(&mut self, point: PathPoint) {
        self.points.insert(point);

        let PathPoint{x, y} = point;

        // track lowest x and y values
        match self.min {
            Some(mut min) => {
                if x < min.x {min.x = x}
                if y < min.y {min.y = y}
            }
            None => {
                self.min = Some(point);
            }
        }

        // track highest x and y values
        match self.max {
            Some(mut max) => {
                if x > max.x {max.x = x}
                if y > max.y {max.y = y}
            }
            None => {
                self.max = Some(point);
            }
        }
    }

    // scale down to 0..=1 f64 range for use in USDFs
    pub fn scaled(&mut self) -> Option<HashSet<PathPoint>> {
        if let Some(min) = self.min {
            if let Some(max) = self.max {
                let factor: PathPoint = max - min;
                let mut scaled: HashSet<PathPoint> = HashSet::new();
                for mut point in self.points.clone() {
                    point -= min;
                    point /= factor;
                    scaled.insert(point);
                }
                Some(scaled)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn score(&self) -> f64 {
        todo!()
    }
}
