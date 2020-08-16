use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Entity},
};
use crate::config::{Block, Offset};
use crate::utils;
use crate::commons::Rotation;

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
                _ => ()
            }
        } else if let Rotation::Left = rotation {
            match self.rotation {
                Rotation::Up => { self.rotation = Rotation::Left; },
                Rotation::Left => { self.rotation = Rotation::Down; },
                Rotation::Down => { self.rotation = Rotation::Right; },
                Rotation::Right => { self.rotation = Rotation::Up; },
                _ => ()
            }
        }
    }

    pub fn get_count(&self, direction: Rotation) -> (f32, f32) {
        if let Rotation::Right = direction {
            if let Some(offset) = self.config.offset {
                let (start, end) = offset.right_rotate;
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
                    _ => (0.0, 0.0)
                }
            } else {
                (0.0,0.0)
            }
        } else if let Rotation::Left = direction {
            if let Some(offset) = self.config.offset {
                let (start, end) = offset.left_rotate;
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
                    _ => (0.0, 0.0)
                }
            } else {
                (0.0, 0.0)
            }
        } else {
            (0.0, 0.0)
        }
    }

    // This is hard code as fuck so check this code asaf when rotation with suboffset problem
    // happens
    pub fn get_sub_count(&self, direction: Rotation) -> (f32, f32) {

        if let None = self.config.sub_offset {
            return (0.0,0.0)
        }

        if let Rotation::Right = direction {
            let (start, end) = self.config.sub_offset.unwrap().right_rotate;
            match self.rotation {
                Rotation::Up=> {
                    (start, end)
                }
                Rotation::Down => {
                    (start, end)
                }
                Rotation::Right => {
                    (start, end)
                }
                Rotation::Left => {
                    (-end, -start)
                }
                _ => (0.0, 0.0)
            }
        } else if let Rotation::Left = direction {
            let (start, end) = self.config.sub_offset.unwrap().left_rotate;
            match self.rotation {
                Rotation::Up => {
                    (start, end)
                }
                Rotation::Right => {
                    (start, end)
                }
                Rotation::Left=> {
                    (-end, -start)
                }
                Rotation::Down => {
                    (-end, -start)
                }
                _ => (0.0, 0.0)
            }
        } else {
            (0.0, 0.0)
        }
    }
}
