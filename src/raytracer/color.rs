extern crate image;

use crate::feq;
use image::Rgb;

pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

pub const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new<R: Into<f64>, G: Into<f64>, B: Into<f64>>(red: R, green: G, blue: B) -> Color {
        Color {
            red: red.into(),
            green: green.into(),
            blue: blue.into(),
        }
    }
}

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        Rgb([
            (value.red.clamp(0.0, 1.0) * 255.0) as u8,
            (value.green.clamp(0.0, 1.0) * 255.0) as u8,
            (value.blue.clamp(0.0, 1.0) * 255.0) as u8,
        ])
    }
}

impl Default for Color {
    fn default() -> Self {
        BLACK
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        feq(self.red, other.red) && feq(self.green, other.green) && feq(self.blue, other.blue)
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl<T> std::ops::Mul<T> for Color
where
    T: Into<f64>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        let val = scalar.into();
        Color {
            red: self.red * val,
            green: self.green * val,
            blue: self.blue * val,
        }
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn substracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
        assert_eq!(c1.red, 1.0);
    }
}
