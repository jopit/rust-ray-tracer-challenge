pub mod geometry {
    pub mod matrix;
    pub use matrix::Matrix;

    pub mod point;
    pub use point::Point;

    mod tuple;
    pub use tuple::Tuple;

    mod transformation;
    pub use transformation::view_transform;

    mod vector;
    pub use vector::Vector;
}

pub mod raytracer {
    mod camera;
    pub use camera::Camera;

    mod canvas;
    pub use canvas::Canvas;

    pub mod color;
    pub use color::Color;

    mod intersection;
    pub use intersection::{Intersection, IntersectionState, Intersections};

    mod lights;
    pub use lights::PointLight;

    mod material;
    pub use material::Material;

    mod ray;
    pub use ray::Ray;

    mod world;
    pub use world::World;
}

pub mod shape {
    mod base;
    pub use base::Shape;

    mod sphere;
    pub use sphere::Sphere;
}

const EPSILON: f64 = 0.00001;

pub fn feq(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPSILON
}
