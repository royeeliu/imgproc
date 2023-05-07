use image::{DynamicImage, GenericImageView, GrayImage, Pixel, Rgba, RgbaImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_line_segment_mut},
    rect::Rect,
};

use crate::{
    alg::{self, gray::average_gray_level, gray::histogram_equalize},
    draw::ImageDrawer,
};

pub fn gray(image: DynamicImage, color_space: Option<&String>) -> Vec<ImageDrawer> {
    let mut images = vec![image];
    let src_image = &images[0];

    match color_space {
        Some(str) => {
            let dst_image = match str.as_str() {
                "hsv" => Some(alg::color::rgb_to_hsv(&src_image)),
                "hsi" => Some(alg::color::rgb_to_hsi(&src_image)),
                "hsl" => Some(alg::color::rgb_to_hsl(&src_image)),
                "yuv" => Some(alg::color::rgb_to_yuv(&src_image)),
                _ => {
                    println!("Unknown color space: {}", str);
                    None
                }
            };
            if let Some(dst_image) = dst_image {
                let planes = alg::gray::split_planes(&dst_image);
                images.push(DynamicImage::from(dst_image));
                planes
                    .into_iter()
                    .for_each(|p| images.push(DynamicImage::from(p)));
            }
        }
        None => {
            images.push(DynamicImage::from(src_image.to_luma8()));
        }
    }
    images.into_iter().map(|i| ImageDrawer::from(i)).collect()
}

pub fn binary(image: DynamicImage, threshold: Option<u8>) -> Vec<ImageDrawer> {
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
    let equalized_image = histogram_equalize(&gray_image);

    let (hist_original, scale) = draw_histogram_scale(&image, None);
    let hist_equalized = draw_histogram_scale_gray(&equalized_image, Some(scale)).0;

    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(gray_image),
        ImageDrawer::from(equalized_image),
        ImageDrawer::from(hist_original),
        ImageDrawer::from(hist_equalized),
    ]
}
