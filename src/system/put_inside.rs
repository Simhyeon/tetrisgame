use amethyst::{
//    prelude::*,
    core::timing::Time,
    core::transform::{Transform, Parent},
//    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{WriteExpect, System, ReadStorage, Join, Read, SystemData, WriteStorage, World, Write, ReadExpect, Entities},
    input::{InputHandler},
    shrev::{ReaderId, EventChannel},
};

use crate::system::stack_system::StackEvent;
use crate::component::dyn_block::{DynamicBlock, DynBlockHandler, Rotation};
use crate::component::stt_block::StaticBlock;
use crate::config::{MovementBindingTypes, AxisBinding, ActionBinding};
use crate::world::{
    block_data::BlockData,
    gravity_status::GravityStatus,
    key_int::KeyInt,
    stack_status::StackStatus,
};
use crate::utils;
use std::f64::consts::PI;
use std::cmp::Ordering;

#[derive(SystemDesc, Default)]
pub struct PutInsideSystem;


impl<'s> System<'s> for PutInsideSystem {
    type SystemData = (
        ReadExpect<'s, DynBlockHandler>,
        ReadStorage<'s, DynamicBlock>,
        ReadStorage<'s, StaticBlock>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (handler, blocks, stt, mut locals): Self::SystemData) {
        for entity in handler.blocks.iter() {
            let local_matrix = locals.get(*entity).unwrap().global_matrix();
            if local_matrix.m14 > 405.0 {
                locals.get_mut(handler.parent.unwrap()).unwrap().append_translation_xyz(-45.0, 0.0, 0.0);
            } else if local_matrix.m14 < 0.0 {
                locals.get_mut(handler.parent.unwrap()).unwrap().append_translation_xyz(45.0, 0.0, 0.0);
            }
        }
    }
}
