pub mod sphere;
pub mod camera;
pub mod square;
pub mod plane;
pub mod light;

use crate::material::Material;
use crate::math::{
    point::Point,
    ray::Ray
};

use serde::{Deserialize, Deserializer, de::{Visitor, Error, MapAccess}};
use rulinalg::matrix::Matrix;

const GAP: f32 = 0.0005;

pub trait Movable {
    fn tra(&self) -> &Matrix<f32>;
    fn tra_mut(&mut self) -> &mut Matrix<f32>;

    fn inv(&self) -> &Matrix<f32>;
    fn inv_mut(&mut self) -> &mut Matrix<f32>;

    fn local_to_global_ray(&self, ray: &Ray) -> Ray {
        let origin = self.local_to_global_point(ray.origin());
        let vector = self.local_to_global_vector(ray.vector());
        Ray::new(origin, vector)
    }

    fn local_to_global_point(&self, pts: &Point) -> Point {
        let pts = pts.into_pt4();
        let pts = self.tra() * pts;
        pts.into_pt()
    }

    fn local_to_global_vector(&self, vec: &Point) -> Point {
        let vec = vec.into_vec4();
        let vec = self.tra() * vec;
        vec.into_vec()
    }

    fn global_to_local_ray(&self, ray: &Ray) -> Ray {
        let origin = self.global_to_local_point(ray.origin());
        let vector = self.global_to_local_vector(ray.vector());
        Ray::new(origin, vector)
    }

    fn global_to_local_point(&self, pts: &Point) -> Point {
        let pts = pts.into_pt4();
        let pts = self.inv() * pts;
        pts.into_pt()
    }

    fn global_to_local_vector(&self, vec: &Point) -> Point {
        let vec = vec.into_vec4();
        let vec = self.inv() * vec;
        vec.into_vec()
    }

    fn move_global(&mut self, x: f32, y: f32, z: f32) {
        let mat = Matrix::new(4, 4, vec![
            1., 0., 0., x,
            0., 1., 0., y,
            0., 0., 1., z,
            0., 0., 0., 1.
        ]);

        *self.tra_mut() = mat * self.tra();
        *self.inv_mut() = self.tra().clone().inverse().unwrap();
    }

    fn rotate_x(&mut self, angle: f32) {
        let angle = angle.to_radians();

        let mat = Matrix::new(4, 4, vec![
            1., 0., 0., 0.,
            0., angle.cos(), -angle.sin(), 0.,
            0., angle.sin(), angle.cos(), 0.,
            0., 0., 0., 1.
        ]);

        *self.tra_mut() = self.tra() * mat;
        *self.inv_mut() = self.tra().clone().inverse().unwrap();
    }

    fn rotate_y(&mut self, angle: f32) {
        let angle = angle.to_radians();

        let mat = Matrix::new(4, 4, vec![
            angle.cos(), 0., angle.sin(), 0.,
            0., 1., 0., 0.,
            -angle.sin(), 0., angle.cos(), 0.,
            0., 0., 0., 1.
        ]);

        *self.tra_mut() = self.tra() * mat;
        *self.inv_mut() = self.tra().clone().inverse().unwrap();
    }

    fn rotate_z(&mut self, angle: f32) {
        let angle = angle.to_radians();

        let mat = Matrix::new(4, 4, vec![
            angle.cos(), -angle.sin(), 0., 0.,
            angle.sin(), angle.cos(), 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.
        ]);

        *self.tra_mut() = self.tra() * mat;
        *self.inv_mut() = self.tra().clone().inverse().unwrap();
    }

    fn scale(&mut self, scale: f32) {
        let mat = Matrix::new(4, 4, vec![
            scale, 0., 0., 0.,
            0., scale, 0., 0.,
            0., 0., scale, 0.,
            0., 0., 0., 1.
        ]);

        *self.tra_mut() = self.tra() * mat;
        *self.inv_mut() = self.tra().clone().inverse().unwrap();
    }
}

pub trait Object: Movable {
    fn intersect(&self, ray: &Ray) -> Option<Point>;
    fn normal(&self, at: &Point, observer: &Point) -> Ray;
    fn material_at(&self, impact: &Point) -> Material;
    fn outter_normal(&self, impact: &Point) -> Point;
    fn coef_refraction(&self) -> f32;

    fn reflected_ray(&self, ray: &Ray, impact: &Point) -> Ray {
        let normal = self.normal(impact, ray.origin());

        let dot = ray.vector().dot(normal.vector());
        let reflected = ray.vector() - normal.vector() * 2.0 * dot;

        Ray::new(impact + reflected * GAP, reflected)
    }

    fn refracted_ray(&self, ray: &Ray, impact: &Point) -> Ray {
        let mut normal = self.outter_normal(impact);

        let mut cosi = ray.vector().dot(&normal);
        let eta = if cosi < 0.0 {
            cosi = -cosi;
            1.0 / self.coef_refraction()
        } else {
            normal = -normal;
            self.coef_refraction() / 1.0
        };

        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
        let refracted = if k < 0.0 {
            Point::default()
        } else {
            ray.vector() * eta + normal * (eta * cosi - k.sqrt())
        };

        Ray::new(impact + refracted * GAP, refracted)
    }
}

impl<'de> Deserialize<'de> for Box<dyn Object> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const FIELDS: &[&str] = &["type", "material", "refraction", "transform", "rotate", "scale"];
        const TYPES: &[&str] = &["SPHERE", "PLANE", "SQUARE"];
        struct ObjectVisitor;

        impl<'de> Visitor<'de> for ObjectVisitor {
            type Value = Box<dyn Object>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Object struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
                let mut obj_type = None;
                let mut material = None;
                let mut refraction = None;
                let mut transform = None;
                let mut rotate = None;
                let mut scale = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        "type" => obj_type = Some(map.next_value()?),
                        "material" => material = Some(map.next_value()?),
                        "refraction" => refraction = Some(map.next_value()?),
                        "transform" => transform = Some(map.next_value()?),
                        "rotate" => rotate = Some(map.next_value()?),
                        "scale" => scale = Some(map.next_value()?),
                        _ => return Err(Error::unknown_field(field, FIELDS)),
                    }
                }

                let obj_type = obj_type.ok_or_else(|| Error::missing_field("type"))?;
                let material = material.ok_or_else(|| Error::missing_field("material"))?;
                let coef_refraction = refraction.unwrap_or(1.0);

                let mut object: Box<dyn Object> = match obj_type {
                    "SPHERE" => Box::new(sphere::Sphere::new(material, coef_refraction)),
                    "PLANE" => Box::new(plane::Plane::new(material, coef_refraction)),
                    "SQUARE" => Box::new(square::Square::new(material, coef_refraction)),
                    _ => return Err(Error::unknown_variant(obj_type, TYPES)),
                };

                if let Some(Point {x, y , z}) = transform {
                    object.move_global(x, y, z);
                }

                if let Some(Point {x, y , z}) = rotate {
                    object.rotate_x(x);
                    object.rotate_y(y);
                    object.rotate_z(z);
                }

                if let Some(scale) = scale {
                    object.scale(scale);
                }

                Ok(object)
            }
        }

        deserializer.deserialize_map(ObjectVisitor)
    }
}
