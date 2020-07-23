use amethyst::{
//    prelude::*,
    core::math::Translation3,
    core::timing::Time,
    core::transform::Transform,
//    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{ReadExpect, Entity, World, System, ReadStorage, Write, Join, Read, SystemData, WriteStorage},
    input::{InputHandler},
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};
use crate::component::stt_block::StaticBlock;
use crate::config::{MovementBindingTypes, AxisBinding};

const INPUTINTERVAL: f32 = 0.05;

#[derive(SystemDesc)]
pub struct KeyInputSystem {
    pub key_interval: Option<f32>,
    noinput: NoInput,
}

impl Default for KeyInputSystem {
    fn default() -> KeyInputSystem {
        KeyInputSystem {
            key_interval: None,
            noinput: NoInput::None,
        }
    }
}

enum NoInput{
    Right,
    Left,
    None,
}

const WIDTH: f32 = 450.0;

impl<'s> System<'s> for KeyInputSystem {
    type SystemData = (
        WriteStorage<'s ,Transform>,
        ReadStorage<'s ,DynamicBlock>,
        ReadStorage<'s ,StaticBlock>,
        ReadExpect<'s, DynBlockHandler>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut locals ,blocks, stt, handler, input, time): Self::SystemData) {
        if let Some(mut timer) = self.key_interval.take(){
            timer -= time.delta_seconds();
            if timer <= 0.0 {
                self.key_interval = None;
            } else {
                self.key_interval.replace(timer);
            }
        } else {
            // run 마다 clone 한다는 건데 이건 상당히 비효율적이긴 한다. 
            for entity in handler.blocks.clone(){
                let local_value = locals.get(entity).unwrap().translation().x;
                if local_value == 0.0{
                    self.noinput = NoInput::Left;
                } else if local_value == WIDTH - 45.0 {
                    self.noinput = NoInput::Right;
                } else {
                    self.noinput = NoInput::None;
                }
            }
            for (local, _block, ()) in ( &mut locals, &blocks ,!&stt).join(){
                let horizontal = input.axis_value(&AxisBinding::Horizontal).unwrap_or(0.0);
                let vertical = input.axis_value(&AxisBinding::Vertical).unwrap_or(0.0);
                match self.noinput {
                    NoInput::Right => {
                        if horizontal < 0.0 {
                            local.prepend_translation_x(45.0 * horizontal); // Hard Coded for now
                        }
                    }
                    NoInput::Left => {
                        if horizontal > 0.0 {
                            local.prepend_translation_x(45.0 * horizontal); // Hard Coded for now
                        }
                    }
                    NoInput::None => {
                        local.prepend_translation_x(45.0 * horizontal); // Hard Coded for now
                    }
                }

                if vertical < 0.0 {
                    local.prepend_translation_y(45.0 * vertical); // Hard Coded for now
                }
            }
            self.key_interval.replace(INPUTINTERVAL);
        }

    }
}
