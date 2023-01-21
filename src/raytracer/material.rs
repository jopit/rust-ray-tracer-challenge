use crate::geometry::{Point, Vector};

use super::{color, Color, PointLight};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: color::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn ambient(&self) -> f64 {
        self.ambient
    }

    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn specular(&self) -> f64 {
        self.specular
    }

    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    pub fn lighting(
        &self,
        light: PointLight,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
    ) -> Color {
        // combine the surface color with the light's color/intensity
        let effective_color = self.color * light.intensity();

        // find the direction to the light source
        let lightv = (light.position() - point).norm();

        // compute the ambient contribution
        let ambient = effective_color * self.ambient;

        // ignore diffuse and specular components if in shadow
        if in_shadow {
            return ambient;
        }

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal < 0.0 {
            return ambient;
        }

        // compute the diffuse contribution
        let diffuse = effective_color * self.diffuse * light_dot_normal;

        // reflect_dot_eye represents the cosine of the angle between the
        // reflection vector and the eye vector. A negative number means the
        // light reflects away from the eye.
        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);

        let specular = if reflect_dot_eye > 0.0 {
            // compute the specular contribution
            let factor = reflect_dot_eye.powf(self.shininess);
            light.intensity() * (self.specular * factor)
        } else {
            color::BLACK
        };

        // Add the three contributions together to get the final shading
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::f32::consts::SQRT_2;

    use super::*;
    use crate::{
        geometry::{Point, Vector},
        raytracer::PointLight,
    };

    fn background() -> (Material, Point) {
        (Material::new(), Point::new(0, 0, 0))
    }

    #[test]
    fn the_default_material() {
        let m = Material::new();

        assert_eq!(m.color, color::WHITE);
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let (m, position) = background();
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees() {
        let (m, position) = background();
        let eyev = Vector::new(0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let (m, position) = background();
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let (m, position) = background();
        let eyev = Vector::new(0, -SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let (m, position) = background();
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, 10), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let (m, position) = background();
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;

        let result = m.lighting(light, position, eyev, normalv, in_shadow);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
