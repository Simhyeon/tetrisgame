use amethyst::{
//    prelude::*,
    core::timing::Time,
    core::transform::Transform,
//    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{ReadExpect, WriteExpect, System, ReadStorage, Join, Read, SystemData, WriteStorage, World},
    input::{InputHandler},
    shrev::{ReaderId, EventChannel},
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler, Rotation};
use crate::component::stt_block::StaticBlock;
use crate::config::{MovementBindingTypes, AxisBinding, ActionBinding};
use crate::system::stack_system::StackEvent;
use std::f64::consts::PI;
use std::cmp::Ordering;

const INPUTDELAY : f32 = 0.07;
const EPSILON: f32 = 0.0001;

#[derive(SystemDesc)]
pub struct KeyInputSystem {
    pub key_interval: Option<f32>,
    noinput: NoInput,
    key_status: KeyStatus,
    axis_delay: f32,
}

// If same key press was given then set that input as hold
enum KeyPressType {
    Click,
    Hold,
    None,
}

impl KeyInputSystem {
    pub fn new() -> Self {
        Self { 
            key_interval: None,
            noinput: NoInput::None,
            key_status : KeyStatus::default(),
            axis_delay : INPUTDELAY,
        }
    }

    fn update_key_status(&mut self, horizontal_value: f32, vertical_value: f32, right_value: bool, left_value: bool) {
        self.key_status.update_hor(horizontal_value);
        self.key_status.update_ver(vertical_value);
        // This is because right_value and left_value is given true when clicked
        // While set_to_none parameter set KepressType to none when given true
        // So to update key_status you should set negation of rotation value
        self.key_status.update_right(!right_value);
        self.key_status.update_left(!left_value);
    }

    fn delay_hold_input(&mut self, horizontal_value: &mut f32, vertical_value: &mut f32, right_value: &mut bool, left_value: &mut bool, dtime: f32) {

        // Disblae for axis input
        if let KeyPressType::Hold = self.key_status.horizontal {
            if self.axis_delay >= 0.0 {
                *horizontal_value = 0.0;
                self.axis_delay -= dtime;
            } else {
                self.axis_delay = INPUTDELAY;
            }
        }
        if let KeyPressType::Hold = self.key_status.vertical {
            if self.axis_delay >= 0.0 {
                *vertical_value = 0.0;
                self.axis_delay -= dtime;
            } else {
                self.axis_delay = INPUTDELAY;
            }
        }

        if let KeyPressType::Hold = self.key_status.right_rotate {
            *right_value = false;
        }
        if let KeyPressType::Hold = self.key_status.left_rotate {
            *left_value = false;
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

        // get input value from key input
        let mut horizontal = input.axis_value(&AxisBinding::Horizontal).unwrap_or(0.0);
        let mut vertical = input.axis_value(&AxisBinding::Vertical).unwrap_or(0.0);
        let mut rotate_right = input.action_is_down(&ActionBinding::RotateRight).unwrap_or(false);
        let mut rotate_left = input.action_is_down(&ActionBinding::RotateLeft).unwrap_or(false);
        let shoot = input.action_is_down(&ActionBinding::Shoot).unwrap_or(false);

        // Up
        self.update_key_status(horizontal, vertical, rotate_right, rotate_left);
        self.delay_hold_input(&mut horizontal, &mut vertical, &mut rotate_right, &mut rotate_left, time.delta_seconds());

        if horizontal != 0.0 {
            vertical = 0.0;
        }

        // Only get negative vertical value 
        // Player cannot move blocks upward.
        if vertical > 0.0 {
            vertical = 0.0;
        }

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



        // If vertcial input is not possible then set value to 0.0
        if vertical < 0.0 {
            for entity in handler.blocks.iter() {

                let x_pos = locals.get(*entity).unwrap().global_matrix().m14.round();
                let y_pos = locals.get(*entity).unwrap().global_matrix().m24.round();
                if y_pos == 45.0 {
                    vertical = 0.0;
                    break;
                }

                for (local, _block, _) in ( &mut locals, &blocks ,&stt).join(){
                    if y_pos == local.global_matrix().m24.round() + 45.0
                        && x_pos == local.global_matrix().m14.round(){
                            vertical = 0.0;
                            break;
                    } 
                }
            }
        }

        // Now translate blocks according to user inputs for real.
        if let Some(parent) = handler.parent {
            if let Some(local) = locals.get_mut(parent) {
                local.prepend_translation_x(45.0 * horizontal).prepend_translation_y(45.0 * vertical);
            }
        }

        // Currently for Debugging purpose
        // Print out useful location informations
        if shoot {

            //println!("Printing handler's blocks transforms");
            println!("Printing DEBUGGING Informations ...");
            println!("Current rotation is :{:?}", handler.rotation);
            for entity in &handler.blocks {
                println!("X : {}, Y : {}", locals.get(*entity).unwrap().global_matrix().m14, locals.get(*entity).unwrap().global_matrix().m24);
            }
        }

        // If rotate button was given
        if rotate_right || rotate_left {

            let mut block_rotate = false;
            let start: f32;
            let end: f32;

            if rotate_right {
                let (s, e) = handler.get_count(Rotation::Right);
                start = s;
                end = e;
            } else { // if rotate left
                let (s, e) = handler.get_count(Rotation::Left);
                start = s;
                end = e;
            }

            // Check Rotation validation prevent roation when not possible
            // Get offset
            let x: f32;
            let y: f32;

            match handler.rotation {
                Rotation::Up | Rotation::Down => {
                    x = 1.0;
                    y = 0.0;
            }
                Rotation::Right | Rotation::Left => {
                    x = 0.0;
                    y = 1.0;
            }
            }

            // Loop through transforms
            let parent = locals.get(handler.parent.unwrap()).unwrap().global_matrix().clone();
            for count in start as i32 .. end as i32 + 1 {
                for (local, _block, _) in ( &mut locals, &blocks ,&stt).join(){
                    if parent.m14.round() + count as f32 * x * 45.0 == local.global_matrix().m14.round() 
                        && parent.m24.round() + count as f32 * y * 45.0 == local.global_matrix().m24.round(){
                            block_rotate = true;
                            break;
                    } 
                }

                if parent.m14.round() + count as f32 * x * 45.0 == -45.0 
                    || parent.m14.round() + count as f32 * x * 45.0 == WIDTH 
                        || parent.m24.round() + count as f32 * y * 45.0 == 0.0
                        || parent.m24.round() + count as f32 * y * 45.0 == HEIGHT + 45.0 {
                            block_rotate = true;
                            break;
                }
            }

            if let Some(_) = handler.config.sub_offset {
                // Reuse variables names because the variables are not gonna used again.
                let start: f32;
                let end: f32;

                if rotate_right {
                    let (s, e) = handler.get_sub_count(Rotation::Right);
                    start = s;
                    end = e;
                } else { // if rotate left
                    let (s, e) = handler.get_sub_count(Rotation::Left);
                    start = s;
                    end = e;
                }
                let x: f32;
                let y: f32;

                // This is exactly reverser that of normal counting
                match handler.rotation {
                    Rotation::Up | Rotation::Down => {
                        x = 0.0;
                        y = 1.0;
                }
                    Rotation::Right | Rotation::Left => {
                        x = 1.0;
                        y = 0.0;
                }
                }

                // Loop through transforms
                let parent = locals.get(handler.parent.unwrap()).unwrap().global_matrix().clone();
                for count in start as i32 .. end as i32 + 1 {
                    for (local, _block, _) in ( &mut locals, &blocks ,&stt).join(){
                        if parent.m14.round() + count as f32 * x * 45.0 == local.global_matrix().m14.round() 
                            && parent.m24.round() + count as f32 * y * 45.0 == local.global_matrix().m24.round(){
                                block_rotate = true;
                                break;
                        } 
                    }
                    if parent.m14.round() + count as f32 * x * 45.0 == -45.0 
                        || parent.m14.round() + count as f32 * x * 45.0 == WIDTH 
                            || parent.m24.round() + count as f32 * y * 45.0 == 0.0
                            || parent.m24.round() + count as f32 * y * 45.0 == HEIGHT + 45.0 {
                                block_rotate = true;
                                break;
                    }
                }
            }

            //Rotate parent if not prevented from prior logics
            if !block_rotate {
                println!("--Executing Rotation--");
                if rotate_right {
                    handler.rotate_handler(Rotation::Right);
                    locals.get_mut(handler.parent.unwrap()).unwrap().prepend_rotation_z_axis((PI * 0.5) as f32);
                } else {
                    handler.rotate_handler(Rotation::Left);
                    locals.get_mut(handler.parent.unwrap()).unwrap().prepend_rotation_z_axis(-(PI * 0.5) as f32);
                }
            } else {
                println!("--Blocked Rotation--");
            }
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

struct KeyStatus {
    pub horizontal: KeyPressType,
    pub vertical: KeyPressType,
    pub right_rotate: KeyPressType,
    pub left_rotate: KeyPressType,
    //shoot: KeyPressType,
}

impl Default for KeyStatus {
    fn default() -> Self {
        Self {
            horizontal: KeyPressType::None,
            vertical: KeyPressType::None,
            right_rotate: KeyPressType::None,
            left_rotate: KeyPressType::None,
        }
    }

}

impl KeyStatus {

    fn update_hor(&mut self, set_to_none: f32) {

        if let Ordering::Equal = set_to_none.partial_cmp(&0.0).unwrap() {
            self.horizontal = KeyPressType::None;
            return;
        }

        match self.horizontal {
            KeyPressType::None => self.horizontal = KeyPressType::Click,
            KeyPressType::Click => self.horizontal = KeyPressType::Hold,
            _ => ()
        }
    }

    fn update_ver(&mut self, set_to_none: f32) {

        if let Ordering::Equal = set_to_none.partial_cmp(&0.0).unwrap() {
            self.vertical = KeyPressType::None;
            return;
        }

        match self.vertical {
            KeyPressType::None => self.vertical = KeyPressType::Click,
            KeyPressType::Click => self.vertical = KeyPressType::Hold,
            _ => ()
        }
    }
    
    fn update_right(&mut self, set_to_none: bool) {

        if set_to_none {
            self.right_rotate = KeyPressType::None;
            return;
        }

        match self.right_rotate {
            KeyPressType::None => self.right_rotate = KeyPressType::Click,
            KeyPressType::Click => self.right_rotate = KeyPressType::Hold,
            _ => ()
        }
    }

    fn update_left(&mut self, set_to_none: bool) {

        if set_to_none {
            self.left_rotate = KeyPressType::None;
            return;
        }

        match self.left_rotate {
            KeyPressType::None => self.left_rotate = KeyPressType::Click,
            KeyPressType::Click => self.left_rotate = KeyPressType::Hold,
            _ => ()
        }
    }
}
