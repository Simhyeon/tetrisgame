use amethyst::core::math::Vector3;

use crate::config::Offset;

// Queue struct where physics realted orientation is saved
pub struct PhysicsQueue {
    queue : Option<Vec<Vector3<f32>>>,
    shoot : bool,
    offset: Option<(f32, f32)>,
    sub_offset: Option<(f32, f32)>,
}

impl PhysicsQueue {
    pub fn add(&mut self, physics : Vector3<f32>) {
        if let None = self.queue {
            self.queue = Some(Vec::new());
        }
        self.queue.unwrap().push(physics);
    }

    //TODO Not alwyas shoot method is called finally.
    // Shoot should be mutually exclusive
    pub fn shoot(&mut self) {
        self.queue = None;
        self.shoot = true;
        self.offset = None; 
        self.sub_offset = None; 
    }

    pub fn reset(&mut self) {
        self.queue = None;
        self.shoot = false;
        self.offset = None; 
        self.sub_offset = None; 
    }

    pub fn set_offset(&mut self, offset: (f32, f32)) {
        self.offset = Some(offset);
    }

    pub fn set_sub_offset(&mut self, offset: (f32, f32)) {
        self.sub_offset = Some(offset);
    }
}
