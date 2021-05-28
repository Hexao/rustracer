use crate::material::{MatProvider, Material};

use serde::{Deserialize, Deserializer, de::{Visitor, Error, MapAccess}};

#[derive(Clone, Copy)]
pub struct StripYMat {
    materials: [Material; 2],
    rep: f32,
}

impl StripYMat {
    pub fn new(mat_1: Material, mat_2: Material, rep: usize) -> Self {
        assert!(rep > 0);
        Self { materials: [mat_1, mat_2], rep: rep as f32 }
    }
}

impl MatProvider for StripYMat {
    fn material(&self, _x: f32, y: f32) -> Material {
        let y = y * self.rep % 1.0;

        if y <= 0.5 {
            self.materials[0]
        } else {
            self.materials[1]
        }
    }
}

impl<'de> Deserialize<'de> for StripYMat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const FIELDS: &[&str] = &["mat", "rep"];
        struct StripYMatVisitor;

        impl<'de> Visitor<'de> for StripYMatVisitor {
            type Value = StripYMat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StripYMat struct")
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

                Ok(StripYMat::new(mat_1, mat_2, rep))
            }
        }

        deserializer.deserialize_map(StripYMatVisitor)
    }
}
