use image::{DynamicImage, GenericImageView, Rgb, RgbImage, Rgba};

pub struct Hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl Hsv {
    pub fn as_rgb8(&self) -> Rgb<u8> {
        Rgb([
            (self.h * 255.0).round() as u8,
            (self.s * 255.0).round() as u8,
            (self.v * 255.0).round() as u8,
        ])
    }
}

impl From<Rgba<u8>> for Hsv {
    fn from(pixel: Rgba<u8>) -> Self {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let mut h = 0.0;
        let mut s = 0.0;
        let v = max;
        let d = max - min;
        if max != 0.0 {
            s = d / max;
        }
        if max != min {
            h = match max {
                x if x == r => (g - b) / d + (if g < b { 6.0 } else { 0.0 }),
                x if x == g => (b - r) / d + 2.0,
                x if x == b => (r - g) / d + 4.0,
                _ => 0.0,
            };
            h /= 6.0;
        }
        Hsv { h, s, v }
    }
}

struct Hsl {
    h: f32,
    s: f32,
    l: f32,
}

impl Hsl {
    fn as_rgb8(&self) -> Rgb<u8> {
        Rgb([
            (self.h * 255.0).round() as u8,
            (self.s * 255.0).round() as u8,
            (self.l * 255.0).round() as u8,
        ])
    }
}

impl From<Rgba<u8>> for Hsl {
    fn from(pixel: Rgba<u8>) -> Self {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let mut h = 0.0;
        let mut s = 0.0;
        let l = (max + min) / 2.0;
        let d = max - min;
        if max != 0.0 {
            s = d / (1.0 - (2.0 * l - 1.0).abs());
        }
        if max != min {
            h = match max {
                x if x == r => (g - b) / d + (if g < b { 6.0 } else { 0.0 }),
                x if x == g => (b - r) / d + 2.0,
                x if x == b => (r - g) / d + 4.0,
                _ => 0.0,
            };
            h /= 6.0;
        }
        Hsl { h, s, l }
    }
}

pub struct Hsi {
    h: f32,
    s: f32,
    i: f32,
}

impl Hsi {
    fn as_rgb8(&self) -> Rgb<u8> {
        Rgb([
            (self.h * 255.0).round() as u8,
            (self.s * 255.0).round() as u8,
            (self.i * 255.0).round() as u8,
        ])
    }
}

impl From<Rgba<u8>> for Hsi {
    fn from(pixel: Rgba<u8>) -> Self {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;
        let mut h = 0.0;
        let mut s = 0.0;
        let i = (r + g + b) / 3.0;
        let min = r.min(g).min(b);
        if i != 0.0 {
            s = 1.0 - min / i;
        }
        if i == 0.0 {
            s = 0.0;
        }
        if s != 0.0 {
            h = match i {
                x if x == r => (g - b) / (3.0 * s),
                x if x == g => (b - r) / (3.0 * s) + 2.0,
                x if x == b => (r - g) / (3.0 * s) + 4.0,
                _ => 0.0,
            };
            h /= 6.0;
            if h < 0.0 {
                h += 1.0;
            }
        }
        Hsi { h, s, i }
    }
}

pub struct Yuv {
    y: f32,
    u: f32,
    v: f32,
}

impl Yuv {
    fn as_rgb8(&self) -> Rgb<u8> {
        Rgb([
            (self.y * 255.0).round() as u8,
            (self.u * 255.0 + 128.0).round() as u8,
            (self.v * 255.0 + 128.0).round() as u8,
        ])
    }
}

impl From<Rgba<u8>> for Yuv {
    fn from(pixel: Rgba<u8>) -> Self {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;
        let y = 0.299 * r + 0.587 * g + 0.114 * b;
        let u = -0.169 * r - 0.331 * g + 0.5 * b;
        let v = 0.5 * r - 0.419 * g - 0.081 * b;
        Yuv { y, u, v }
    }
}

pub fn rgb_to_hsv(image: &DynamicImage) -> DynamicImage {
    let mut out = RgbImage::new(image.width(), image.height());
    for (x, y, pixel) in image.pixels() {
        let hsv = Hsv::from(pixel).as_rgb8();
        out.put_pixel(x, y, hsv);
    }
    DynamicImage::from(out)
}

pub fn rgb_to_hsl(image: &DynamicImage) -> DynamicImage {
    let mut out = RgbImage::new(image.width(), image.height());
    for (x, y, pixel) in image.pixels() {
        let hsl = Hsl::from(pixel).as_rgb8();
        out.put_pixel(x, y, hsl);
    }
    DynamicImage::from(out)
}

pub fn rgb_to_hsi(image: &DynamicImage) -> DynamicImage {
    let mut out = RgbImage::new(image.width(), image.height());
    for (x, y, pixel) in image.pixels() {
        let hsi = Hsi::from(pixel).as_rgb8();
        out.put_pixel(x, y, hsi);
    }
    DynamicImage::from(out)
}

pub fn rgb_to_yuv(image: &DynamicImage) -> DynamicImage {
    let mut out = RgbImage::new(image.width(), image.height());
    for (x, y, pixel) in image.pixels() {
        let yuv = Yuv::from(pixel).as_rgb8();
        out.put_pixel(x, y, yuv);
    }
    DynamicImage::from(out)
}
