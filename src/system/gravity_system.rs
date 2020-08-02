use amethyst::{
//    prelude::*,
    core::timing::Time,
    core::transform::Transform,
//    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{ReadExpect, System, Read, SystemData, WriteStorage},
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};

const MOVEDELAY: f32 = 0.6;

#[derive(SystemDesc, Default)]
pub struct GravitySystem{
    pub time_delay: f32,
}

impl<'s> System<'s> for GravitySystem{
    type SystemData = (
        ReadExpect<'s, DynBlockHandler>,
        WriteStorage<'s, DynamicBlock>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (handler, _, mut locals, time): Self::SystemData){
        self.time_delay += time.delta_seconds();
        if self.time_delay >= MOVEDELAY {
            //println!("Delay : {}", self.time_delay);
            self.time_delay = 0.0;
            locals.get_mut(handler.parent.unwrap()).unwrap().prepend_translation_y(-45.0);
        }
    }
}
