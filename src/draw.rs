use image::{DynamicImage, GenericImageView, GrayImage};
use softbuffer::Surface;
use std::num::NonZeroU32;

trait Draw {
    fn draw(&self, surface: &mut Surface);
}

pub struct ImagePainter {
    width: u32,
    height: u32,
    draw: Box<dyn Draw>,
}

impl ImagePainter {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn draw(&self, surface: &mut Surface) {
        self.draw.draw(surface);
    }
}

impl From<DynamicImage> for ImagePainter {
    fn from(image: DynamicImage) -> Self {
        ImagePainter {
            width: image.width(),
            height: image.height(),
            draw: Box::new(image),
        }
    }
}

impl From<GrayImage> for ImagePainter {
    fn from(image: GrayImage) -> Self {
        ImagePainter {
            width: image.width(),
            height: image.height(),
            draw: Box::new(image),
        }
    }
}

impl Draw for DynamicImage {
    fn draw(&self, surface: &mut Surface) {
        let width = self.width();
        let height = self.height();
        surface
            .resize(
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            )
            .unwrap();

        let mut buffer = surface.buffer_mut().unwrap();
        for (x, y, pixel) in self.pixels() {
            let red = pixel.0[0] as u32;
            let green = pixel.0[1] as u32;
            let blue = pixel.0[2] as u32;
            let color = blue | (green << 8) | (red << 16);
            let index = y as usize * width as usize + x as usize;
            buffer[index] = color;
        }

        buffer.present().unwrap();
    }
}

impl Draw for GrayImage {
    fn draw(&self, surface: &mut Surface) {
        let width = self.width();
        let height = self.height();
        surface
            .resize(
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            )
            .unwrap();

        let mut buffer = surface.buffer_mut().unwrap();
        for y in 0..height {
            for x in 0..width {
                let luma = self.get_pixel(x, y).0[0] as u32;
                let color = luma | (luma << 8) | (luma << 16);
                let index = y as usize * width as usize + x as usize;
                buffer[index] = color;
            }
        }

        buffer.present().unwrap();
    }
}
