use amethyst::core::math::Vector3;

use crate::config::Offset;
use crate::commons::Rotation;

// Queue struct where physics realted orientation is saved
pub struct PhysicsQueue {
    queue : Option<Vec<(f32, f32)>>,
    rotation: Option<Rotation>,
    shoot : bool,
    offset: Option<(f32, f32)>,
    sub_offset: Option<(f32, f32)>,
}

impl PhysicsQueue {

    pub fn get_queue(&self) -> Option<Vec<(f32, f32)>>{
        self.queue
    }

    pub fn add_to_queue(&mut self, physics : (f32,f32)) {
        if let None = self.queue {
            self.queue = Some(Vec::new());
        }
        self.queue.as_mut().unwrap().push(physics);
    }

    pub fn get_rotation(&self) -> Option<Rotation> {
        if let Some(rotation) = self.rotation {
            Some(rotation)
        } else {
            None
        }
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.reset();
        self.rotation.replace(rotation);
    }

    pub fn get_shoot(&self) -> bool {
        self.shoot
    }

    pub fn shoot_check(&mut self) {
        if self.shoot {
            self.queue = None;
            self.offset = None; 
            self.sub_offset = None; 
            self.rotation = None;
        }
    }

    pub fn reset(&mut self) {
        self.queue = None;
        self.shoot = false;
        self.offset = None; 
        self.sub_offset = None; 
        self.rotation = None;
    }

    pub fn get_offset(&self) -> (f32, f32) {
        if let Some(value) = self.offset {
            value
        } else {
            (0.0, 0.0)
        }
    }

    pub fn set_offset(&mut self, offset: (f32, f32)) {
        self.offset = Some(offset);
    }

    pub fn get_sub_offset(&self) -> (f32, f32) {
        if let Some(value) = self.sub_offset {
            value
        } else {
            (0.0, 0.0)
        }
    }

    pub fn set_sub_offset(&mut self, offset: (f32, f32)) {
        self.sub_offset = Some(offset);
    }
}
