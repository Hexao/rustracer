use image::{DynamicImage, GenericImageView, io::Reader};

use crate::material::{MatProvider, Material, Color};


pub struct Texture {
    image: DynamicImage,
    shininess: f32,
}

impl Texture {
    pub fn new(file_name: &str, shininess: f32) -> Self {
        let image = Reader::open(file_name).unwrap().decode().unwrap();

        Self { image, shininess }
    }
}

impl MatProvider for Texture {
    fn material(&self, x: f32, y: f32) -> Material {
        let x = x * (self.image.width() - 1) as f32;
        let y = y * (self.image.height() - 1) as f32;
        let pix = self.image.get_pixel(x as u32, y as u32).0;
        let color = Color::new(pix[0], pix[1], pix[2]);

        Material::new(
            color * 0.5,
            color,
            color * 1.5,
            pix[3],
            self.shininess
        )
    }
}
