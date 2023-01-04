extern crate image;

use crate::color::{Color, BLACK};
use image::{ImageBuffer, ImageResult, RgbImage};

pub struct Canvas {
    data: Vec<Color>,
    width: usize,
    height: usize,
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas {
        data: vec![BLACK; width * height],
        width,
        height,
    }
}

impl Canvas {
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
            *pixel = self.get(col as usize, row as usize).to_rgb()
        }
        image.save(path)
    }
}
