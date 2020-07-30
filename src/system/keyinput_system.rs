use amethyst::{
//    prelude::*,
    core::timing::Time,
    core::transform::Transform,
//    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{ReadExpect, System, ReadStorage, Join, Read, SystemData, WriteStorage},
    input::{InputHandler},
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};
use crate::component::stt_block::StaticBlock;
use crate::config::{MovementBindingTypes, AxisBinding, ActionBinding};
use std::f64::consts::PI;

const INPUTINTERVAL: f32 = 0.05;
const EPSILON: f32 = 0.0001;

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
    Both,
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

        if handler.blocks.len() == 0 {
            return;
        }

        if let Some(mut timer) = self.key_interval.take(){
            timer -= time.delta_seconds();
            if timer <= 0.0 {
                self.key_interval = None;
            } else {
                self.key_interval.replace(timer);
            }
        } else {
            self.noinput = NoInput::None;
            for entity in &handler.blocks {
                if let Some(transform) = locals.get(*entity) {
                    let local_value = transform.global_matrix().clone();
                    if KeyInputSystem::similar(local_value.m14, 0.0){
                        self.append_no_input(NoInput::Left);
                    } else if KeyInputSystem::similar(local_value.m14, WIDTH - 45.0 ){
                        self.append_no_input(NoInput::Right);
                    }

                    for (local, _block, _) in ( &mut locals, &blocks ,&stt).join(){
                        if KeyInputSystem::similar(local.global_matrix().m14, local_value.m14 + 45.0 )
                            && KeyInputSystem::similar(local.global_matrix().m24, local_value.m24) {
                            self.append_no_input(NoInput::Right);
                        } else if KeyInputSystem::similar(local.global_matrix().m14 , local_value.m14 - 45.0) 
                            && KeyInputSystem::similar(local.global_matrix().m24 , local_value.m24 ){
                            self.append_no_input(NoInput::Left);
                        }
                    }

                    match self.noinput {
                        NoInput::None => (),
                        _ => break,
                    }

                } else {
                    return;
                }
            }

            let mut horizontal = input.axis_value(&AxisBinding::Horizontal).unwrap_or(0.0);
            let mut vertical = input.axis_value(&AxisBinding::Vertical).unwrap_or(0.0);

            match self.noinput {
                NoInput::Left => {
                    if horizontal < 0.0 { horizontal = 0.0; }
                },
                NoInput::Right =>{
                    if horizontal > 0.0 { horizontal = 0.0; }
                },
                NoInput::Both =>{
                    horizontal = 0.0;
                },
                _ => (),
            }

            if vertical > 0.0 {
                vertical = 0.0;
            }

            if let Some(parent) = handler.parent {
                if let Some(local) = locals.get_mut(parent) {
                    local.prepend_translation_x(45.0 * horizontal).prepend_translation_y(45.0 * vertical);
                }
            }

            let shoot = input.action_is_down(&ActionBinding::Shoot).unwrap_or(false);
            let rotate_right = input.action_is_down(&ActionBinding::RotateRight).unwrap_or(false);
            if shoot {
                for (local, _block, ()) in ( &mut locals, &blocks ,!&stt).join(){
                    println!("{}", local.global_matrix());
                }
            }
            if rotate_right {
                //println!("Rotating");
                //locals.get_mut(handler.parent.unwrap()).unwrap().set_rotation_z_axis((PI * 0.5) as f32);
                locals.get_mut(handler.parent.unwrap()).unwrap().append_rotation_z_axis((PI * 0.5) as f32);
                //for (local, _block, ()) in ( &mut locals, &blocks ,!&stt).join(){
                    //println!("{}", local.global_matrix());
                    //println!("{}", local.global_matrix().m14);
                //}
            }

            self.key_interval.replace(INPUTINTERVAL);
        }

    }
}

impl KeyInputSystem {
    fn similar(value1: f32, value2: f32) -> bool{
        if (value1 - value2).abs() <= EPSILON {
            true
        } else {
            false
        }
    }

    fn append_no_input(&mut self, no_input: NoInput) {
        match self.noinput {
            NoInput::Left => {
                if let NoInput::Right = no_input {
                    self.noinput = NoInput::Both;
                }
            },

            NoInput::Right => {
                if let NoInput::Left = no_input {
                    self.noinput = NoInput::Both;
                }
            },

            NoInput::None => {
                self.noinput = no_input;
            },

            _ => (),
        }
    }
}
