pub mod geometry {
    mod matrix;
    pub use matrix::Matrix;

    pub mod point;
    pub use point::Point;

    mod tuple;
    pub use tuple::Tuple;

    mod vector;
    pub use vector::Vector;
}

pub mod raytracer {
    mod canvas;
    pub use canvas::Canvas;

    pub mod color;
    pub use color::Color;

    mod intersection;
    pub use intersection::{Intersection, Intersections};

    mod lights;
    pub use lights::PointLight;

    mod material;
    pub use material::Material;

    mod ray;
    pub use ray::Ray;
}

pub mod shape {
    mod sphere;
    pub use sphere::Sphere;
}

const EPSILON: f64 = 0.00001;

pub fn feq(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPSILON
}
