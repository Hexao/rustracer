use crate::object::Object;

pub struct Scene {
    objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::default(),
        }
    }
}
