use crate::{feq, Point, Tuple};

#[derive(Debug, Default, Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Vector {
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

impl Vector {
    pub fn new<T1: Into<f64>, T2: Into<f64>, T3: Into<f64>>(x: T1, y: T2, z: T3) -> Vector {
        Vector {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
    
    pub fn mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn norm(&self) -> Self {
        let mag = self.mag();
        Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        feq(self.x, other.x) && feq(self.y, other.y) && feq(self.z, other.z)
    }
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, point: Point) -> Self::Output {
        Point::new(self.x + point.x(), self.y + point.y(), self.z + point.z())
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> std::ops::Mul<T> for Vector
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

impl<T> std::ops::Div<T> for Vector
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
