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

pub fn histogram_equalize(image: &GrayImage) -> GrayImage {
    let mut gray_count = [0u64; 256];
    for luma in image.pixels() {
        gray_count[luma.0[0] as usize] += 1;
    }

    let mut gray_map = [0u8; 256];
    let len = image.width() as u64 * image.height() as u64;
    let mut sum = 0u64;
    for i in 0..256 {
        sum += gray_count[i];
        gray_map[i] = ((sum * 255 + len / 2) / len) as u8;
    }

    let mut out = image.clone();
    for luma in out.iter_mut() {
        *luma = gray_map[*luma as usize];
    }
    out
}
