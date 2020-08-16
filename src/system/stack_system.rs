use amethyst::{
    core::timing::Time,
    core::transform::{Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadStorage, WriteExpect, Write, Read, World, LazyUpdate, ReadExpect},
    shrev::{EventChannel, ReaderId},
    shred::PanicHandler,
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};
use crate::component::stt_block::StaticBlock;
use crate::world::{
    block_data::BlockData,
    blockage::Blockage,
    physics_queue::PhysicsQueue,
    input_cache::{InputCache,AxisType},
};
use crate::events::GameEvent;
use crate::system::gravity_system::Gravity;

const STACKDELAY: f32 = 0.3;

#[derive(Debug)]
pub enum StackStatus {
    TobeStacked,
    ShootStack,
    None,
}

#[derive(SystemDesc)]
pub struct StackSystem {
    stack_delay: f32,
    stack_status: StackStatus,
}

impl StackSystem {
    pub fn new() -> Self {
        Self {  
            stack_delay: STACKDELAY,
            stack_status: StackStatus::None,
        }
    }
}

impl<'s> System<'s> for StackSystem {
    type SystemData = (
        WriteExpect<'s, DynBlockHandler>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, StaticBlock>,
        Read<'s, Time>,
        WriteExpect<'s, BlockData>,
        WriteExpect<'s, GameEvent>,
        ReadExpect<'s, PhysicsQueue>,
        ReadExpect<'s, Blockage>,
        ReadExpect<'s, InputCache>,
        Write<'s, EventChannel<Gravity>>
    );

    // TODO Change to_be_stacked value as some kind of trigger
    // TODO THIS CODE IS SHIT FUCK ME
    fn run(&mut self, (
            mut handler, 
            locals,
            mut stt_blocks, 
            time, 
            mut block_data, 
            mut game_event,
            queue,
            blockage,
            input_cache,
            mut gravity_event,
    ): Self::SystemData) {

        if handler.blocks.len() == 0 {
            return;
        }

        match self.stack_status {
            StackStatus::TobeStacked | StackStatus::None => {
                if let StackStatus::TobeStacked = self.stack_status {
                    self.stack_delay -= time.delta_seconds();
                }

                //TODO Problem is axix_down is resetted
                if blockage.axis_down {
                    self.stack_status = StackStatus::TobeStacked;
                } else {
                    self.stack_status = StackStatus::None;
                    self.stack_delay = STACKDELAY;
                }

                // Set ShootStack
                // This code override prior stack_stratus 
                // therefore should come later.
                if queue.get_shoot() {
                    self.stack_status = StackStatus::ShootStack;
                }
            }
            _ => ()
        }

        let mut do_stack: bool = false;

        match input_cache.axis {
            AxisType::Right | AxisType::Left => {
                self.stack_delay += 0.01;
            }
            _ =>()
        }

        if let StackStatus::ShootStack = self.stack_status {
            do_stack = true;
        } else if self.stack_delay <= 0.0{
            do_stack = true;
        }

        if !blockage.axis_down {
            do_stack = false;
        }

        if do_stack {
            // Now stack the blocks
            for entity in &handler.blocks {

                stt_blocks.insert(*entity, StaticBlock).expect("ERR");
                // Add block to block_data
                let matrix_m = locals.get(*entity).unwrap().global_matrix();
                match block_data.add_block(matrix_m.m14.round(), matrix_m.m24.round(), entity.clone()) {
                    Ok(_) => (),
                    Err(_) => {
                        *game_event = GameEvent::GameOver;
                    }
                }
            }

            // Reset variables
            // TODO Is resetting handler in stack system desirable?
            handler.blocks.clear();
            handler.parent.take();
            self.stack_delay = STACKDELAY;
            self.stack_status = StackStatus::None;
            gravity_event.single_write(Gravity::Reset);
            //*key_int = KeyInt::None;
            //*collapse_status = CollapseStatus::Triggered;
        }

    }
}
