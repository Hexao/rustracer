use image::{DynamicImage, GenericImageView, io::Reader};

use crate::material::{MatProvider, Material, Color};


pub struct Texture {
    image: DynamicImage,
    rep_x: f32,
    rep_y: f32,
    shininess: f32,
}

impl Texture {
    pub fn new(file_name: &str, rep_x: usize, rep_y: usize, shininess: f32) -> Self {
        let image = Reader::open(file_name).unwrap().decode().unwrap();
        assert!(rep_x > 0 && rep_y > 0);

        Self { image, rep_x: rep_x as f32, rep_y: rep_y as f32, shininess }
    }
}

impl MatProvider for Texture {
    fn material(&self, x: f32, y: f32) -> Material {
        let (w , h) = self.image.dimensions();

        let x = (x * self.rep_x % 1.0).min(1.0 - f32::EPSILON) * w as f32;
        let y = (y * self.rep_y % 1.0).min(1.0 - f32::EPSILON) * h as f32;
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
