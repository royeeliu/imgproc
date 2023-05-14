use image::{DynamicImage, GenericImageView, GrayImage, Pixel, Rgba, RgbaImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_line_segment_mut},
    rect::Rect,
};

use crate::{
    alg::{
        self,
        color::*,
        gray::histogram_equalize,
        gray::{average_gray_level, split_planes},
    },
    draw::ImageDrawer,
};

pub fn grayscale(image: DynamicImage, color_space: Option<&String>) -> Vec<ImageDrawer> {
    let mut images = vec![image];
    let src_image = &images[0];

    match color_space {
        Some(str) => {
            let (dst_image, recovered) = match str.as_str() {
                "hsv" => {
                    let hsv = rgb_to_hsv(&src_image);
                    let recovered = hsv_to_rgb(&hsv);
                    (Some(hsv), Some(recovered))
                }
                "hsi" => {
                    let hsi = rgb_to_hsi(&src_image);
                    let recovered = hsi_to_rgb(&hsi);
                    (Some(hsi), Some(recovered))
                }
                "hsl" => {
                    let hsl = rgb_to_hsl(&src_image);
                    let recovered = hsl_to_rgb(&hsl);
                    (Some(hsl), Some(recovered))
                }
                "yuv" => {
                    let yuv = rgb_to_yuv(&src_image);
                    let recovered = yuv_to_rgb(&yuv);
                    (Some(yuv), Some(recovered))
                }
                "rgb" => (Some(src_image.clone()), None),
                _ => {
                    println!("Unknown color space: {}", str);
                    (None, None)
                }
            };
            if let Some(dst_image) = dst_image {
                let planes = alg::gray::split_planes(&dst_image);
                images.push(DynamicImage::from(dst_image));
                planes
                    .into_iter()
                    .for_each(|p| images.push(DynamicImage::from(p)));
                if let Some(recovered) = recovered {
                    images.push(DynamicImage::from(recovered));
                }
            }
        }
        None => {
            images.push(DynamicImage::from(src_image.to_luma8()));
        }
    }
    images.into_iter().map(|i| ImageDrawer::from(i)).collect()
}

pub fn binarize(image: DynamicImage, threshold: Option<u8>) -> Vec<ImageDrawer> {
    let gray_image = image.to_luma8();
    let level = match threshold {
        Some(v) => v,
        None => average_gray_level(&gray_image),
    };
    println!("Binary threshold: {}", level);
    let binary_image = alg::gray::threshold(&gray_image, level);
    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(gray_image),
        ImageDrawer::from(binary_image),
    ]
}

fn create_histogram_canvas() -> RgbaImage {
    let background = Rgba([255u8, 255u8, 255u8, 255u8]);
    let gray = Rgba([128u8, 128u8, 128u8, 255u8]);
    let mut image = RgbaImage::from_pixel(512, 512, background);
    draw_line_segment_mut(&mut image, (0.0, 127.0), (511.0, 127.0), gray);
    draw_line_segment_mut(&mut image, (0.0, 255.0), (511.0, 255.0), gray);
    draw_line_segment_mut(&mut image, (0.0, 383.0), (511.0, 383.0), gray);
    draw_line_segment_mut(&mut image, (0.0, 511.0), (511.0, 511.0), gray);
    image
}

fn draw_histogram_part(
    canvas: &mut RgbaImage,
    values: &[u64; 256],
    scale: u64,
    vertical_range: (i32, i32),
    color: Rgba<u8>,
) {
    let height = (vertical_range.1 - vertical_range.0) as u64;
    for (i, v) in values.iter().enumerate() {
        let x = (i * 2) as i32;
        let h = ((v * height + (scale / 2)) / scale) as i32;
        if h > 0 {
            let rect = Rect::at(x, vertical_range.1 - h).of_size(2, h as u32);
            draw_filled_rect_mut(canvas, rect, color);
        }
    }
}

fn draw_histogram_scale_gray(image: &GrayImage, scale: Option<u64>) -> (DynamicImage, u64) {
    let black = Rgba([0u8, 0u8, 0u8, 255u8]);
    let mut gray_count = [0u64; 256];
    for luma in image.pixels() {
        gray_count[luma.0[0] as usize] += 1;
    }
    let scale = if let Some(v) = scale {
        v
    } else {
        gray_count.iter().map(|v| *v).max().unwrap()
    };

    let mut canvas = create_histogram_canvas();
    draw_histogram_part(&mut canvas, &gray_count, scale, (384, 511), black);
    (DynamicImage::from(canvas), scale)
}

fn draw_histogram_scale(image: &DynamicImage, scale: Option<u64>) -> (DynamicImage, u64) {
    let black = Rgba([0u8, 0u8, 0u8, 255u8]);
    let red = Rgba([255u8, 0u8, 0u8, 255u8]);
    let green = Rgba([0u8, 255u8, 0u8, 255u8]);
    let blue = Rgba([0u8, 0u8, 255u8, 255u8]);

    let mut gray_count = [0u64; 256];
    let mut red_count = [0u64; 256];
    let mut green_count = [0u64; 256];
    let mut blue_count = [0u64; 256];
    for (_, _, pixel) in image.pixels() {
        let luma = pixel.to_luma();
        gray_count[luma.0[0] as usize] += 1;
        red_count[pixel.0[0] as usize] += 1;
        green_count[pixel.0[1] as usize] += 1;
        blue_count[pixel.0[2] as usize] += 1;
    }
    let scale = if let Some(v) = scale {
        v
    } else {
        let max = [
            gray_count.iter().map(|v| *v).max().unwrap(),
            red_count.iter().map(|v| *v).max().unwrap(),
            green_count.iter().map(|v| *v).max().unwrap(),
            blue_count.iter().map(|v| *v).max().unwrap(),
        ]
        .into_iter()
        .max()
        .unwrap();
        max
    };

    let mut canvas = create_histogram_canvas();
    draw_histogram_part(&mut canvas, &red_count, scale, (0, 127), red);
    draw_histogram_part(&mut canvas, &green_count, scale, (128, 255), green);
    draw_histogram_part(&mut canvas, &blue_count, scale, (256, 383), blue);
    draw_histogram_part(&mut canvas, &gray_count, scale, (384, 511), black);

    (DynamicImage::from(canvas), scale)
}

pub fn histogram(image: DynamicImage) -> Vec<ImageDrawer> {
    let gray_image = image.to_luma8();
    let (hist_original, _) = draw_histogram_scale(&image, None);

    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(gray_image),
        ImageDrawer::from(hist_original),
    ]
}

fn equalize_grayscale_luma(image: DynamicImage) -> Vec<ImageDrawer> {
    let grayscale = image.to_luma8();
    let equalized = histogram_equalize(&grayscale);
    let (hist_original, scale) = draw_histogram_scale(&image, None);
    let hist_equalized = draw_histogram_scale_gray(&equalized, Some(scale)).0;

    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(grayscale),
        ImageDrawer::from(equalized),
        ImageDrawer::from(hist_original),
        ImageDrawer::from(hist_equalized),
    ]
}

fn equalize_grayscale_value(image: DynamicImage) -> Vec<ImageDrawer> {
    let hsv = rgb_to_hsv(&image);
    let planes = split_planes(&hsv);
    let grayscale = planes[2].clone();
    let equalized = histogram_equalize(&grayscale);
    let (hist_original, scale) = draw_histogram_scale(&image, None);
    let hist_grayscale = draw_histogram_scale_gray(&grayscale, Some(scale)).0;
    let hist_equalized = draw_histogram_scale_gray(&equalized, Some(scale)).0;

    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(grayscale),
        ImageDrawer::from(equalized),
        ImageDrawer::from(hist_original),
        ImageDrawer::from(hist_grayscale),
        ImageDrawer::from(hist_equalized),
    ]
}

fn equalize_grayscale_lightness(image: DynamicImage) -> Vec<ImageDrawer> {
    let hsl = rgb_to_hsl(&image);
    let planes = split_planes(&hsl);
    let grayscale = planes[2].clone();
    let equalized = histogram_equalize(&grayscale);
    let (hist_original, scale) = draw_histogram_scale(&image, None);
    let hist_grayscale = draw_histogram_scale_gray(&grayscale, Some(scale)).0;
    let hist_equalized = draw_histogram_scale_gray(&equalized, Some(scale)).0;

    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(grayscale),
        ImageDrawer::from(equalized),
        ImageDrawer::from(hist_original),
        ImageDrawer::from(hist_grayscale),
        ImageDrawer::from(hist_equalized),
    ]
}

fn equalize_grayscale_intensity(image: DynamicImage) -> Vec<ImageDrawer> {
    let hsi = rgb_to_hsi(&image);
    let planes = split_planes(&hsi);
    let grayscale = planes[2].clone();
    let equalized = histogram_equalize(&grayscale);
    let (hist_original, scale) = draw_histogram_scale(&image, None);
    let hist_grayscale = draw_histogram_scale_gray(&grayscale, Some(scale)).0;
    let hist_equalized = draw_histogram_scale_gray(&equalized, Some(scale)).0;

    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(grayscale),
        ImageDrawer::from(equalized),
        ImageDrawer::from(hist_original),
        ImageDrawer::from(hist_grayscale),
        ImageDrawer::from(hist_equalized),
    ]
}

fn equalize_color_hsv(image: DynamicImage) -> Vec<ImageDrawer> {
    let hsv = rgb_to_hsv(&image);
    let planes = split_planes(&hsv);
    let grayscale = histogram_equalize(&planes[2]);
    let mut equalized = RgbaImage::new(hsv.width(), hsv.height());
    for (x, y, mut pixel) in hsv.pixels() {
        pixel.0[2] = grayscale.get_pixel(x, y).0[0];
        equalized.put_pixel(x, y, pixel);
    }
    let equalized = hsv_to_rgb(&equalized.into());
    let (hist_original, scale) = draw_histogram_scale(&image, None);
    let hist_equalized = draw_histogram_scale(&equalized, Some(scale)).0;

    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(equalized),
        ImageDrawer::from(hist_original),
        ImageDrawer::from(hist_equalized),
    ]
}

fn equalize_color_hsi(image: DynamicImage) -> Vec<ImageDrawer> {
    let hsv = rgb_to_hsi(&image);
    let planes = split_planes(&hsv);
    let value_plane = histogram_equalize(&planes[2]);
    let mut equalized = RgbaImage::new(hsv.width(), hsv.height());
    for (x, y, mut pixel) in hsv.pixels() {
        pixel.0[2] = value_plane.get_pixel(x, y).0[0];
        equalized.put_pixel(x, y, pixel);
    }
    let equalized = hsi_to_rgb(&equalized.into());
    let (hist_original, scale) = draw_histogram_scale(&image, None);
    let hist_equalized = draw_histogram_scale(&equalized, Some(scale)).0;

    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(equalized),
        ImageDrawer::from(hist_original),
        ImageDrawer::from(hist_equalized),
    ]
}

fn equalize_color_hsl(image: DynamicImage) -> Vec<ImageDrawer> {
    let hsv = rgb_to_hsl(&image);
    let planes = split_planes(&hsv);
    let value_plane = histogram_equalize(&planes[2]);
    let mut equalized = RgbaImage::new(hsv.width(), hsv.height());
    for (x, y, mut pixel) in hsv.pixels() {
        pixel.0[2] = value_plane.get_pixel(x, y).0[0];
        equalized.put_pixel(x, y, pixel);
    }
    let equalized = hsl_to_rgb(&equalized.into());
    let (hist_original, scale) = draw_histogram_scale(&image, None);
    let hist_equalized = draw_histogram_scale(&equalized, Some(scale)).0;

    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(equalized),
        ImageDrawer::from(hist_original),
        ImageDrawer::from(hist_equalized),
    ]
}

fn equalize_color_yuv(image: DynamicImage) -> Vec<ImageDrawer> {
    let hsv = rgb_to_yuv(&image);
    let planes = split_planes(&hsv);
    let value_plane = histogram_equalize(&planes[0]);
    let mut equalized = RgbaImage::new(hsv.width(), hsv.height());
    for (x, y, mut pixel) in hsv.pixels() {
        pixel.0[0] = value_plane.get_pixel(x, y).0[0];
        equalized.put_pixel(x, y, pixel);
    }
    let equalized = yuv_to_rgb(&equalized.into());
    let (hist_original, scale) = draw_histogram_scale(&image, None);
    let hist_equalized = draw_histogram_scale(&equalized, Some(scale)).0;

    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(equalized),
        ImageDrawer::from(hist_original),
        ImageDrawer::from(hist_equalized),
    ]
}

fn equalize_color_rgb(image: DynamicImage) -> Vec<ImageDrawer> {
    let planes = split_planes(&image);
    let red_plane = histogram_equalize(&planes[0]);
    let green_plane = histogram_equalize(&planes[1]);
    let blue_plane = histogram_equalize(&planes[2]);
    let mut equalized = RgbaImage::new(image.width(), image.height());
    for (x, y, mut pixel) in image.pixels() {
        pixel.0[0] = red_plane.get_pixel(x, y).0[0];
        pixel.0[1] = green_plane.get_pixel(x, y).0[0];
        pixel.0[2] = blue_plane.get_pixel(x, y).0[0];
        equalized.put_pixel(x, y, pixel);
    }
    let equalized = DynamicImage::from(equalized);
    let (hist_original, scale) = draw_histogram_scale(&image, None);
    let hist_equalized = draw_histogram_scale(&equalized, Some(scale)).0;

    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(equalized),
        ImageDrawer::from(hist_original),
        ImageDrawer::from(hist_equalized),
    ]
}

pub fn equalize(
    image: DynamicImage,
    grayscale_only: bool,
    color_space: Option<&String>,
) -> Vec<ImageDrawer> {
    match color_space {
        Some(str) => match str.as_str() {
            "hsv" => {
                if grayscale_only {
                    equalize_grayscale_value(image)
                } else {
                    equalize_color_hsv(image)
                }
            }
            "hsi" => {
                if grayscale_only {
                    equalize_grayscale_intensity(image)
                } else {
                    equalize_color_hsi(image)
                }
            }
            "hsl" => {
                if grayscale_only {
                    equalize_grayscale_lightness(image)
                } else {
                    equalize_color_hsl(image)
                }
            }
            "yuv" => {
                if grayscale_only {
                    equalize_grayscale_luma(image)
                } else {
                    equalize_color_yuv(image)
                }
            }
            "rgb" => {
                if grayscale_only {
                    equalize_grayscale_luma(image)
                } else {
                    equalize_color_rgb(image)
                }
            }
            _ => {
                panic!("Unknown color space: {}", str);
            }
        },
        None => {
            if grayscale_only {
                equalize_grayscale_luma(image)
            } else {
                equalize_color_hsi(image)
            }
        }
    }
}

pub fn invert(image: DynamicImage) -> Vec<ImageDrawer> {
    let mut inverse = RgbaImage::new(image.width(), image.height());
    for (x, y, mut pixel) in image.pixels() {
        pixel.0[0] = 255 - pixel.0[0];
        pixel.0[1] = 255 - pixel.0[1];
        pixel.0[2] = 255 - pixel.0[2];
        inverse.put_pixel(x, y, pixel);
    }
    let inverse = DynamicImage::from(inverse);
    let (hist_original, scale) = draw_histogram_scale(&image, None);
    let hist_inverse = draw_histogram_scale(&inverse, Some(scale)).0;

    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(inverse),
        ImageDrawer::from(hist_original),
        ImageDrawer::from(hist_inverse),
    ]
}
