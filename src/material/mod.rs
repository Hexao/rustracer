pub mod strip_x_mat;
pub mod strip_y_mat;
pub mod simple_mat;
pub mod grid_mat;
pub mod texture;

use serde::{Deserialize, Deserializer, de::{Visitor, Error, Unexpected, SeqAccess, MapAccess, value::MapAccessDeserializer}};
use std::ops::{Mul, Add, AddAssign, Sub};

pub trait MatProvider {
    fn material(&self, x: f32, y: f32) -> Material;
}

impl<'de> Deserialize<'de> for Box<dyn MatProvider> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const TYPES: &[&str] = &["SIMPLE", "STRIP_X", "STRIP_Y", "GRID", "TEXTURE"];
        struct MatVisitor;

        impl<'de> Visitor<'de> for MatVisitor {
            type Value = Box<dyn MatProvider>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Material struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
                match map.next_key()? {
                    Some("type") => {
                        let value = map.next_value()?;
                        let des = MapAccessDeserializer::new(map);

                        match value {
                            "SIMPLE" => {
                                let simple_mat: simple_mat::SimpleMat = Deserialize::deserialize(des)?;
                                let boxed: Box<dyn MatProvider> = Box::new(simple_mat);
                                Ok(boxed)
                            }
                            "STRIP_X" => {
                                let strip_x: strip_x_mat::StripXMat = Deserialize::deserialize(des)?;
                                let boxed: Box<dyn MatProvider> = Box::new(strip_x);
                                Ok(boxed)
                            }
                            "STRIP_Y" => {
                                let strip_y: strip_y_mat::StripYMat = Deserialize::deserialize(des)?;
                                let boxed: Box<dyn MatProvider> = Box::new(strip_y);
                                Ok(boxed)
                            }
                            "GRID" => {
                                let grid: grid_mat::GridMat = Deserialize::deserialize(des)?;
                                let boxed: Box<dyn MatProvider> = Box::new(grid);
                                Ok(boxed)
                            }
                            "TEXTURE" => {
                                let texture: texture::Texture = Deserialize::deserialize(des)?;
                                let boxed: Box<dyn MatProvider> = Box::new(texture);
                                Ok(boxed)
                            }
                            _ => Err(Error::unknown_variant(value, TYPES)),
                        }
                    }
                    _ => Err(Error::custom("Expected `type` key as first")),
                }
            }
        }

        deserializer.deserialize_map(MatVisitor)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub const SKY: Color = Color { red: 50, green: 120, blue: 170};

    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn new_gray(gray: u8) -> Self {
        Self { red: gray, green: gray, blue: gray }
    }

    pub fn to_vec(&self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new_gray(0)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: ((self.red as f32 / 255.0) * (rhs.red as f32 / 255.0) * 255.0).clamp(0.0, 255.0) as u8,
            green: ((self.green as f32 / 255.0) * (rhs.green as f32 / 255.0) * 255.0).clamp(0.0, 255.0) as u8,
            blue: ((self.blue as f32 / 255.0) * (rhs.blue as f32 / 255.0) * 255.0).clamp(0.0, 255.0) as u8,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, coef: f32) -> Self::Output {
        Self {
            red: (self.red as f32 * coef).clamp(0.0, 255.0) as u8,
            green: (self.green as f32 * coef).clamp(0.0, 255.0) as u8,
            blue: (self.blue as f32 * coef).clamp(0.0, 255.0) as u8,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red.saturating_add(rhs.red),
            green: self.green.saturating_add(rhs.green),
            blue: self.blue.saturating_add(rhs.blue),
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.red = self.red.saturating_add(rhs.red);
        self.green = self.green.saturating_add(rhs.green);
        self.blue = self.blue.saturating_add(rhs.blue);
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red.saturating_sub(rhs.red),
            green: self.green.saturating_sub(rhs.green),
            blue: self.blue.saturating_sub(rhs.blue),
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct ColorVisitor;

        impl<'de> Visitor<'de> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("u8 or array of size 3")
            }

            fn visit_u64<E: Error>(self, gray: u64) -> Result<Self::Value, E> {
                if gray <= u64::from(u8::MAX) {
                    Ok(Color::new_gray(gray as u8))
                } else {
                    Err(Error::invalid_type(Unexpected::Unsigned(gray), &self))
                }
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'de> {
                let red = seq.next_element()?.ok_or_else(|| Error::invalid_length(0, &self))?;
                let green = seq.next_element()?.ok_or_else(|| Error::invalid_length(1, &self))?;
                let blue = seq.next_element()?.ok_or_else(|| Error::invalid_length(2, &self))?;

                Ok(Color::new(red, green, blue))
            }
        }

        deserializer.deserialize_any(ColorVisitor)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,

    pub alpha: u8,
    pub reflection: u8,
    pub shininess: f32,
}

impl Material {
    pub fn new(ambient: Color, diffuse: Color, specular: Color, alpha: u8, reflection: u8, shininess: f32) -> Self {
        Self { ambient, diffuse, specular, alpha, reflection, shininess }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: Color::new_gray(63),
            diffuse: Color::new_gray(127),
            specular: Color::new_gray(191),
            alpha: 255,
            reflection: 0,
            shininess: 50.0,
        }
    }
}

impl<'de> Deserialize<'de> for Material {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const FIELDS: &[&str] = &["ambient", "diffuse", "specular", "alpha", "reflection", "shininess"];
        struct MatVisitor;

        impl<'de> Visitor<'de> for MatVisitor {
            type Value = Material;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Material struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
                let mut ambient = None;
                let mut diffuse = None;
                let mut specular = None;
                let mut alpha = None;
                let mut reflection = None;
                let mut shininess = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        "ambient" => ambient = Some(map.next_value()?),
                        "diffuse" => diffuse = Some(map.next_value()?),
                        "specular" => specular = Some(map.next_value()?),
                        "alpha" => alpha = Some(map.next_value()?),
                        "reflection" => reflection = Some(map.next_value()?),
                        "shininess" => shininess = Some(map.next_value()?),
                        _ => return Err(Error::unknown_field(field, FIELDS))
                    }
                }

                let ambient = ambient.ok_or_else(|| Error::missing_field("ambient"))?;
                let diffuse = diffuse.ok_or_else(|| Error::missing_field("diffuse"))?;
                let specular = specular.ok_or_else(|| Error::missing_field("specular"))?;
                let alpha = alpha.unwrap_or(255);
                let reflection = reflection.unwrap_or(0);
                let shininess = shininess.unwrap_or(50.0);

                Ok(Material::new(ambient, diffuse, specular, alpha, reflection, shininess))
            }
        }

        deserializer.deserialize_map(MatVisitor)
    }
}
