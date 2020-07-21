use amethyst::{
//    prelude::*,
    ecs::prelude::{Component, DenseVecStorage, Entity},
};

pub struct StaticBlock;

impl Component for StaticBlock {
    type Storage = DenseVecStorage<Self>;
}

