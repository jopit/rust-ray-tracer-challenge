mod canvas;
pub use crate::canvas::{canvas, Canvas};

mod color;
pub use crate::color::{color, Color};

mod point;
pub use crate::point::{point, Point};

mod vector;
pub use crate::vector::{vector, Vector};

const EPSILON: f64 = 0.00001;

pub fn feq(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPSILON
}
