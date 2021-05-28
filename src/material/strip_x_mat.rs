use crate::material::{MatProvider, Material};

use serde::{Deserialize, Deserializer, de::{Visitor, Error, MapAccess}};

#[derive(Clone, Copy)]
pub struct StripXMat {
    materials: [Material; 2],
    rep: f32,
}

impl StripXMat {
    pub fn new(mat_1: Material, mat_2: Material, rep: usize) -> Self {
        assert!(rep > 0);
        Self { materials: [mat_1, mat_2], rep: rep as f32 }
    }
}

impl MatProvider for StripXMat {
    fn material(&self, x: f32, _y: f32) -> Material {
        let x = x * self.rep % 1.0;

        if x <= 0.5 {
            self.materials[0]
        } else {
            self.materials[1]
        }
    }
}

impl<'de> Deserialize<'de> for StripXMat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const FIELDS: &[&str] = &["mat", "rep"];
        struct StripXMatVisitor;

        impl<'de> Visitor<'de> for StripXMatVisitor {
            type Value = StripXMat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StripXMat struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de>, {
                let mut material: Option<[Material; 2]> = None;
                let mut rep = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        "rep" => rep = Some(map.next_value()?),
                        "mat" => material = Some(map.next_value()?),
                        _ => return Err(Error::unknown_field(field, FIELDS))
                    }
                }

                let [mat_1, mat_2] = material.ok_or_else(|| Error::missing_field("mat"))?;
                let rep = rep.unwrap_or(1);

                Ok(StripXMat::new(mat_1, mat_2, rep))
            }
        }

        deserializer.deserialize_map(StripXMatVisitor)
    }
}
