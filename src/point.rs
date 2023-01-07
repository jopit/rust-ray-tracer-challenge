use crate::{feq, vector, Tuple, Vector};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        feq(self.x, other.x) && feq(self.y, other.y) && feq(self.z, other.z)
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Self;

    fn add(self, vector: Vector) -> Self::Output {
        Point {
            x: self.x + vector.x(),
            y: self.y + vector.y(),
            z: self.z + vector.z(),
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, other: Vector) -> Self::Output {
        Point {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

pub fn new<T1: Into<f64>, T2: Into<f64>, T3: Into<f64>>(x: T1, y: T2, z: T3) -> Point {
    Point {
        x: x.into(),
        y: y.into(),
        z: z.into(),
    }
}
