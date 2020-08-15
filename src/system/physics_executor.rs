use amethyst::{
    core::math::Vector3,
    core::math::Matrix4,
    core::transform::{Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadStorage, WriteExpect, Write, Read, World, LazyUpdate, ReadExpect},
};

use crate::component::{
    dyn_block::DynBlockHandler,
    stt_block::StaticBlock,
};

use crate::consts::*;
use crate::commons::Rotation;

use crate::world::{
    input_cache::AxisType,
    physics_queue::PhysicsQueue,
    block_data::BlockData,
};

#[derive(Default)]
struct Blockage {
    axis_right : bool,
    axis_left : bool,
    axis_down : bool,
    rotate_right : bool,
    rotate_left : bool,
    shoot: bool,
}

#[derive(SystemDesc, Default)]
pub struct PhysicsExecutor {
    blockage: Blockage,
}

impl<'s> System<'s> for PhysicsExecutor {
    type SystemData = (
        ReadExpect<'s, DynBlockHandler>,
        ReadExpect<'s, PhysicsQueue>,
        ReadExpect<'s, BlockData>,
        WriteStorage<'s, Transform>,
    );

    // Read from physisqueue and apply physics accordingly 
    fn run(&mut self, (handler, queue, block_data, mut locals): Self::SystemData) {
        let dyn_blocks : Vec<(f32,f32)> = handler.blocks
            .iter()
            .map(|&entity| {
                let local = locals.get(entity).unwrap().global_matrix();
                (local.m14, local.m24)
            }).collect();

        let origin = locals.get(handler.parent.unwrap()).unwrap().global_matrix().clone();

        if queue.get_shoot() && !self.blockage.shoot {
            let shoot_distance = self.get_shoot_distance(origin, &dyn_blocks);
            locals.get_mut(handler.parent.unwrap()).unwrap().prepend_translation_y(shoot_distance);
            return;
        }

        self.check_axis_block(&dyn_blocks, &*block_data);
        self.check_rotataion_block(&handler, origin, &block_data);

        for item in queue.get_queue().unwrap().iter() {

        }
    }
}

impl<'s> PhysicsExecutor {
    fn check_axis_block(&mut self, dyn_blocks : &Vec<(f32,f32)>, block_data : &BlockData) {
        for (x,y) in dyn_blocks.iter().map(|(x,y)| (x.round(), y.round())) {

            // Right Blockage
            if x == PLAY_PANE_WIDTH - BLOCK_SIZE || 
                block_data.find_block(x + 45.0, y) {
                self.blockage.rotate_right = true;
            }

            // left Blockage
            if x == 0.0 || 
                block_data.find_block(x - 45.0, y) {
                self.blockage.axis_left = true;
            }

            // Down Blockage
            if y == BLOCK_SIZE || 
                block_data.find_block(x, y - 45.0) {
                    self.blockage.axis_down = true;
            }
        }

    }

    fn check_rotataion_block(&mut self, handler: &DynBlockHandler, origin: Matrix4<f32>, block_data : &BlockData) {

        let (right_offset_start, right_offset_end) = handler.get_count(Rotation::Right);
        let (left_offset_start, left_offset_end) = handler.get_count(Rotation::Left);
        let (x_offset, y_offset): (f32, f32);

        match handler.rotation {
            Rotation::Up | Rotation::Down => {
                x_offset = 1.0;
                y_offset = 0.0;
            },
            Rotation::Right | Rotation::Left => {
                x_offset = 0.0;
                y_offset = 1.0;
            },
        }

        // Check right rotation blockage
        for count in right_offset_start as i32 .. right_offset_end as i32 + 1 {
            let (x_check, y_check) = (origin.m14.round() + (count as f32 * x_offset) , origin.m24.round() + (count as f32 * y_offset));

            if x_check <= -BLOCK_SIZE               ||
               x_check >= PLAY_PANE_WIDTH           ||
               y_check <= 0.0                       ||
               y_check >= GAME_HEIGHT + BLOCK_SIZE  ||
               block_data.find_block(x_check, y_check){
                   self.blockage.rotate_right = true;
                   break;
            }
        }

        // Check left rotation blockage
        for count in left_offset_start as i32 .. left_offset_end as i32 + 1 {
            let (x_check, y_check) = (origin.m14.round() + (count as f32 * x_offset) , origin.m24.round() + (count as f32 * y_offset));

            if x_check <= -BLOCK_SIZE               ||
               x_check >= PLAY_PANE_WIDTH           ||
               y_check <= 0.0                       ||
               y_check >= GAME_HEIGHT + BLOCK_SIZE  ||
               block_data.find_block(x_check, y_check){
                   self.blockage.rotate_left = true;
                   break;
            }
        }
    }

    fn get_shoot_distance(&self, origin: Matrix4<f32>, dyn_blocks : &Vec<(f32,f32)>) -> f32 {
        0.0
    }
}
