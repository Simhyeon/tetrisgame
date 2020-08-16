use amethyst::{
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

pub enum OffsetType {
    Main,
    Sub,
}

#[derive(SystemDesc, Default)]
pub struct PhysicsSystem{
    movement_cache: (f32,f32),
}

impl<'s> System<'s> for PhysicsSystem {
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
    
        if handler.blocks.len() == 0 {
            return;
        }


        // Physics Post processing
        // Converts input cache into real physics operation queue
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
                queue.set_rotation(input_cache.rotation);
            }
            _ => ()
        }

        // Set shoot to queue if present
        queue.set_shoot(input_cache.shoot);

        // Start operations  From here
        // Should set movement_cache default
        self.movement_cache = (0.0, 0.0);

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

        match rotation {
            Rotation::Right | Rotation::Left => {

                if let Some(_) = handler.config.offset {
                    self.check_rotataion_block(&handler, origin, &block_data, &mut blockage, OffsetType::Main);
                } else {
                    blockage.block_rotation();
                }

                if let Some(_) = handler.config.sub_offset{
                    self.check_rotataion_block(&handler, origin, &block_data, &mut blockage, OffsetType::Sub);
                }
            }
            _ => ()
        }

        // IMPORTANT! This method should be called regardless of user input. 
        // Becuase blockage resource is referenced by other systems. eg. stacking
        self.check_axis_block(&dyn_blocks, &*block_data, &mut blockage);
        self.set_blockage(&mut new_queue, &mut rotation, &mut blockage);

        if queue.get_shoot() && !blockage.shoot {
            let shoot_distance = self.get_shoot_distance(origin, &dyn_blocks, &block_data);
            locals.get_mut(handler.parent.unwrap()).unwrap().prepend_translation_y(-shoot_distance);
            return;
        }

        // Append Location(translation)
        let origin = locals.get_mut(handler.parent.unwrap()).unwrap();
        while new_queue.len() != 0 {

            let item = new_queue.last().unwrap();
            self.movement_cache.0 += item.0;
            self.movement_cache.1 += item.1;
            new_queue.pop();

            self.check_axis_block(&dyn_blocks, &*block_data, &mut blockage);
            self.set_blockage(&mut new_queue, &mut rotation, &mut blockage);
            //println!("Queue Length : {} --- Movement -> {:?}", new_queue.len(),self.movement_cache);
        }
        // This is the real logic to be executed
        origin.prepend_translation_x(self.movement_cache.0).prepend_translation_y(self.movement_cache.1);

        // Inputs are mutually exclusive 
        // therefore no need to worry about movement_cache not being properly 
        // handled in conseqeunt translation + rotation scenario.
        // Append Rotation
        if let Some(_) = handler.config.offset {
            match rotation {
                Rotation::Right => {
                    handler.rotate_handler(Rotation::Right);
                    origin.prepend_rotation_z_axis((PI * 0.5) as f32);
                    self.check_axis_block(&dyn_blocks, &*block_data, &mut blockage);
                    self.set_blockage(&mut new_queue, &mut rotation, &mut blockage);
                }
                Rotation::Left => {
                    handler.rotate_handler(Rotation::Left);
                    origin.prepend_rotation_z_axis((-PI * 0.5) as f32);
                    self.check_axis_block(&dyn_blocks, &*block_data, &mut blockage);
                    self.set_blockage(&mut new_queue, &mut rotation, &mut blockage);
                }
                _ =>()
            }
        }

        // For cleanup and set blockage properly after final operation
        self.check_axis_block(&dyn_blocks, &*block_data, &mut blockage);
        self.set_blockage(&mut new_queue, &mut rotation, &mut blockage);
    }
}

impl<'s> PhysicsSystem {
    fn check_axis_block(&mut self, dyn_blocks : &Vec<(f32,f32)>, block_data : &BlockData, blockage: &mut Blockage) {
        for (x,y) in dyn_blocks.iter().map(|(x,y)| (x.round() + self.movement_cache.0, y.round() + self.movement_cache.1)) {


            // Right Blockage
            if x == PLAY_PANE_WIDTH - BLOCK_SIZE || 
                block_data.find_block(x + BLOCK_SIZE, y) {
                blockage.axis_right = true;
            }

            // left Blockage
            if x == 0.0 || 
                block_data.find_block(x - BLOCK_SIZE, y) {
                blockage.axis_left = true;
            }

            // Down Blockage
            if y == BLOCK_SIZE || 
                block_data.find_block(x, y - BLOCK_SIZE) {
                    println!("Blocking Y");
                    blockage.axis_down = true;
                    blockage.shoot = true;
            }
        }
    }

    fn check_rotataion_block(&mut self, handler: &DynBlockHandler, origin: Matrix4<f32>, block_data : &BlockData, blockage: &mut Blockage, offset_type : OffsetType) {

        let right_offset :(f32,f32);
        let left_offset :(f32,f32);

        if let OffsetType::Main = offset_type {
            right_offset = handler.get_count(Rotation::Right);
            left_offset = handler.get_count(Rotation::Left);
        } else {
            right_offset = handler.get_sub_count(Rotation::Right);
            left_offset = handler.get_sub_count(Rotation::Left);
        }

        let (right_offset_start, right_offset_end) = (right_offset.0, right_offset.1);
        let (left_offset_start, left_offset_end) = (left_offset.0, left_offset.1);
        let (mut x_offset, mut y_offset): (f32, f32) = (0.0, 0.0);

        match handler.rotation {
            Rotation::Up | Rotation::Down => {
                if let OffsetType::Main = offset_type {
                    x_offset = 1.0;
                    y_offset = 0.0;
                } else {
                    x_offset = 0.0;
                    y_offset = 1.0;
                }
            },
            Rotation::Right | Rotation::Left => {
                if let OffsetType::Main = offset_type{
                    x_offset = 0.0;
                    y_offset = 1.0;
                } else {
                    x_offset = 1.0;
                    y_offset = 0.0;
                }
            },
            _ => ()
        }

        // Check right rotation blockage
        for count in right_offset_start as i32 .. right_offset_end as i32 + 1 {
            let (x_check, y_check) = (origin.m14.round() + (count as f32 * x_offset) * BLOCK_SIZE , origin.m24.round() + (count as f32 * y_offset) * BLOCK_SIZE);

            if x_check < 0.0                        ||
               x_check >= PLAY_PANE_WIDTH           ||
               y_check < BLOCK_SIZE                 ||
               y_check >  GAME_HEIGHT               ||
               block_data.find_block(x_check, y_check){
                   blockage.rotate_right = true;
                   break;
            }
        }

        // Check left rotation blockage
        for count in left_offset_start as i32 .. left_offset_end as i32 + 1 {
            let (x_check, y_check) = (origin.m14.round() + (count as f32 * x_offset) * BLOCK_SIZE, origin.m24.round() + (count as f32 * y_offset) * BLOCK_SIZE);

            if x_check < 0.0                        ||
               x_check >= PLAY_PANE_WIDTH           ||
               y_check <= BLOCK_SIZE                ||
               y_check >  GAME_HEIGHT               ||
               block_data.find_block(x_check, y_check){
                   blockage.rotate_left = true;
                   break;
            }
        }

        // TODO Suboffset 
    }

    fn get_shoot_distance(&self, origin: Matrix4<f32>, dyn_blocks : &Vec<(f32,f32)>, block_data : &BlockData) -> f32 {

        let mut distance: usize = 20;
        for (x,y) in dyn_blocks.iter().map(|(x,y)| (BlockData::get_row_index_from_m14(*x).unwrap(), BlockData::get_col_index_from_m24(*y).unwrap())) {

            //x, y is entitytis' x and y
            if let Some((row, col)) = block_data.get_top_block_index(x, y) {
                if y - col - 1<= distance {
                    distance = y - col - 1;
                }

            } else {
                if y<= distance {
                    distance = y;
                }
            }
        }

        println!("Shoot distance : {}", distance as f32 * BLOCK_SIZE);
        distance as f32 * BLOCK_SIZE
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
