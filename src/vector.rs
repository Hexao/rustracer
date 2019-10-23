#[derive(Default, Debug)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z:f32) -> Self {
        Self{x, y, z}
    }
}

impl std::ops::Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, rhs: &Vector) {
        *self = &(*self) + rhs;
    }
}
