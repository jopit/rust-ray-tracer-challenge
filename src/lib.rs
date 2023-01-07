mod canvas;
pub use crate::canvas::{canvas, Canvas};

mod color;
pub use crate::color::{color, Color};

mod matrix;
pub use crate::matrix::{
    matrix, matrix_sized, rotate_x, rotate_y, rotate_z, scale, scale_u, shear, translate, Matrix,
};

mod point;
pub use crate::point::{point, Point};

mod vector;
pub use crate::vector::{vector, Vector};

const EPSILON: f64 = 0.00001;

pub fn feq(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPSILON
}
