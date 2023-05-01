use crate::{
    alg::{self, gray::average_gray_level},
    draw::ImageDrawer,
};

pub fn gray(raw_image: image::DynamicImage) -> Vec<ImageDrawer> {
    let gray_image = raw_image.to_luma8();
    vec![ImageDrawer::from(raw_image), ImageDrawer::from(gray_image)]
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
