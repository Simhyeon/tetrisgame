use amethyst::{
    //prelude::*,
    ecs::prelude::{Component, DenseVecStorage},
};

pub struct StaticBlock;

impl Component for StaticBlock {
    type Storage = DenseVecStorage<Self>;
}
