use amethyst::{
    core::math::Vector3,
    core::transform::{Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadStorage, WriteExpect, Write, Read, World, LazyUpdate, ReadExpect},
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
        match input_cache.axis {
            AxisType::Right => {
                queue.add(Vector3::new(BLOCK_SIZE, 0.0, 0.0));
            }
            AxisType::Left => {
                queue.add(Vector3::new(-BLOCK_SIZE, 0.0, 0.0));
            }
            AxisType::Down => {
                queue.add(Vector3::new(0.0, -BLOCK_SIZE, 0.0));
            }
            AxisType::None => {
                ()
            }
        }

        match input_cache.rotation {
            Rotation::Right | Rotation::Left => {
                queue.set_offset(handler.get_count(input_cache.rotation));
                queue.set_sub_offset(handler.get_sub_count(input_cache.rotation));
            }
            _ => ()
        }
    }
}
