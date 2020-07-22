use amethyst::{
    prelude::*,
    ecs::prelude::{Component, DenseVecStorage, Entity},
};

pub struct StaticBLock;

impl Component for StaticBLock {
    type Storage = DenseVecStorage<Self>;
}
