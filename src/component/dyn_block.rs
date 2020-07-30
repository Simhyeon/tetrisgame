use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Entity},
};

pub struct DynamicBlock;

impl Component for DynamicBlock {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct DynBlockHandler {
    pub blocks: Vec<Entity>,
    pub parent: Option<Entity>,
}
