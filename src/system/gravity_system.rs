use amethyst::{
//    prelude::*,
    core::timing::Time,
    core::transform::Transform,
//    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{ReadExpect, System, Read, SystemData, WriteStorage, World, ReadStorage, Join},
    shrev::{ReaderId, EventChannel},
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};
use crate::component::stt_block::StaticBlock;
use crate::system::stack_system::StackEvent;
use crate::world::{
    gravity_status::GravityStatus,
    stack_status::StackStatus,
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
        ReadExpect<'s, DynBlockHandler>,
        ReadStorage<'s, DynamicBlock>,
        ReadStorage<'s, StaticBlock>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, EventChannel<StackEvent>>,
        ReadExpect<'s, GravityStatus>,
        ReadExpect<'s, StackStatus>,
    );

    fn run(&mut self, (handler, blocks,stt, mut locals, time, event_channel, gravity_status, stack_status): Self::SystemData){

        // If gravity status is off then igrnoe run
        if let GravityStatus::Off = *gravity_status {
            self.time_delay = 0.0;
            return;
        }

        if let None = handler.parent {
            return;
        }

        if let StackStatus::None = *stack_status {

            // Prevent block duplication in any consequences
            for entity in handler.blocks.iter() {
                let x_pos = locals.get(*entity).unwrap().global_matrix().m14.round();
                let y_pos = locals.get(*entity).unwrap().global_matrix().m24.round();
                if y_pos == 45.0 {
                    return;
                }

                for (local, _block, _) in ( &mut locals, &blocks ,&stt).join(){
                    if y_pos == local.global_matrix().m24.round() + 45.0
                        && x_pos == local.global_matrix().m14.round(){
                            return;
                    } 
                }
            }

            // Increase time_delay count
            self.time_delay += time.delta_seconds();

            // if time ha reached then move downward
            if self.time_delay >= self.move_delay {
                //println!("Delay : {}", self.time_delay);
                self.time_delay = 0.0;
                locals.get_mut(handler.parent.unwrap()).unwrap().prepend_translation_y(-45.0);
                if self.move_delay >= 0.3 {
                    self.move_delay -= 0.005;
                }
            }
        }

    }
}
