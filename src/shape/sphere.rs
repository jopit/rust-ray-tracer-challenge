use crate::geometry::{Matrix, Point};
use crate::raytracer::{Intersection, Intersections, Ray};

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    transform: Matrix,
    inverse_transform: Matrix,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix::new(),
            inverse_transform: Matrix::new(),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
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

            Intersections::with(vec![i1, i2])
        } else {
            Intersections::new()
        }
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
        self.inverse_transform = transform.inverse();
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Matrix, Vector};

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
        let mut s = Sphere::new();
        let t = Matrix::new().translate(2, 3, 4);

        s.set_transform(t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut s = Sphere::new();

        s.set_transform(Matrix::new().scale(2, 2, 2));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 3.0);
        assert_eq!(xs[1].t(), 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut s = Sphere::new();

        s.set_transform(Matrix::new().translate(5, 0, 0));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }
}
