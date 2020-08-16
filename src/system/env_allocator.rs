use amethyst::{
//    prelude::*,
    core::timing::Time,
    core::transform::Transform,
//    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{ReadExpect, System, Read, SystemData, WriteStorage, World, ReadStorage, Join, WriteExpect},
};
use crate::world::{
    input_cache::InputCache,
    physics_queue::PhysicsQueue,
    blockage::Blockage,
};
#[derive(SystemDesc, Default)]
pub struct EnvAllocator;

impl<'s> System<'s> for EnvAllocator {
    type SystemData = (
        WriteExpect<'s, InputCache>,
        WriteExpect<'s, PhysicsQueue>,
        WriteExpect<'s, Blockage>,
    );

    fn run(&mut self, (mut input_cache, mut queue, mut blockage): Self::SystemData) {
        input_cache.clear();
        queue.clear();
        blockage.clear();
    }
}
