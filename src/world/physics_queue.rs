use amethyst::core::math::Vector3;

// Queue struct where physics realted orientation is saved
pub struct PhysicsQueue {
    queue : Vec<Vector3<f32>>,
    shoot : bool,
}

impl PhysicsQueue {
    pub fn add(&mut self, physics : Vector3<f32>) {
        self.queue.push(physics);
    }

    pub fn shoot(&mut self) {
        self.queue.clear();
        self.shoot = true;
    }

    pub fn reset(&mut self) {
        self.queue.clear();
        self.shoot = false;
    }
}

