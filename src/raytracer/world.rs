use std::marker::PhantomData;

use crate::raytracer::{color, Color, IntersectionState, Intersections, PointLight, Ray};
use crate::shape::Shape;

#[derive(Debug)]
pub struct World<'a> {
    objects: Vec<Shape>,
    lights: Vec<PointLight>,
    _marker: PhantomData<&'a Shape>,
}

impl<'a> World<'a> {
    pub fn new() -> World<'a> {
        World {
            objects: vec![],
            lights: vec![],
            _marker: PhantomData,
        }
    }

    pub fn with_lights(mut self, mut lights: Vec<PointLight>) -> Self {
        self.lights.append(&mut lights);
        self
    }

    pub fn with_objects(mut self, mut objects: Vec<Shape>) -> Self {
        self.objects.append(&mut objects);
        self
    }

    pub fn intersect_world(&'a self, r: Ray) -> Intersections<'a> {
        let xs: Intersections = self.objects.iter().flat_map(|o| o.intersect(r)).collect();
        xs.sort()
    }

    pub fn shade_hit(&self, state: IntersectionState) -> Color {
        self.lights
            .iter()
            .map(|light| state.lighting(*light))
            .fold(color::BLACK, |acc, c| acc + c)
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let xs = self.intersect_world(ray);
        if let Some(x) = xs.hit() {
            self.shade_hit(x.compute_state(ray))
        } else {
            color::BLACK
        }
    }
}

impl<'a> Default for World<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use crate::{
        geometry::{Matrix, Point},
        raytracer::{Color, Material},
        shape::Sphere,
    };

    pub fn default_world<'a>() -> World<'a> {
        let light = PointLight::new(Point::new(-10, 10, -10), Color::new(1, 1, 1));
        let material = Material::new()
            .with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);

        let s1 = Sphere::new().with_material(material);
        let s2 = Sphere::new().with_transform(Matrix::new().scale_u(0.5));

        let mut w = World::new();
        w.objects = vec![s1, s2];
        w.lights = vec![light];

        w
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raytracer::world::test_utils::*;
    use crate::{
        geometry::{Matrix, Point, Vector},
        raytracer::{Color, Intersection, Material, Ray},
        shape::Sphere,
    };

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn the_default_world() {
        let light = PointLight::new(Point::new(-10, 10, -10), Color::new(1, 1, 1));
        let material = Material::new()
            .with_color(Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = Sphere::new().with_material(material);
        let s2 = Sphere::new().with_transform(Matrix::new().scale(0.5, 0.5, 0.5));

        let w = default_world();

        assert_eq!(w.lights.len(), 1);
        assert!(w.lights.contains(&light));
        assert_eq!(w.objects.len(), 2);
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));

        let xs = w.intersect_world(r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[1].t(), 4.5);
        assert_eq!(xs[2].t(), 5.5);
        assert_eq!(xs[3].t(), 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = &w.objects[0];
        let i = Intersection::new(4, shape);

        let comps = i.compute_state(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.lights[0] = PointLight::new(Point::new(0, 0.25, 0), Color::new(1, 1, 1));
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);

        let comps = i.compute_state(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 1, 0));

        let c = w.color_at(r);

        assert_eq!(c, color::BLACK);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));

        let c = w.color_at(r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = default_world();

        w.objects[0] = w.objects[0]
            .clone()
            .with_material(w.objects[0].material().with_ambient(1.0));
        w.objects[1] = w.objects[1]
            .clone()
            .with_material(w.objects[1].material().with_ambient(1.0));
        let r = Ray::new(Point::new(0, 0, 0.75), Vector::new(0, 0, -1));

        let c = w.color_at(r);

        assert_eq!(c, w.objects[1].material().color());
    }
}
