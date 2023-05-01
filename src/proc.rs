use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::draw_line_segment_mut;

use crate::{
    alg::{self, gray::average_gray_level},
    draw::ImageDrawer,
};

pub fn gray(image: image::DynamicImage) -> Vec<ImageDrawer> {
    let gray_image = image.to_luma8();
    vec![ImageDrawer::from(image), ImageDrawer::from(gray_image)]
}

pub fn binary(image: image::DynamicImage, threshold: Option<u8>) -> Vec<ImageDrawer> {
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

pub fn histogram(image: image::DynamicImage) -> Vec<ImageDrawer> {
    let gray_image = image.to_luma8();
    let background = Rgba([255u8, 255u8, 255u8, 255u8]);
    let gray = Rgba([128u8, 128u8, 128u8, 255u8]);
    let black = Rgba([0u8, 0u8, 0u8, 255u8]);
    let red = Rgba([255u8, 0u8, 0u8, 255u8]);
    let green = Rgba([0u8, 255u8, 0u8, 255u8]);
    let blue = Rgba([0u8, 0u8, 255u8, 255u8]);

    let mut gray_count = [0u64; 256];
    let mut red_count = [0u64; 256];
    let mut green_count = [0u64; 256];
    let mut blue_count = [0u64; 256];
    for pixel in gray_image.pixels() {
        gray_count[pixel.0[0] as usize] += 1;
    }
    for (_, _, pixel) in image.pixels() {
        red_count[pixel.0[0] as usize] += 1;
        green_count[pixel.0[1] as usize] += 1;
        blue_count[pixel.0[2] as usize] += 1;
    }
    let max = [
        gray_count.iter().map(|v| *v).max().unwrap(),
        red_count.iter().map(|v| *v).max().unwrap(),
        green_count.iter().map(|v| *v).max().unwrap(),
        blue_count.iter().map(|v| *v).max().unwrap(),
    ]
    .iter()
    .map(|v| *v)
    .max()
    .unwrap();
    let scale = 127.0 / max as f32;

    let mut hist_image = RgbaImage::from_pixel(512, 512, background);
    draw_line_segment_mut(&mut hist_image, (0.0, 128.0), (511.0, 128.0), gray);
    draw_line_segment_mut(&mut hist_image, (0.0, 256.0), (511.0, 256.0), gray);
    draw_line_segment_mut(&mut hist_image, (0.0, 382.0), (511.0, 382.0), gray);

    for (i, v) in red_count.iter().enumerate() {
        let x = i as f32 * 2.0;
        let h = *v as f32 * scale;
        if h > 0.001 {
            draw_line_segment_mut(&mut hist_image, (x, 127.0 - h), (x, 127.0), red);
        }
    }
    for (i, v) in green_count.iter().enumerate() {
        let x = i as f32 * 2.0;
        let h = *v as f32 * scale;
        if h > 0.001 {
            draw_line_segment_mut(&mut hist_image, (x, 255.0 - h), (x, 255.0), green);
        }
    }
    for (i, v) in blue_count.iter().enumerate() {
        let x = i as f32 * 2.0;
        let h = *v as f32 * scale;
        if h > 0.001 {
            draw_line_segment_mut(&mut hist_image, (x, 381.0 - h), (x, 381.0), blue);
        }
    }
    for (i, v) in gray_count.iter().enumerate() {
        let x = i as f32 * 2.0;
        let h = *v as f32 * scale;
        if h > 0.001 {
            draw_line_segment_mut(&mut hist_image, (x, 511.0 - h), (x, 511.0), black);
        }
    }

    let gray_image = image.to_luma8();
    vec![
        ImageDrawer::from(image),
        ImageDrawer::from(gray_image),
        ImageDrawer::from(DynamicImage::from(hist_image)),
    ]
}
