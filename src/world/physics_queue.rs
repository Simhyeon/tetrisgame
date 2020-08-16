use amethyst::core::math::Vector3;

use crate::config::Offset;
use crate::commons::Rotation;

// Queue struct where physics realted orientation is saved
#[derive(Default)]
pub struct PhysicsQueue {
    queue : Option<Vec<(f32, f32)>>,
    rotation: Option<Rotation>,
    shoot : bool,
    offset: Option<(f32, f32)>,
    sub_offset: Option<(f32, f32)>,
}

impl PhysicsQueue {

    pub fn get_queue(&mut self) -> Option<Vec<(f32, f32)>>{
        self.queue.clone()
    }

    pub fn add_to_queue(&mut self, physics : (f32,f32)) {
        if let None = self.queue {
            self.queue = Some(Vec::new());
        }
        self.queue.as_mut().unwrap().push(physics);
    }

    pub fn get_rotation(&self) -> Option<Rotation> {
        self.rotation.clone()
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.clear();
        self.rotation.replace(rotation);
    }

    pub fn get_shoot(&self) -> bool {
        self.shoot
    }

    pub fn set_shoot(&mut self, shoot: bool) {
        self.shoot = shoot;
    }

    pub fn shoot_check(&mut self) -> bool {
        if self.shoot {
            self.queue = None;
            self.offset = None; 
            self.sub_offset = None; 
            self.rotation = None;
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
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
