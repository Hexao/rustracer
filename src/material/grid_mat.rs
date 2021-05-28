use crate::material::{MatProvider, Material};

use serde::{Deserialize, Deserializer, de::{Visitor, Error, MapAccess}};

#[derive(Clone, Copy)]
pub struct GridMat {
    materials: [Material; 2],
    rep_x: f32,
    rep_y: f32,
}

impl GridMat {
    pub fn new(mat_1: Material, mat_2: Material, rep_x: usize, rep_y: usize) -> Self {
        assert!(rep_x > 0 && rep_y > 0);
        Self { materials: [mat_1, mat_2], rep_x: rep_x as f32, rep_y: rep_y as f32 }
    }
}

impl MatProvider for GridMat {
    fn material(&self, x: f32, y: f32) -> Material {
        let x = x * self.rep_x % 1.0;
        let y = y * self.rep_y % 1.0;

        if (x <= 0.5) ^ (y <= 0.5) {
            self.materials[0]
        } else {
            self.materials[1]
        }
    }
}

impl<'de> Deserialize<'de> for GridMat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const FIELDS: &[&str] = &["mat", "rep[X|Y]"];
        struct GridMatVisitor;

        impl<'de> Visitor<'de> for GridMatVisitor {
            type Value = GridMat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StripXMat struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de>, {
                let mut material: Option<[Material; 2]> = None;
                let mut rep_x = None;
                let mut rep_y = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        "rep" => {
                            let [x, y]: [usize; 2] = map.next_value()?;
                            rep_x = Some(x);
                            rep_y = Some(y);
                        }
                        "repX" => rep_x = Some(map.next_value()?),
                        "repY" => rep_y = Some(map.next_value()?),
                        "mat" => material = Some(map.next_value()?),
                        _ => return Err(Error::unknown_field(field, FIELDS))
                    }
                }

                let [mat_1, mat_2] = material.ok_or_else(|| Error::missing_field("mat"))?;
                let rep_x = rep_x.unwrap_or(1);
                let rep_y = rep_y.unwrap_or(1);

                Ok(GridMat::new(mat_1, mat_2, rep_x, rep_y))
            }
        }

        deserializer.deserialize_map(GridMatVisitor)
    }
}
