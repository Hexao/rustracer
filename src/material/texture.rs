use crate::material::{MatProvider, Material, Color};

use serde::{Deserialize, Deserializer, de::{Visitor, Error, MapAccess}};
use image::{DynamicImage, GenericImageView, io::Reader};

pub struct Texture {
    image: DynamicImage,
    rep_x: f32,
    rep_y: f32,
    reflection: u8,
    shininess: f32,
}

impl Texture {
    pub fn new(file_name: &str, rep_x: usize, rep_y: usize, reflection: u8, shininess: f32) -> Self {
        let image = Reader::open(file_name).unwrap().decode().unwrap();
        assert!(rep_x > 0 && rep_y > 0);

        Self { image, rep_x: rep_x as f32, rep_y: rep_y as f32, reflection, shininess }
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
            self.reflection,
            self.shininess
        )
    }
}

impl<'de> Deserialize<'de> for Texture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const FIELDS: &[&str] = &["resource", "rep[X|Y]", "reflection", "shininess"];
        struct TextureVisitor;

        impl<'de> Visitor<'de> for TextureVisitor {
            type Value = Texture;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Texture struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de>, {
                let mut file = None;
                let mut rep_x = None;
                let mut rep_y = None;
                let mut shininess = None;
                let mut reflection = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        "rep" => {
                            let [x, y]: [usize; 2] = map.next_value()?;
                            rep_x = Some(x);
                            rep_y = Some(y);
                        }
                        "repX" => rep_x = Some(map.next_value()?),
                        "repY" => rep_y = Some(map.next_value()?),
                        "resource" => file = Some(map.next_value()?),
                        "shininess" => shininess = Some(map.next_value()?),
                        "reflection" => reflection = Some(map.next_value()?),
                        _ => return Err(Error::unknown_field(field, FIELDS))
                    }
                }

                let file_name = file.ok_or_else(|| Error::missing_field("resource"))?;
                let reflection = reflection.unwrap_or(0);
                let shininess = shininess.unwrap_or(50.0);
                let rep_x = rep_x.unwrap_or(1);
                let rep_y = rep_y.unwrap_or(1);

                Ok(Texture::new(file_name, rep_x, rep_y, reflection, shininess))
            }
        }

        deserializer.deserialize_map(TextureVisitor)
    }
}
