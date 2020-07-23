use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Entity},
};

pub struct DynamicBlock;

impl Component for DynamicBlock {
    type Storage = DenseVecStorage<Self>;
}

pub struct DynBlockHandler {
    pub blocks: Vec<Entity>,
}
