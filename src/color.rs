extern crate image;

use crate::feq;
use image::Rgb;
use std::ops::{Add, Mul, Sub};

pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub fn color<R: Into<f64>, G: Into<f64>, B: Into<f64>>(red: R, green: G, blue: B) -> Color {
    Color {
        red: red.into(),
        green: green.into(),
        blue: blue.into(),
    }
}

impl Color {
    pub fn to_rgb(&self) -> Rgb<u8> {
        fn clamp(val: f64) -> f64 {
            if val < 0.0 {
                0.0
            } else if val > 1.0 {
                1.0
            } else {
                val
            }
        }

        Rgb([
            (clamp(self.red) * 255.0) as u8,
            (clamp(self.green) * 255.0) as u8,
            (clamp(self.blue) * 255.0) as u8,
        ])
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        feq(self.red, other.red) && feq(self.green, other.green) && feq(self.blue, other.blue)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        color(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        color(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        color(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl<T> Mul<T> for Color
where
    T: Into<f64>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        let val = scalar.into();
        color(self.red * val, self.green * val, self.blue * val)
    }
}
