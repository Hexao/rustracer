pub mod directional_light;
pub mod point_light;

use crate::material::Color;
use crate::object::Movable;
use crate::math::{
    point::Point,
    ray::Ray
};

use serde::{Deserialize, Deserializer, de::{Visitor, Error, MapAccess}};

pub trait Light: Movable {
    fn vec_from_light(&self, point: &Point) -> Point {
        let vec = self.local_to_global_vector(&self.global_to_local_point(point));
        vec.normalized()
    }

    fn vec_to_light(&self, point: &Point) -> Point {
        let vec = self.local_to_global_vector(&-self.global_to_local_point(point));
        vec.normalized()
    }

    fn ray_from_light(&self, point: &Point) -> Ray {
        let local = self.global_to_local_point(point);
        let ray = Ray::new(Point::default(), local);

        self.local_to_global_ray(&ray).normalized()
    }

    fn ray_to_light(&self, point: &Point) -> Ray {
        let local = self.global_to_local_point(point);
        let ray = Ray::new(local, -local);

        self.local_to_global_ray(&ray).normalized()
    }

    fn distance(&self, to: &Point) -> f32 {
        let point = self.global_to_local_point(to);
        point.norm()
    }

    fn illuminate(&self, point: &Point) -> bool;

    fn diffuse(&self) -> Color;
    fn specular(&self) -> Color;
}

impl<'de> Deserialize<'de> for Box<dyn Light> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const FIELDS: &[&str] = &["type", "color", "transform", "rotate", "scale"];
        const TYPES: &[&str] = &["DIRECTIONAL", "POINT"];
        struct LightVisitor;

        impl<'de> Visitor<'de> for LightVisitor {
            type Value = Box<dyn Light>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Light struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
                struct LightColor(Color, Color);

                impl<'de> Deserialize<'de> for LightColor {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
                        const FIELDS: &[&str] = &["diffuse", "specular"];
                        struct LightColorVisitor;

                        impl<'de> Visitor<'de> for LightColorVisitor {
                            type Value = LightColor;

                            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                                formatter.write_str("Light color struct")
                            }

                            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de>, {
                                let mut diffuse = None;
                                let mut specular = None;

                                while let Some(field) = map.next_key()? {
                                    match field {
                                        "diffuse" => diffuse = Some(map.next_value()?),
                                        "specular" => specular = Some(map.next_value()?),
                                        _ => return Err(Error::unknown_field(field, FIELDS)),
                                    }
                                }

                                let diffuse = diffuse.ok_or_else(|| Error::missing_field("diffuse"))?;
                                let specular = specular.ok_or_else(|| Error::missing_field("specular"))?;
                                Ok(LightColor(diffuse, specular))
                            }
                        }

                        deserializer.deserialize_map(LightColorVisitor)
                    }
                }

                let mut light_type = None;
                let mut color = None;
                let mut transform = None;
                let mut rotate = None;
                let mut scale = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        "type" => light_type = Some(map.next_value()?),
                        "color" => color = Some(map.next_value()?),
                        "transform" => transform = Some(map.next_value()?),
                        "rotate" => rotate = Some(map.next_value()?),
                        "scale" => scale = Some(map.next_value()?),
                        _ => return Err(Error::unknown_field(field, FIELDS)),
                    }
                }

                let light_type = light_type.ok_or_else(|| Error::missing_field("type"))?;
                let LightColor(diffuse, specular) = color.ok_or_else(|| Error::missing_field("color"))?;

                let mut light: Box<dyn Light> = match light_type {
                    "DIRECTIONAL" => Box::new(directional_light::DirectionalLight::new(diffuse, specular)),
                    "POINT" => Box::new(point_light::PointLight::new(diffuse, specular)),
                    _ => return Err(Error::unknown_variant(light_type, TYPES)),
                };

                if let Some(Point {x, y , z}) = transform {
                    light.move_global(x, y, z);
                }

                if let Some(Point {x, y , z}) = rotate {
                    light.rotate_x(x);
                    light.rotate_y(y);
                    light.rotate_z(z);
                }

                if let Some(scale) = scale {
                    light.scale(scale);
                }

                Ok(light)
            }
        }

        deserializer.deserialize_map(LightVisitor)
    }
}
