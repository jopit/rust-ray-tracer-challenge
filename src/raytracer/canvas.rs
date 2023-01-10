extern crate image;

use crate::raytracer::{color, Color};
use image::{ImageBuffer, ImageResult, RgbImage};

pub struct Canvas {
    data: Vec<Color>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            data: vec![color::BLACK; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, col: usize, row: usize) -> Color {
        self.data[self.height * col + row]
    }

    pub fn set(&mut self, col: usize, row: usize, color: Color) {
        self.data[self.height * col + row] = color;
    }

    pub fn save(&self, path: &str) -> ImageResult<()> {
        let mut image: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);
        for (col, row, pixel) in image.enumerate_pixels_mut() {
            *pixel = self.get(col as usize, row as usize).into()
        }
        image.save(path)
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);

        for col in 0..c.width() {
            for row in 0..c.height() {
                assert_eq!(c.get(col, row), color::BLACK);
            }
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.set(2, 3, red);

        assert_eq!(c.get(2, 3), red);
    }
}
