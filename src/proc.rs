use crate::{
    alg::{self, gray::average_gray_level},
    draw::ImagePainter,
};

pub fn gray(raw_image: image::DynamicImage) -> Vec<ImagePainter> {
    let gray_image = raw_image.to_luma8();
    vec![
        ImagePainter::from(raw_image),
        ImagePainter::from(gray_image),
    ]
}

pub fn binary(image: image::DynamicImage, threshold: Option<u8>) -> Vec<ImagePainter> {
    let gray_image = image.to_luma8();
    let level = match threshold {
        Some(v) => v,
        None => average_gray_level(&gray_image),
    };
    println!("Binary threshold: {}", level);
    let binary_image = alg::gray::threshold(&gray_image, level);
    vec![
        ImagePainter::from(image),
        ImagePainter::from(gray_image),
        ImagePainter::from(binary_image),
    ]
}
