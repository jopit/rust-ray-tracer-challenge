use crate::geometry::{point, Matrix, Point, Vector};
use crate::raytracer::{Intersection, Intersections, Material, Ray};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    material: Material,

    transform: Matrix,
    inverse_transform: Matrix,
    transposed_inverse_transform: Matrix,
}

impl<'a> Sphere {
    pub fn new() -> Sphere {
        Sphere {
            material: Material::new(),
            transform: Matrix::new(),
            inverse_transform: Matrix::new(),
            transposed_inverse_transform: Matrix::new(),
        }
    }

    pub fn intersect(&'a self, ray: Ray) -> Intersections<'a> {
        let t_ray = ray.transform(&self.inverse_transform);

        // Vector from the sphere's center, to the ray's origin
        let to_ray = t_ray.origin() - Point::new(0, 0, 0);

        let a = t_ray.direction().dot(t_ray.direction());
        let b = 2.0 * (t_ray.direction().dot(to_ray));
        let c = (to_ray.dot(to_ray)) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            let i1 = Intersection::new(t1, self);
            let i2 = Intersection::new(t2, self);

            Intersections::from(vec![i1, i2])
        } else {
            Intersections::new()
        }
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        let object_point = self.inverse_transform * world_point;
        let object_normal = object_point - point::ORIGIN;
        let world_normal = self.transposed_inverse_transform * object_normal;
        world_normal.norm()
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn with_transform(mut self, transform: Matrix) -> Self {
        self.transform = transform;
        self.inverse_transform = transform.inverse();
        self.transposed_inverse_transform = self.inverse_transform.transpose();
        self
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use super::*;
    use crate::{
        geometry::{Matrix, Vector},
        raytracer::Material,
    };

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[1].t(), 6.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0, 2, -5), Vector::new(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), -1.0);
        assert_eq!(xs[1].t(), 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Point::new(0, 0, 5), Vector::new(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), -6.0);
        assert_eq!(xs[1].t(), -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(std::ptr::eq(xs[0].object(), &s));
        assert!(std::ptr::eq(xs[1].object(), &s));
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::new();

        assert_eq!(*s.transform(), Matrix::new());
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let t = Matrix::new().translate(2, 3, 4);
        let s = Sphere::new().with_transform(t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Sphere::new().with_transform(Matrix::new().scale(2, 2, 2));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 3.0);
        assert_eq!(xs[1].t(), 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Sphere::new().with_transform(Matrix::new().translate(5, 0, 0));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Point::new(1, 0, 0));

        assert_eq!(n, Vector::new(1, 0, 0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Point::new(0, 1, 0));

        assert_eq!(n, Vector::new(0, 1, 0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Point::new(0, 0, 1));

        assert_eq!(n, Vector::new(0, 0, 1));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let sqrt_3: f64 = 3.0_f64.sqrt();

        let s = Sphere::new();

        let n = s.normal_at(Point::new(sqrt_3 / 3.0, sqrt_3 / 3.0, sqrt_3 / 3.0));

        assert_eq!(n, Vector::new(sqrt_3 / 3.0, sqrt_3 / 3.0, sqrt_3 / 3.0));
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let sqrt_3: f64 = 3.0_f64.sqrt();

        let s = Sphere::new();

        let n = s.normal_at(Point::new(sqrt_3 / 3.0, sqrt_3 / 3.0, sqrt_3 / 3.0));

        assert_eq!(n, n.norm());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Sphere::new().with_transform(Matrix::new().translate(0, 1, 0));

        let n = s.normal_at(Point::new(0, 1.70711, -0.70711));

        assert_eq!(n, Vector::new(0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let m = Matrix::new().scale(1, 0.5, 1) * Matrix::new().rotate_z(PI / 5.0);
        let s = Sphere::new().with_transform(m);

        let n = s.normal_at(Point::new(0, SQRT_2 / 2.0, -SQRT_2 / 2.0));

        assert_eq!(n, Vector::new(0, 0.97014, -0.24254));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::new();

        let m = s.material();

        assert_eq!(m, Material::default());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let m = Material::new().with_ambient(1.0);
        let s = Sphere::new().with_material(m);

        assert_eq!(s.material(), m);
    }
}
