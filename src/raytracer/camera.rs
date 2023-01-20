use crate::geometry::{view_transform, Matrix, Point, Vector};

use super::{Canvas, Ray, World};

pub struct Camera {
    hsize: usize,
    vsize: usize,
    transform: Matrix,
    inverse_transform: Matrix,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = (hsize as f64) / (vsize as f64);
        let half_width = if aspect >= 1.0 {
            half_view
        } else {
            half_view * aspect
        };
        let half_height = if aspect >= 1.0 {
            half_view / aspect
        } else {
            half_view
        };
        let pixel_size = (half_width * 2.0) / (hsize as f64);

        Camera {
            hsize,
            vsize,
            transform: Matrix::new(),
            inverse_transform: Matrix::new(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn with_transform(mut self, transform: Matrix) -> Self {
        self.transform = transform;
        self.inverse_transform = transform.inverse();
        self
    }

    pub fn with_view_transform(self, from: Point, to: Point, up: Vector) -> Self {
        self.with_transform(view_transform(from, to, up))
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let xoffset = ((px as f64) + 0.5) * self.pixel_size;
        let yoffset = ((py as f64) + 0.5) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space
        // (note that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (note that the canvas is at z=-1)
        let pixel = self.inverse_transform * Point::new(world_x, world_y, -1);
        let origin = self.inverse_transform * Point::new(0, 0, 0);
        let direction = (pixel - origin).norm();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                image.set(x, y, color);
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        feq,
        geometry::{view_transform, Point, Vector},
        raytracer::{world::test_utils::default_world, Color},
    };
    use std::{f32::consts::SQRT_2, f64::consts::PI};

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        // assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, Matrix::new());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert!(feq(c.pixel_size, 0.01));
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert!(feq(c.pixel_size, 0.01));
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin(), Point::new(0, 0, 0));
        assert_eq!(r.direction(), Vector::new(0, 0, -1));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);

        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin(), Point::new(0, 0, 0));
        assert_eq!(r.direction(), Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let transform = Matrix::new().translate(0, -2, 5).rotate_y(PI / 4.0);
        let c = Camera::new(201, 101, PI / 2.0).with_transform(transform);

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin(), Point::new(0, 2, -5));
        assert_eq!(r.direction(), Vector::new(SQRT_2 / 2.0, 0, -SQRT_2 / 2.0));
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = default_world();
        let from = Point::new(0, 0, -5);
        let to = Point::new(0, 0, 0);
        let up = Vector::new(0, 1, 0);
        let transform = view_transform(from, to, up);
        let c = Camera::new(11, 11, PI / 2.0).with_transform(transform);

        let image = c.render(&w);

        assert_eq!(image.get(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
