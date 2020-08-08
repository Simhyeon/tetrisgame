use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadStorage, WriteExpect, Write, Read},
    shrev::EventChannel,
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};
use crate::component::stt_block::StaticBlock;
use crate::system::keyinput_system::KeyInt;

const STACKDELAY: f32 = 0.3;
const KEYINTDELAY: f32 = 0.05;

#[derive(Debug)]
pub enum StackEvent {
    Stacked,
    ToBeStacked,
    Free,
}

#[derive(SystemDesc)]
pub struct StackSystem {
    to_be_stacked: bool,
    stack_delay: f32,
}

impl Default for StackSystem {
    fn default() -> Self {
        Self {
            to_be_stacked: false,
            stack_delay: STACKDELAY,
        }
    }
}

// Simple arrangements before refactoring Codes
// StackSystem first checks if dynamic blocsk are in the state to be stacked. which is one block
// upward ground or static blocks.
// If stacked stated are detected send to_be_stacked event to channel
// Gravity read the event and stop gravity logic.
// After stack delay has passed stack system really stack the blocks and send
// stacked event to channel
//
// And what I should refactor is about chaning event logics so that  
// to_be_stacked event can be reverted when stack is not detected.
// which also should retrigger gravity system 
impl<'s> System<'s> for StackSystem {
    type SystemData = (
        WriteExpect<'s, DynBlockHandler>,
        ReadStorage<'s, DynamicBlock>,
        WriteStorage<'s, StaticBlock>,
        ReadStorage<'s, Transform>,
        Write<'s, EventChannel<StackEvent>>,
        Write<'s, EventChannel<KeyInt>>,
        Read<'s, Time>
    );

    // TODO Change to_be_stacked value as some kind of trigger
    // TODO THIS CODE IS SHIT FUCK ME
    fn run(&mut self, (mut handler, dyn_blocks, mut stt_blocks, locals, mut stack_event_channel,mut key_event_channel, time): Self::SystemData) {
        if handler.blocks.len() == 0 {
            return;
        }

        let mut stack_confirm = false;
        if self.to_be_stacked {
            //Wait for certain times and 
            self.stack_delay -= time.delta_seconds();
            if self.stack_delay <= 0.0 {
                stack_confirm = true;
            } else if self.stack_delay <= KEYINTDELAY {
                // This else if statement is to prevent from user to give input at the same time
                // block is stacked. Which makes block stacked on air. Which is not desired
                // actions.
                // However this method is not good at all. Since sending key event is based on
                // delta time which is time between continous function calls. and such delta
                // time can be different among other devices and enviorments.
                // For example when os failed to allocate enough resource for ths program
                // then this functionality might fail. 
                // (While it is also highly expected to fail to get user input anyway.)
                key_event_channel.single_write(KeyInt::Stack);
            }
        }

        let mut to_free : bool = false;
        'outer :for (dyn_local, _, ()) in (&locals, &dyn_blocks, !&stt_blocks).join() {
            if dyn_local.global_matrix().m24.round() == 45.0 { // this is when to be stacked
                if !self.to_be_stacked {
                    self.to_be_stacked = true;
                    println!("TOBESTACKED");
                    stack_event_channel.single_write(StackEvent::ToBeStacked);
                }
                to_free = false;
                break;
            }

            for (local, _) in (&locals, &stt_blocks).join() {
                if local.global_matrix().m24.round() == dyn_local.global_matrix().m24.round() - 45.0 
                    && local.global_matrix().m14.round() == dyn_local.global_matrix().m14.round() {
                        if !self.to_be_stacked {
                            self.to_be_stacked = true;
                            println!("TOBESTACKED");
                            stack_event_channel.single_write(StackEvent::ToBeStacked);
                        }
                        to_free = false;
                        break 'outer;
                    }
            }

            // During Looping no stack conditions has been detected;
            // which means condifion for freeing gravity system has been met
            to_free = true;
        }

        // if gravtiy free condition has been met and also 
        // to_be_stacked was already called which means stack system priorly
        // detected stack call  and now it is not detected.
        if self.to_be_stacked && to_free {
            println!("Free stack event");
            stack_event_channel.single_write(StackEvent::Free);
            self.stack_delay = STACKDELAY;
            self.to_be_stacked = false;
            return;
        }

        if stack_confirm {
            // Reset variables
            self.stack_delay = STACKDELAY;
            self.to_be_stacked = false;

            // Now stack the blocks
            for entity in &handler.blocks {
                stt_blocks.insert(*entity, StaticBlock).expect("ERR");
            }
            handler.blocks.clear();
            stack_event_channel.single_write(StackEvent::Stacked);
            println!("Stacked!");
        }
    }
}
