use crate::{feq, Vector};
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        feq(self.x, other.x) && feq(self.y, other.y) && feq(self.z, other.z)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, vector: Vector) -> Self::Output {
        Point {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, other: Vector) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

pub fn point<T1: Into<f64>, T2: Into<f64>, T3: Into<f64>>(x: T1, y: T2, z: T3) -> Point {
    Point {
        x: x.into(),
        y: y.into(),
        z: z.into(),
    }
}
