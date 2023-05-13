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

    pub fn to_rgb8(&self) -> Rgb<u8> {
        let (r, g, b) = if self.s == 0.0 {
            (self.v, self.v, self.v)
        } else {
            let h = self.h * 6.0;
            let s = self.s;
            let v = self.v;
            let c = v * s;
            let x = c * (1.0 - (h % 2.0 - 1.0).abs());
            let (r, g, b) = match h.floor() as i32 % 6 {
                0 => (c, x, 0.0),
                1 => (x, c, 0.0),
                2 => (0.0, c, x),
                3 => (0.0, x, c),
                4 => (x, 0.0, c),
                5 => (c, 0.0, x),
                _ => (0.0, 0.0, 0.0),
            };
            let m = v - c;
            (r + m, g + m, b + m)
        };
        Rgb([
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
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

    fn to_rgb8(&self) -> Rgb<u8> {
        let (r, g, b) = if self.s == 0.0 {
            (self.l, self.l, self.l)
        } else {
            let h = self.h * 6.0;
            let s = self.s;
            let l = self.l;
            let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
            let x = c * (1.0 - (h % 2.0 - 1.0).abs());
            let (r, g, b) = match h.floor() as i32 % 6 {
                0 => (c, x, 0.0),
                1 => (x, c, 0.0),
                2 => (0.0, c, x),
                3 => (0.0, x, c),
                4 => (x, 0.0, c),
                5 => (c, 0.0, x),
                _ => (0.0, 0.0, 0.0),
            };
            let m = l - 0.5 * c;
            (r + m, g + m, b + m)
        };
        Rgb([
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
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
        if l > 0.0 && l < 1.0 {
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

    fn to_rgb8(&self) -> Rgb<u8> {
        let r;
        let g;
        let b;
        let h = self.h * 2.0 * std::f32::consts::PI;
        let s = self.s;
        let i = self.i;
        if h >= 0.0 && h < 2.0 * std::f32::consts::PI / 3.0 {
            b = i * (1.0 - s);
            r = i * (1.0 + s * (h.cos() / ((60.0f32).to_radians() - h).cos()));
            g = 3.0 * i - (r + b);
        } else if h >= 2.0 * std::f32::consts::PI / 3.0 && h < 4.0 * std::f32::consts::PI / 3.0 {
            let h = h - 2.0 * std::f32::consts::PI / 3.0;
            r = i * (1.0 - s);
            g = i * (1.0 + s * (h.cos() / ((60.0f32).to_radians() - h).cos()));
            b = 3.0 * i - (r + g);
        } else {
            let h = h - 4.0 * std::f32::consts::PI / 3.0;
            g = i * (1.0 - s);
            b = i * (1.0 + s * (h.cos() / ((60.0f32).to_radians() - h).cos()));
            r = 3.0 * i - (g + b);
        }
        Rgb([
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
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
        if s != 0.0 {
            h = 0.5 * ((r - g) + (r - b)) / ((r - g).powi(2) + (r - b) * (g - b)).sqrt();
            h = h.acos();
            if b > g {
                h = 2.0 * std::f32::consts::PI - h;
            }
            h = h / (2.0 * std::f32::consts::PI);
        }
        Hsi { h, s, i }
    }
}

pub struct Yuv {
    y: f32,
    u: f32,
    v: f32,
}

#[allow(dead_code)]
impl Yuv {
    fn as_rgb8(&self) -> Rgb<u8> {
        Rgb([
            (self.y * 255.0).round() as u8,
            ((self.u + 0.5) * 255.0).round() as u8,
            ((self.v + 0.5) * 255.0).round() as u8,
        ])
    }

    fn to_rgb8(&self) -> Rgb<u8> {
        let y = self.y;
        let u = self.u;
        let v = self.v;
        let r = y + 1.4075 * v;
        let g = y - 0.3455 * u - 0.7169 * v;
        let b = y + 1.7790 * u;
        Rgb([
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
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

pub fn hsv_to_rgb(image: &DynamicImage) -> DynamicImage {
    let mut out = RgbImage::new(image.width(), image.height());
    for (x, y, pixel) in image.pixels() {
        let rgb = Hsv {
            h: pixel[0] as f32 / 255.0,
            s: pixel[1] as f32 / 255.0,
            v: pixel[2] as f32 / 255.0,
        }
        .to_rgb8();
        out.put_pixel(x, y, rgb);
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

pub fn hsl_to_rgb(image: &DynamicImage) -> DynamicImage {
    let mut out = RgbImage::new(image.width(), image.height());
    for (x, y, pixel) in image.pixels() {
        let rgb = Hsl {
            h: pixel[0] as f32 / 255.0,
            s: pixel[1] as f32 / 255.0,
            l: pixel[2] as f32 / 255.0,
        }
        .to_rgb8();
        out.put_pixel(x, y, rgb);
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

pub fn hsi_to_rgb(image: &DynamicImage) -> DynamicImage {
    let mut out = RgbImage::new(image.width(), image.height());
    for (x, y, pixel) in image.pixels() {
        let rgb = Hsi {
            h: pixel[0] as f32 / 255.0,
            s: pixel[1] as f32 / 255.0,
            i: pixel[2] as f32 / 255.0,
        }
        .to_rgb8();
        out.put_pixel(x, y, rgb);
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

pub fn yuv_to_rgb(image: &DynamicImage) -> DynamicImage {
    let mut out = RgbImage::new(image.width(), image.height());
    for (x, y, pixel) in image.pixels() {
        let rgb = Yuv {
            y: pixel[0] as f32 / 255.0,
            u: (pixel[1] as f32 - 128.0) / 255.0,
            v: (pixel[2] as f32 - 128.0) / 255.0,
        }
        .to_rgb8();
        out.put_pixel(x, y, rgb);
    }
    DynamicImage::from(out)
}
