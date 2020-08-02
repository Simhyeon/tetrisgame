use amethyst::{
//    prelude::*,
    core::math::Matrix4,
    core::timing::Time,
    core::transform::Transform,
//    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{ReadExpect, WriteExpect, System, ReadStorage, Join, Read, SystemData, WriteStorage},
    input::{InputHandler},
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler, Rotation};
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
const HEIGHT: f32 = 900.0;

impl<'s> System<'s> for KeyInputSystem {
    type SystemData = (
        WriteStorage<'s ,Transform>,
        ReadStorage<'s ,DynamicBlock>,
        ReadStorage<'s ,StaticBlock>,
        WriteExpect<'s, DynBlockHandler>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut locals ,blocks, stt, mut handler, input, time): Self::SystemData) {

        if handler.blocks.len() == 0 {
            return;
        }

        // Don't do logics if key interval has not passed yet.
        if let Some(mut timer) = self.key_interval.take(){
            timer -= time.delta_seconds();
            if timer <= 0.0 {
                self.key_interval = None;
            } else {
                self.key_interval.replace(timer);
            }
        } else {

            // Check if key input is possible
            // like, translation should not work if no enough spaces are given to blocks
            self.noinput = NoInput::None;
            for entity in &handler.blocks {
                if let Some(transform) = locals.get(*entity) {

                    // Cache entity's transform data
                    let local_value = transform.global_matrix().clone();

                    // If moving blocks are next to walls than cannot move toward walls
                    if KeyInputSystem::similar(local_value.m14, 0.0){
                        self.append_no_input(NoInput::Left);
                    } else if KeyInputSystem::similar(local_value.m14, WIDTH - 45.0 ){
                        self.append_no_input(NoInput::Right);
                    }

                    // If moving blocks are next to stacked blocks than cannot move toward stacked blocks
                    for (local, _block, _) in ( &mut locals, &blocks ,&stt).join(){
                        if KeyInputSystem::similar(local.global_matrix().m14, local_value.m14 + 45.0 )
                            && KeyInputSystem::similar(local.global_matrix().m24, local_value.m24) {
                            self.append_no_input(NoInput::Right);
                        } else if KeyInputSystem::similar(local.global_matrix().m14 , local_value.m14 - 45.0) 
                            && KeyInputSystem::similar(local.global_matrix().m24 , local_value.m24 ){
                            self.append_no_input(NoInput::Left);
                        }
                    }

                    // If input invalidation detected than break out
                    match self.noinput {
                        NoInput::None => (),
                        _ => break,
                    }

                } else {
                    return;
                }
            }

            // get axis value from key input
            let mut horizontal = input.axis_value(&AxisBinding::Horizontal).unwrap_or(0.0);
            let mut vertical = input.axis_value(&AxisBinding::Vertical).unwrap_or(0.0);

            // If input blockage detected then invalidate given axis value
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

            // Only get negative vertical value 
            // Player cannot move blocks upward.
            if vertical > 0.0 {
                vertical = 0.0;
            }

            // Now translate blocks according to user inputs for real.
            if let Some(parent) = handler.parent {
                if let Some(local) = locals.get_mut(parent) {
                    local.prepend_translation_x(45.0 * horizontal).prepend_translation_y(45.0 * vertical);
                }
            }

            // Get Actio inputs
            let shoot = input.action_is_down(&ActionBinding::Shoot).unwrap_or(false);
            let rotate_right = input.action_is_down(&ActionBinding::RotateRight).unwrap_or(false);

            // Currently for Debugging purpose
            // Print out useful location informations
            if shoot {
                println!("-------------------------");
                println!("Printing local transforms");
                for (local, _block, _) in ( &mut locals, &blocks ,&stt).join(){
                    println!("X : {}, Y : {}", local.global_matrix().m14, local.global_matrix().m24);
                }

                println!("Printing handler's blocks transforms");
                for entity in &handler.blocks {
                    println!("X : {}, Y : {}", locals.get(*entity).unwrap().global_matrix().m14, locals.get(*entity).unwrap().global_matrix().m24);
                }
                let (x, y) = handler.get_x_y_count(Rotation::Right);
                println!("X, Y value to Move is {}, {}", x, y);
                println!("-------------------------");
            }

            // If right rotate button was given
            if rotate_right {

                // Check Rotation validation prevent roation when not possible by meaning
                let rotate_offset = 3;
                let mut block_rotate = false;
                let (x, y) = handler.get_x_y_count(Rotation::Right);

                'outer: for entity in &handler.blocks{
                    let local_entity = locals.get(*entity).unwrap().global_matrix().clone();

                    for count in 1..rotate_offset+1 {

                        for (local, _block, _) in ( &mut locals, &blocks ,&stt).join(){
                            if local_entity.m14.round() + count as f32 * x * 45.0 == local.global_matrix().m14.round() 
                                && local_entity.m24.round() + count as f32 * y * 45.0 == local.global_matrix().m24.round(){
                                    block_rotate = true;
                                    break 'outer;
                                } 
                        }

                        if local_entity.m14.round() + count as f32 * x * 45.0 == -45.0 
                            || local_entity.m14.round() + count as f32 * x * 45.0 == WIDTH 
                                || local_entity.m24.round() + count as f32 * y * 45.0 == -45.0
                                || local_entity.m24.round() + count as f32 * y * 45.0 == HEIGHT + 45.0 {
                                    block_rotate = true;
                                    break 'outer;
                        }
                    }
                }

                //Rotate parent if not prevented from prior logics
                println!("Rotating");
                if !block_rotate {
                    locals.get_mut(handler.parent.unwrap()).unwrap().prepend_rotation_z_axis((PI * 0.5) as f32);
                    handler.rotate_handler(Rotation::Right);
                }
            }

            // Set Interval so that continous rotations or key inputs are kept from recognized.
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
