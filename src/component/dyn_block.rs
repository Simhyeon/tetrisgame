use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Entity},
};
use crate::config::{Block, Offset};
use crate::utils;

pub struct DynamicBlock;

impl Component for DynamicBlock {
    type Storage = DenseVecStorage<Self>;
}

pub struct DynBlockHandler {
    pub blocks: Vec<Entity>,
    pub config: Block,
    pub parent: Option<Entity>,
    pub rotation: Rotation,
}

impl Default for DynBlockHandler {
    fn default() -> Self {
        Self {
            blocks : vec![],
            config : Block::default(),
            parent : None,
            rotation : Rotation::Up,
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

    pub fn get_count(&self, direction: Rotation) -> (f32, f32) {
        if let Rotation::Right = direction {
            let (start, end) = self.config.offset.right_rotate;
            match self.rotation {
                Rotation::Up => {
                    (start, end)
                }
                Rotation::Right=> {
                    (-end, -start)
                }
                Rotation::Down => {
                    (-end, -start)
                }
                Rotation::Left => {
                    (start, end)
                }
            }
        } else if let Rotation::Left = direction {
            let (start, end) = self.config.offset.left_rotate;
            match self.rotation {
                Rotation::Up => {
                    (start, end)
                }
                Rotation::Right=> {
                    (-end, -start)
                }
                Rotation::Down => {
                    (-end, -start)
                }
                Rotation::Left => {
                    (start, end)
                }
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
