use amethyst::{
//    prelude::*,
    core::timing::Time,
    core::transform::Transform,
//    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{ReadExpect, System, Read, SystemData, WriteStorage, World},
    shrev::{ReaderId, EventChannel},
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};
use crate::system::stack_system::StackEvent;

const MOVEDELAY: f32 = 0.8;

#[derive(SystemDesc)]
pub struct GravitySystem{
    pub time_delay: f32,
    stop_gravity: bool,
    reader_id : ReaderId<StackEvent>,
}

impl GravitySystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<StackEvent>>().register_reader();
        Self { 
            time_delay : 0.0,
            stop_gravity : false,
            reader_id 
        }
    }
}

impl<'s> System<'s> for GravitySystem{
    type SystemData = (
        ReadExpect<'s, DynBlockHandler>,
        WriteStorage<'s, DynamicBlock>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, EventChannel<StackEvent>>,
    );

    fn run(&mut self, (handler, _, mut locals, time, event_channel): Self::SystemData){

        // Read all events
        for event in event_channel.read(&mut self.reader_id) {
            match event {
                StackEvent::ToBeStacked => {
                    println!("Stop Gravity");
                    self.stop_gravity = true;
                    self.time_delay = 0.0; // Also reset time dealy for continous ingegration? I guess
                    return;
                }
                StackEvent::Stacked | StackEvent::Free => {
                    println!("Use Gravity again");
                    self.stop_gravity = false;
                    break;
                }
                
                _ => ()
            }
        }

        if self.stop_gravity {
            return;
        }

        self.time_delay += time.delta_seconds();
        if self.time_delay >= MOVEDELAY {
            //println!("Delay : {}", self.time_delay);
            self.time_delay = 0.0;
            locals.get_mut(handler.parent.unwrap()).unwrap().prepend_translation_y(-45.0);
        }
    }
}
