
use std::{ops::{Add, Sub, SubAssign, Mul, MulAssign, DivAssign, Div}, hash::Hash, f64::consts::TAU, fmt::Display};

use macroquad::prelude::*;

// very specific implementation of point ignoring NaN dangers and unorthodox Div impl
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PathPoint {
    pub x: f64,
    pub y: f64,
}

impl PathPoint {
    // constructors
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }

    #[allow(dead_code)]
    pub fn from_angle(angle: f64) -> Self {
        Self {
            x: angle.cos(),
            y: angle.sin(),
        }.lerp_to_normalized_domain((Self::new(-1.0, -1.0), Self::new(1.0, 1.0)))
    }

    pub fn from_screenspace(x: f32, y: f32) -> Self {
        // let x = x / screen_width();
        // let y = y / screen_height();
        // Self {
        //     x: x as f64,
        //     y: y as f64,
        // }
        Self::new(x as f64, y as f64).lerp_to_normalized_domain(Self::screen_domain())
    }

    pub fn from_mouse_pos() -> Self {
        let (x, y) = mouse_position();
        Self::from_screenspace(x, y)
    }

    pub fn screen_domain() -> (Self, Self) {
        (
            Self::new(0.0, 0.0),
            Self::new(screen_width() as f64, screen_height() as f64),
        )
    }

    // domain conversion
    // converts from <min-max> domain to <0-1>
    pub fn lerp_to_normalized_domain(self, (min, max): (Self, Self)) -> Self {
        let old_domain = max - min;
        let point = self - min;
        point / old_domain
    }

    // converts from <0-1> domain to <min-max>
    pub fn lerp_from_normalized_domain(self, (min, max): (Self, Self)) -> Self {
        let new_domain = max - min;
        let point = self * new_domain;
        point + min
    }

    // utils
    pub fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn angle(&self) -> f64 {
        f64::atan2(self.y, self.x).rem_euclid(TAU)
    }

    #[allow(dead_code)]
    pub fn cross(&self, other: &Self) -> f64 {
        (self.x * other.y) - (self.y * other.x)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

impl Display for PathPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.2}, {:.2}]", self.x, self.y)
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

impl Mul for PathPoint {
    type Output = Self;
 
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl MulAssign<f64> for PathPoint {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
    }
}

impl Div for PathPoint {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
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
