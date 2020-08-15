use amethyst::{
    core::math::Vector3,
    derive::SystemDesc,
    ecs::prelude::{System, SystemData, WriteExpect, ReadExpect},
};

use crate::world::{
    physics_queue::PhysicsQueue,
    input_cache::{InputCache, AxisType},
};
use crate::component::dyn_block::DynBlockHandler;
use crate::commons::Rotation;
use crate::consts::BLOCK_SIZE;

#[derive(SystemDesc, Default)]
pub struct PhysicsAllocator;

impl<'s> System<'s> for PhysicsAllocator {
    type SystemData = (
        ReadExpect<'s, DynBlockHandler>,
        ReadExpect<'s, InputCache>,
        WriteExpect<'s, PhysicsQueue>,
    );

    // Read from physisqueue and apply physics accordingly 
    fn run(&mut self, (handler, input_cache, mut queue): Self::SystemData) {
    }
}
