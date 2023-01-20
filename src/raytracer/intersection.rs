use crate::{
    geometry::{Point, Vector},
    raytracer::{Color, PointLight, Ray},
    shape::Shape,
};
use std::ops::Index;

#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Shape,
}

impl<'a> Intersection<'a> {
    pub fn new<T: Into<f64>>(t: T, object: &'a Shape) -> Intersection<'a> {
        Intersection {
            t: t.into(),
            object,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &'a Shape {
        self.object
    }

    pub fn compute_state(&self, ray: Ray) -> IntersectionState {
        let point = ray.position(self.t);
        let eye_v = -ray.direction();
        let normal_v = self.object.normal_at(point);
        let inside = normal_v.dot(eye_v) < 0.0;

        IntersectionState {
            t: self.t,
            object: self.object,
            point,
            eye_v,
            normal_v: if inside { -normal_v } else { normal_v },
            inside,
        }
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && std::ptr::eq(self.object, other.object)
    }
}

// -----------------------------------------------------------------------------

pub struct Intersections<'a> {
    xs: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new() -> Self {
        Intersections { xs: vec![] }
    }

    pub fn from(xs: Vec<Intersection<'a>>) -> Self {
        Intersections { xs }
    }

    pub fn len(&self) -> usize {
        self.xs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.xs.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<Intersection<'a>> {
        self.xs.iter()
    }

    pub fn sort(mut self) -> Self {
        self.xs
            .sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        self
    }

    pub fn hit(&self) -> Option<&Intersection<'a>> {
        self.xs.iter().find(|it| it.t >= 0.0)
    }
}

impl<'a> Default for Intersections<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.xs[index]
    }
}

impl<'a> IntoIterator for Intersections<'a> {
    type Item = Intersection<'a>;
    type IntoIter = std::vec::IntoIter<Intersection<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.xs.into_iter()
    }
}

impl<'a> FromIterator<Intersection<'a>> for Intersections<'a> {
    fn from_iter<T: IntoIterator<Item = Intersection<'a>>>(iter: T) -> Self {
        Intersections {
            xs: iter.into_iter().collect(),
        }
    }
}

// -----------------------------------------------------------------------------

pub struct IntersectionState<'a> {
    t: f64,
    object: &'a Shape,
    point: Point,
    eye_v: Vector,
    normal_v: Vector,
    inside: bool,
}
impl<'a> IntersectionState<'a> {
    pub fn lighting(&'a self, light: PointLight) -> Color {
        self.object
            .material()
            .lighting(light, self.point, self.eye_v, self.normal_v)
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Point, Vector},
        raytracer::Ray,
        shape::Sphere,
    };

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert!(std::ptr::eq(i.object, &s));
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1, &s);
        let i2 = Intersection::new(2, &s);

        let xs = Intersections::from(vec![i1, i2]);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 1.0);
        assert_eq!(xs[1].t(), 2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1, &s);
        let i2 = Intersection::new(2, &s);
        let xs = Intersections::from(vec![i1, i2]);

        let i = xs.hit().unwrap();

        assert_eq!(*i, i1);
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1, &s);
        let i2 = Intersection::new(1, &s);
        let xs = Intersections::from(vec![i2, i1]);

        let i = xs.hit().unwrap();

        assert_eq!(*i, i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2, &s);
        let i2 = Intersection::new(-1, &s);
        let xs = Intersections::from(vec![i2, i1]);

        let i = xs.hit();

        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5, &s);
        let i2 = Intersection::new(7, &s);
        let i3 = Intersection::new(-3, &s);
        let i4 = Intersection::new(2, &s);
        let mut xs = Intersections::from(vec![i1, i2, i3, i4]);

        // in this implementation, it's the responsibility of the caller
        // of hit() to ensure the intersections are sorted. Here we test
        // that the intersections can be sorted.
        xs = xs.sort();
        let i = xs.hit().unwrap();

        assert_eq!(*i, i4);
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersection::new(4, &shape);

        let comps = i.compute_state(r);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0, 0, -1));
        assert_eq!(comps.eye_v, Vector::new(0, 0, -1));
        assert_eq!(comps.normal_v, Vector::new(0, 0, -1));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersection::new(4, &shape);

        let comps = i.compute_state(r);

        assert!(comps.inside == false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersection::new(1, &shape);

        let comps = i.compute_state(r);

        assert_eq!(comps.point, Point::new(0, 0, 1));
        assert_eq!(comps.eye_v, Vector::new(0, 0, -1));
        assert_eq!(comps.inside, true);
        // normal would have been (0, 0, 1), but is inverted!
        assert_eq!(comps.normal_v, Vector::new(0, 0, -1));
    }
}
