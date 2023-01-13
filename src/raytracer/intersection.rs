use crate::shape::Sphere;

#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new<T: Into<f64>>(t: T, object: &'a Sphere) -> Intersection<'a> {
        Intersection {
            t: t.into(),
            object,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &'a Sphere {
        self.object
    }
}

impl<'a> std::cmp::PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && std::ptr::eq(self.object, other.object)
    }
}

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

    pub fn sort(mut self) -> Self {
        self.xs
            .sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        self
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.xs.iter().find(|it| it.t >= 0.0)
    }
}

impl<'a> Default for Intersections<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> std::ops::Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.xs[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
