use amethyst::{
//    prelude::*,
    core::timing::Time,
    core::transform::Transform,
//    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{ReadExpect, System, Read, SystemData, WriteStorage, World, ReadStorage, Join, WriteExpect},
    shrev::{ReaderId, EventChannel},
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};
use crate::component::stt_block::StaticBlock;
use crate::system::stack_system::StackEvent;
use crate::world::{
    gravity_status::GravityStatus,
    stack_status::StackStatus,
    physics_queue::PhysicsQueue,
};

#[derive(SystemDesc)]
pub struct GravitySystem{
    pub time_delay: f32,
    //stop_gravity: bool,
    move_delay: f32,
    reader_id : ReaderId<StackEvent>,
}

impl GravitySystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<StackEvent>>().register_reader();
        Self { 
            time_delay : 0.0,
            move_delay : 1.0,
            //stop_gravity : false,
            reader_id 
        }
    }
}

impl<'s> System<'s> for GravitySystem{
    type SystemData = (
        Read<'s, Time>,
        Read<'s, EventChannel<StackEvent>>,
        ReadExpect<'s, GravityStatus>,
        ReadExpect<'s, StackStatus>,
        WriteExpect<'s, PhysicsQueue>,
    );

    fn run(&mut self, (time, event_channel, gravity_status, stack_status, mut queue): Self::SystemData){

        // If gravity status is off then igrnoe run
        if let GravityStatus::Off = *gravity_status {
            self.time_delay = 0.0;
            return;
        }

        // Increase time_delay count
        self.time_delay += time.delta_seconds();

        // if time ha reached then move downward
        if self.time_delay >= self.move_delay {
            queue.add_to_queue((0.0, -45.0));

            self.time_delay = 0.0;

            if self.move_delay >= 0.3 {
                self.move_delay -= 0.005;
            }
        }

    }
}
