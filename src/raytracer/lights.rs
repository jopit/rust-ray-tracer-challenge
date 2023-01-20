use crate::geometry::Point;
use crate::raytracer::color::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointLight {
    position: Point,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::new(0, 0, 0);

        let light = PointLight::new(position, intensity);

        assert_eq!(light.position(), position);
        assert_eq!(light.intensity(), intensity);
    }
}
