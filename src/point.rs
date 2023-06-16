
use std::{ops::{Add, Sub, SubAssign, Mul, MulAssign, DivAssign}, hash::Hash, f64::consts::TAU};

use macroquad::prelude::*;

// very specific implementation of point ignoring NaN dangers and unorthodox Div impl
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PathPoint {
    pub x: f64,
    pub y: f64,
}

impl PathPoint {
    pub fn from_mouse_pos() -> Self {
        let (mut x, mut y) = mouse_position();
        x /= screen_width();
        y /= screen_height();
        Self {
            x: x as f64,
            y: y as f64,
        }
    }

    pub fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn angle(&self) -> f64 {
        f64::atan2(self.y, self.x).rem_euclid(TAU)
    }
}

impl Add for PathPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for PathPoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for PathPoint {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<f64> for PathPoint {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

// dot product
impl Mul for PathPoint {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        (self.x * other.x) + (self.y + other.y)
    }
}

impl MulAssign<f64> for PathPoint {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
    }
}

impl DivAssign for PathPoint {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

// this is dangerous because NaN but that's not gonna happen :clueless:
impl Hash for PathPoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

// this is dangerous because NaN but that's not gonna happen :clueless:
impl Eq for PathPoint {}
