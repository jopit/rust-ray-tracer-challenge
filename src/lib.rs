pub mod canvas;
pub use crate::canvas::Canvas;

pub mod color;
pub use crate::color::Color;

pub mod matrix;
pub use crate::matrix::Matrix;

pub mod point;
pub use crate::point::Point;

pub mod tuple;
pub use crate::tuple::Tuple;

pub mod vector;
pub use crate::vector::Vector;

const EPSILON: f64 = 0.00001;

pub fn feq(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPSILON
}
