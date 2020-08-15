use amethyst::{
    core::math::Vector3,
    core::transform::{Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadStorage, WriteExpect, Write, Read, World, LazyUpdate, ReadExpect},
};

use crate::world::physics_queue::PhysicsQueue;

#[derive(SystemDesc, Default)]
pub struct PhysicsExecutor;

impl<'s> System<'s> for PhysicsExecutor {
    type SystemData = (
        ReadExpect<'s, PhysicsQueue>,
    );


    // Read from physisqueue and apply physics accordingly 
    fn run(&mut self, _data: Self::SystemData) {
        true;
    }
}
