use amethyst::{
    core::math::Vector3,
    core::math::Matrix4,
    core::transform::{Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadStorage, WriteExpect, Write, Read, World, LazyUpdate, ReadExpect},
};

use crate::component::dyn_block::DynBlockHandler;

use crate::consts::*;
use crate::commons::Rotation;
use std::f64::consts::PI;

use crate::world::{
    input_cache::{InputCache, AxisType},
    physics_queue::PhysicsQueue,
    block_data::BlockData,
    blockage::Blockage,
};

#[derive(SystemDesc, Default)]
pub struct PhysicsExecutor;

impl<'s> System<'s> for PhysicsExecutor {
    type SystemData = (
        WriteExpect<'s, DynBlockHandler>,
        ReadExpect<'s, InputCache>,
        WriteExpect<'s, Blockage>,
        WriteExpect<'s, PhysicsQueue>,
        ReadExpect<'s, BlockData>,
        WriteStorage<'s, Transform>,
    );

    // Read from physisqueue and apply physics accordingly 
    fn run(&mut self, (mut handler, input_cache, mut blockage, mut queue, block_data, mut locals): Self::SystemData) {
        match input_cache.axis {
            AxisType::Right => {
                queue.add_to_queue((BLOCK_SIZE, 0.0));
            }
            AxisType::Left => {
                queue.add_to_queue((-BLOCK_SIZE, 0.0));
            }
            AxisType::Down => {
                queue.add_to_queue((0.0, -BLOCK_SIZE));
            }
            AxisType::None => {
                ()
            }
        }

        match input_cache.rotation {
            Rotation::Right | Rotation::Left => {
                queue.set_offset(handler.get_count(input_cache.rotation));
                queue.set_sub_offset(handler.get_sub_count(input_cache.rotation));
            }
            _ => ()
        }

        queue.shoot_check();

        let dyn_blocks : Vec<(f32,f32)> = handler.blocks
            .iter()
            .map(|&entity| {
                let local = locals.get(entity).unwrap().global_matrix();
                (local.m14, local.m24)
            }).collect();

        let origin = locals.get(handler.parent.unwrap()).unwrap().global_matrix().clone();

        let mut new_queue = match queue.get_queue() {
            Some(queue) => queue.clone(),
            None => vec![],
        };
        let mut rotation = match queue.get_rotation() {
            Some(rotation) => rotation,
            None => Rotation::None,
        };

        self.check_axis_block(&dyn_blocks, &*block_data, &mut blockage);
        self.check_rotataion_block(&handler, origin, &block_data, &mut blockage);
        self.set_blockage(&mut new_queue, &mut rotation, &mut blockage);

        if queue.get_shoot() && !blockage.shoot {
            let shoot_distance = self.get_shoot_distance(origin, &dyn_blocks);
            locals.get_mut(handler.parent.unwrap()).unwrap().prepend_translation_y(shoot_distance);
            return;
        }

        // Append Location(translation)
        let origin = locals.get_mut(handler.parent.unwrap()).unwrap();
        for item in new_queue.iter() {
            origin.append_translation_xyz(item.0, item.1, 0.0);
        }

        // Append Rotation
        match rotation {
            Rotation::Right => {
                handler.rotate_handler(Rotation::Right);
                origin.prepend_rotation_z_axis((PI * 0.5) as f32);
            }
            Rotation::Left => {
                handler.rotate_handler(Rotation::Left);
                origin.prepend_rotation_z_axis((-PI * 0.5) as f32);
            }
            _ =>()
        }
    }
}

impl<'s> PhysicsExecutor {
    fn check_axis_block(&mut self, dyn_blocks : &Vec<(f32,f32)>, block_data : &BlockData, blockage: &mut Blockage) {
        for (x,y) in dyn_blocks.iter().map(|(x,y)| (x.round(), y.round())) {

            // Right Blockage
            if x == PLAY_PANE_WIDTH - BLOCK_SIZE || 
                block_data.find_block(x + 45.0, y) {
                blockage.rotate_right = true;
            }

            // left Blockage
            if x == 0.0 || 
                block_data.find_block(x - 45.0, y) {
                blockage.axis_left = true;
            }

            // Down Blockage
            if y == BLOCK_SIZE || 
                block_data.find_block(x, y - 45.0) {
                    blockage.axis_down = true;
            }
        }

    }

    fn check_rotataion_block(&mut self, handler: &DynBlockHandler, origin: Matrix4<f32>, block_data : &BlockData, blockage: &mut Blockage) {

        let (right_offset_start, right_offset_end) = handler.get_count(Rotation::Right);
        let (left_offset_start, left_offset_end) = handler.get_count(Rotation::Left);
        let (mut x_offset, mut y_offset): (f32, f32) = (0.0, 0.0);

        match handler.rotation {
            Rotation::Up | Rotation::Down => {
                x_offset = 1.0;
                y_offset = 0.0;
            },
            Rotation::Right | Rotation::Left => {
                x_offset = 0.0;
                y_offset = 1.0;
            },
            _ => ()
        }

        // Check right rotation blockage
        for count in right_offset_start as i32 .. right_offset_end as i32 + 1 {
            let (x_check, y_check) = (origin.m14.round() + (count as f32 * x_offset) , origin.m24.round() + (count as f32 * y_offset));

            if x_check <= -BLOCK_SIZE               ||
               x_check >= PLAY_PANE_WIDTH           ||
               y_check <= 0.0                       ||
               y_check >= GAME_HEIGHT + BLOCK_SIZE  ||
               block_data.find_block(x_check, y_check){
                   blockage.rotate_right = true;
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
                   blockage.rotate_left = true;
                   break;
            }
        }
    }

    fn get_shoot_distance(&self, origin: Matrix4<f32>, dyn_blocks : &Vec<(f32,f32)>) -> f32 {

        //let mut distance: f32 = HEIGHT;
        //let mut top_block : (f32, f32) = (-1.0, -1.0);
        //let mut down_block : (f32, f32)= (-1.0, -1.0);
        //for block_entity in handler.blocks.iter() {
            //let top_matrix = locals.get(*block_entity).unwrap().global_matrix();
            ////println!("({}, {})", top_matrix.m14, top_matrix.m24);
            //// Get top_most location of down_most columns
            //if let Some(entity) = block_data.get_top_block(top_matrix.m14, top_matrix.m24) {
                //let down_matrix = locals.get(entity).unwrap().global_matrix();
                //if top_matrix.m24 - down_matrix.m24 - 45.0 <= distance {
                    //distance = (top_matrix.m24 - down_matrix.m24 - 45.0).round();
                    //top_block = (top_matrix.m14.round(), top_matrix.m24.round());
                    //down_block = (down_matrix.m14, down_matrix.m24);
                //}

            //} else {
                //if top_matrix.m24 - 45.0 <= distance {
                    //distance = top_matrix.m24 - 45.0;
                    //top_block = (top_matrix.m14.round(), top_matrix.m24.round());
                    //down_block = (0.0, 0.0);
                //}
            //}
        //}

        ////println!("Distance is {}", (distance / 45.0).round());
        ////println!("Top Block is {:?}", top_block);
        ////println!("Down Block is {:?}", down_block);

        //let current = locals.get(handler.parent.unwrap()).unwrap().global_matrix().m24;
        ////println!("Before Transform : {}", current);
        //locals.get_mut(handler.parent.unwrap()).unwrap().set_translation_y(current-distance);

        //*stack_status = StackStatus::ShootStack;
        //*key_int = KeyInt::Stack;

        ////println!("End of shoot");
        //let current = locals.get(handler.parent.unwrap()).unwrap().global_matrix().m24;
        ////println!("New Transform : {}", current);
        //return;

        0.0
    }

    fn set_blockage(&mut self, queue: &mut Vec<(f32, f32)>, rotation: &mut Rotation, blockage: &mut Blockage) {
        if blockage.axis_left {
            queue.retain(|&(x,_)| {
                x >= 0.0
            });
        }
        if blockage.axis_right {
            queue.retain(|&(x,_)| {
                x <= 0.0
            });
        }
        if blockage.axis_down {
            queue.retain(|&(_,y)| {
                y >= 0.0
            });
        }

        if blockage.rotate_left {
            if let Rotation::Left = rotation {
                *rotation = Rotation::None;
            }
        }

        if blockage.rotate_right {
            if let Rotation::Right = rotation {
                *rotation = Rotation::None;
            }
        }
    }
}
