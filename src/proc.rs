use crate::draw::ImagePainter;

pub fn gray(raw_image: image::DynamicImage) -> Vec<ImagePainter> {
    let gray_image = raw_image.to_luma8();
    vec![
        ImagePainter::from(raw_image),
        ImagePainter::from(gray_image),
    ]
}
