use crate::{feq, Point};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        vector(self.x / mag, self.y / mag, self.z / mag)
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        feq(self.x, other.x) && feq(self.y, other.y) && feq(self.z, other.z)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, point: Point) -> Self::Output {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
            z: self.z + point.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Mul<T> for Vector
where
    T: Into<f64>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        let val = scalar.into();
        Vector {
            x: self.x * val,
            y: self.y * val,
            z: self.z * val,
        }
    }
}

impl<T> Div<T> for Vector
where
    T: Into<f64>,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        let val = scalar.into();
        Vector {
            x: self.x / val,
            y: self.y / val,
            z: self.z / val,
        }
    }
}

pub fn vector<T1: Into<f64>, T2: Into<f64>, T3: Into<f64>>(x: T1, y: T2, z: T3) -> Vector {
    Vector {
        x: x.into(),
        y: y.into(),
        z: z.into(),
    }
}
