use crate::material::{MatProvider, Material};

use serde::{Deserialize, Deserializer, de::{Visitor, Error, MapAccess}};

#[derive(Clone, Copy)]
pub struct SimpleMat {
    material: Material
}

impl SimpleMat {
    pub fn new(material: Material) -> Self {
        Self { material }
    }
}

impl MatProvider for SimpleMat {
    fn material(&self, _x: f32, _y: f32) -> Material {
        self.material
    }
}

impl<'de> Deserialize<'de> for SimpleMat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const FIELDS: &[&str] = &["mat"];
        struct SimpleMatVisitor;

        impl<'de> Visitor<'de> for SimpleMatVisitor {
            type Value = SimpleMat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SimpleMat struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de>, {
                let mut material = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        "mat" => material = Some(map.next_value()?),
                        _ => return Err(Error::unknown_field(field, FIELDS))
                    }
                }

                let material = material.ok_or_else(|| Error::missing_field("mat"))?;
                Ok(SimpleMat::new(material))
            }
        }

        deserializer.deserialize_map(SimpleMatVisitor)
    }
}
