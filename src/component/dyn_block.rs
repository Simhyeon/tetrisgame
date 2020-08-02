use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Entity},
};

pub struct DynamicBlock;

impl Component for DynamicBlock {
    type Storage = DenseVecStorage<Self>;
}

pub struct DynBlockHandler {
    pub blocks: Vec<Entity>,
    pub parent: Option<Entity>,
    pub rotation: Rotation,
}

impl Default for DynBlockHandler {
    fn default() -> Self {
        Self {
            blocks : vec![],
            parent : None,
            rotation : Rotation::Down,
        }
    }
}

// Utility Functions
impl DynBlockHandler {
    pub fn rotate_handler(&mut self, rotation: Rotation){
        if let Rotation::Right = rotation {
            match self.rotation {
                Rotation::Up => { self.rotation = Rotation::Right; },
                Rotation::Right => { self.rotation = Rotation::Down; },
                Rotation::Down => { self.rotation = Rotation::Left; },
                Rotation::Left => { self.rotation = Rotation::Up; },
            }
        } else if let Rotation::Left = rotation {
            match self.rotation {
                Rotation::Up => { self.rotation = Rotation::Left; },
                Rotation::Left => { self.rotation = Rotation::Down; },
                Rotation::Down => { self.rotation = Rotation::Right; },
                Rotation::Right => { self.rotation = Rotation::Up; },
            }
        }
    }

    pub fn get_x_y_count(&self, direction: Rotation) -> (f32, f32) {
        println!("Getting calcaultion");
        println!("Current rotation is : {:?}", self.rotation);
        if let Rotation::Right = direction {
            match self.rotation {
                Rotation::Up => (1.0, 0.0),
                Rotation::Right => (0.0, -1.0),
                Rotation::Down => (-1.0, 0.0),
                Rotation::Left => (0.0, 1.0),
            }
        } else if let Rotation::Left = direction {
            match self.rotation {
                Rotation::Up => (-1.0, 0.0),
                Rotation::Right => (0.0, 1.0),
                Rotation::Down => (1.0, 0.0),
                Rotation::Left => (0.0, -1.0),
            }
        } else {
            (0.0, 0.0)
        }
    }
}

#[derive(Debug)]
pub enum Rotation{
    Up,
    Right,
    Left,
    Down,
}
