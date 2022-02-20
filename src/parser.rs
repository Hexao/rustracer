use crate::object::camera::{Camera, Focal};
use crate::material::Color;
use crate::scene::Scene;

use serde::{Deserialize, Deserializer, de::{Visitor, Error, MapAccess}};

pub fn parse_file(file_name: &str) -> (Scene, Camera, Config) {
    let content = std::fs::read_to_string(file_name).unwrap();
    let Parser { scene, camera, config } = serde_json::from_str(content.as_str()).unwrap();

    (scene, camera, config)
}

pub struct Config {
    pub output: String,
    pub threads: usize,
    pub depth: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output: "output.png".to_owned(),
            threads: 1,
            depth: 0,
        }
    }
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const FIELDS: &[&str] = &["output", "threads", "depth"];
        struct ConfigVisitor;

        impl<'de> Visitor<'de> for ConfigVisitor {
            type Value = Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Config struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
                let mut output = "output.png".to_owned();
                let mut threads = 1;
                let mut depth = 0;

                while let Some(field) = map.next_key()? {
                    match field {
                        "threads" => threads = map.next_value()?,
                        "output" => output = map.next_value()?,
                        "depth" => depth = map.next_value()?,
                        _ => return Err(Error::unknown_field(field, FIELDS)),
                    }
                }

                Ok(Self::Value { output, threads, depth })
            }
        }

        deserializer.deserialize_map(ConfigVisitor)
    }
}

struct Parser {
    scene: Scene,
    camera: Camera,
    config: Config,
}

impl<'de> Deserialize<'de> for Parser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const FIELDS: &[&str] = &["scene", "objects", "lights", "camera", "config"];
        struct ParserVisitor;

        impl<'de> Visitor<'de> for ParserVisitor {
            type Value = Parser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("JSON map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
                struct SceneColor(Color, Color);

                impl<'de> Deserialize<'de> for SceneColor {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
                        const FIELDS: &[&str] = &["background", "ambient"];
                        struct SceneColorVisitor;

                        impl<'de> Visitor<'de> for SceneColorVisitor {
                            type Value = SceneColor;

                            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                                formatter.write_str("Scene color struct")
                            }

                            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
                                let mut backgroung = None;
                                let mut ambient = None;

                                while let Some(field) = map.next_key()? {
                                    match field {
                                        "background" => backgroung = Some(map.next_value()?),
                                        "ambient" => ambient = Some(map.next_value()?),
                                        _ => return Err(Error::unknown_field(field, FIELDS)),
                                    }
                                }

                                let backbround = backgroung.unwrap_or(Color::SKY);
                                let ambient = ambient.unwrap_or_else(|| Color::new_gray(120));
                                Ok(SceneColor(backbround, ambient))
                            }
                        }

                        deserializer.deserialize_map(SceneColorVisitor)
                    }
                }

                let mut objects = None;
                let mut lights = None;
                let mut colors = None;
                let mut camera = None;
                let mut config = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        "scene" => colors = Some(map.next_value()?),
                        "objects" => objects = Some(map.next_value()?),
                        "lights" => lights = Some(map.next_value()?),
                        "camera" => camera = Some(map.next_value()?),
                        "config" => config = Some(map.next_value()?),
                        _ => return Err(Error::unknown_field(field, FIELDS)),
                    }
                }

                let objects = objects.unwrap_or_default();
                let lights = lights.unwrap_or_default();
                let SceneColor(background, ambient) = colors.unwrap_or_else(
                    || SceneColor(Color::SKY, Color::new_gray(120))
                );

                let scene = Scene::new(objects, lights, background, ambient);
                let camera = camera.unwrap_or_else(|| Camera::new(1920, 1080, Focal::Perspective(1.7)));
                let config = config.unwrap_or_default();

                Ok(Self::Value { scene, camera, config })
            }
        }

        deserializer.deserialize_map(ParserVisitor)
    }
}
