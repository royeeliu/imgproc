use image::GrayImage;

pub fn threshold(image: &GrayImage, level: u8) -> GrayImage {
    let mut out = image.clone();
    for p in out.iter_mut() {
        *p = if *p > level { 255 } else { 0 };
    }
    out
}

pub fn average_gray_level(image: &GrayImage) -> u8 {
    let pixels = image.pixels();
    let len = pixels.len() as u64;
    let sum: u64 = pixels.into_iter().map(|x| x.0[0] as u64).sum();
    (sum / len) as u8
}
