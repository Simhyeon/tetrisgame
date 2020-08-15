use amethyst::{
//    prelude::*,
    core::timing::Time,
    core::transform::{Transform, Parent},
//    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{WriteExpect, System, ReadStorage, Join, Read, SystemData, WriteStorage, World, Write, ReadExpect, Entities},
    input::InputHandler,
    shrev::{ReaderId, EventChannel},
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};
use crate::component::stt_block::StaticBlock;
use crate::config::{MovementBindingTypes, AxisBinding, ActionBinding};
use crate::world::{
    block_data::BlockData,
    gravity_status::GravityStatus,
    key_int::KeyInt,
    stack_status::StackStatus,
};
use crate::commons::Rotation;
use std::f64::consts::PI;
use std::cmp::Ordering;

const HOR_DELAY : f32 = 0.15;
const VER_DELAY : f32 = 0.07;
const EPSILON: f32 = 0.0001;

#[derive(SystemDesc)]
pub struct KeyInputSystem {
    pub key_interval: Option<f32>,
    key_status: KeyStatus,
    axis_delay: (f32,f32),
    reader_id : ReaderId<KeyInt>,
}

// If same key press was given then set that input as hold
enum KeyPressType {
    Click,
    Hold,
    None,
}

impl KeyInputSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<KeyInt>>().register_reader();
        Self { 
            key_interval: None,
            key_status : KeyStatus::default(),
            axis_delay : (HOR_DELAY, VER_DELAY),
            reader_id,
        }
    }

    fn update_key_status(&mut self, horizontal_value: f32, vertical_value: f32, right_value: bool, left_value: bool, shoot_value: bool) {
        self.key_status.update_hor(horizontal_value);
        self.key_status.update_ver(vertical_value);
        // This is because right_value and left_value is given true when clicked
        // While set_to_none parameter set KepressType to none when given true
        // So to update key_status you should set negation of rotation value
        self.key_status.update_right(!right_value);
        self.key_status.update_left(!left_value);
        self.key_status.update_shoot(!shoot_value);
    }

    fn delay_hold_input(&mut self, horizontal_value: &mut f32, vertical_value: &mut f32, right_value: &mut bool, left_value: &mut bool, shoot_value: &mut bool, dtime: f32) {

        //// Disaable axis input operation while holding
        //if let KeyPressType::Hold = self.key_status.horizontal {
            //*horizontal_value = 0.0;
        //}
        //if let KeyPressType::Hold = self.key_status.vertical {
            //*vertical_value = 0.0;
        //}

        // Slow axis input operation while holding
        if let KeyPressType::Hold = self.key_status.horizontal {
            if self.axis_delay.0 >= 0.0 {
                *horizontal_value = 0.0;
                self.axis_delay.0 -= dtime;
            } else {
                self.axis_delay.0 = HOR_DELAY;
            }
        }
        if let KeyPressType::Hold = self.key_status.vertical {
            if self.axis_delay.1 >= 0.0 {
                *vertical_value = 0.0;
                self.axis_delay.1 -= dtime;
            } else {
                self.axis_delay.1 = VER_DELAY;
            }
        }

        // Disable for action input while holding
        if let KeyPressType::Hold = self.key_status.right_rotate {
            *right_value = false;
        }
        if let KeyPressType::Hold = self.key_status.left_rotate {
            *left_value = false;
        }

        if let KeyPressType::Hold = self.key_status.shoot {
            *shoot_value = false;
        }
    }

    fn mutual_exclude(&mut self, horizontal_value: &mut f32, vertical_value: &mut f32, right_value: &mut bool, left_value: &mut bool, shoot_value: &mut bool) {
        if *horizontal_value != 0.0 {
            *vertical_value = 0.0;
            *right_value = false;
            *left_value = false;
            *shoot_value = false;
        } else if *vertical_value != 0.0{
            *horizontal_value = 0.0;
            *right_value = false;
            *left_value = false;
            *shoot_value = false;
        } else if *right_value {
            *vertical_value = 0.0;
            *horizontal_value = 0.0;
            *left_value = false;
            *shoot_value = false;
        } else if *left_value {
            *vertical_value = 0.0;
            *horizontal_value = 0.0;
            *right_value = false;
            *shoot_value = false;
        } else if *shoot_value {
            *vertical_value = 0.0;
            *horizontal_value = 0.0;
            *left_value = false;
            *right_value = false;
        }
    }
}

impl<'s> System<'s> for KeyInputSystem {
    type SystemData = (
        //DEBUG
        Entities<'s>,
        ReadStorage<'s, Parent>,
        //DEBUGEND
        WriteExpect<'s, DynBlockHandler>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, Time>,
        Write<'s, EventChannel<KeyInt>>,
        WriteExpect<'s, StackStatus>,
        WriteExpect<'s, GravityStatus>,
        WriteExpect<'s, KeyInt>,
    );

    fn run(&mut self, (
            mut entities, 
            parents, 
            mut handler, 
            input, 
            time, 
            mut key_event_channel, 
            mut stack_status, 
            mut gravity_status,
            mut key_int
    ): Self::SystemData) {
        if handler.blocks.len() == 0{
            return;
        }

        if let None = handler.parent {
            return;
        }

        match *key_int {
            KeyInt::Stack => {
                return;
            },
            _ => ()
        }

        // get input value from key input
        let mut horizontal = input.axis_value(&AxisBinding::Horizontal).unwrap_or(0.0);
        let mut vertical = input.axis_value(&AxisBinding::Vertical).unwrap_or(0.0);
        let mut rotate_right = input.action_is_down(&ActionBinding::RotateRight).unwrap_or(false);
        let mut rotate_left = input.action_is_down(&ActionBinding::RotateLeft).unwrap_or(false);
        let mut shoot = input.action_is_down(&ActionBinding::Shoot).unwrap_or(false);

        // Sanitize input
        self.update_key_status(horizontal, vertical, rotate_right, rotate_left, shoot);
        self.delay_hold_input(&mut horizontal, &mut vertical, &mut rotate_right, &mut rotate_left, &mut shoot, time.delta_seconds());
        self.mutual_exclude(&mut horizontal, &mut vertical, &mut rotate_right, &mut rotate_left, &mut shoot);
        
        // TODO Set iput to input_cache
    }
}

struct KeyStatus {
    pub horizontal: KeyPressType,
    pub vertical: KeyPressType,
    pub right_rotate: KeyPressType,
    pub left_rotate: KeyPressType,
    pub shoot: KeyPressType,
}

impl Default for KeyStatus {
    fn default() -> Self {
        Self {
            horizontal: KeyPressType::None,
            vertical: KeyPressType::None,
            right_rotate: KeyPressType::None,
            left_rotate: KeyPressType::None,
            shoot: KeyPressType::None,
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

    fn update_shoot(&mut self, set_to_none: bool) {

        if set_to_none {
            self.shoot = KeyPressType::None;
            return;
        }

        match self.shoot {
            KeyPressType::None => self.shoot = KeyPressType::Click,
            KeyPressType::Click => self.shoot = KeyPressType::Hold,
            _ => ()
        }
    }
}
