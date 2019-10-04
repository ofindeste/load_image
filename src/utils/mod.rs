use image::{GenericImage, GenericImageView, DynamicImage};

pub trait ResizeImage {
    fn resize_to_exact(&self, x: u32, y: u32) -> Self;
}

impl ResizeImage for DynamicImage {
    fn resize_to_exact(&self, x: u32, y: u32) -> Self {
        let resized_image = self.resize(x, y, image::FilterType::Gaussian);

        let mut background = DynamicImage::new_rgba8(x, y);
        let (width, height) = resized_image.dimensions();
        if width < x {
            let margin = (x - width) / 2;
            background.copy_from(&resized_image, margin, 0);
        }
        if height < y {
            let margin = (y - height) / 2;
            background.copy_from(&resized_image, 0, margin);
        }
        background
    }
}