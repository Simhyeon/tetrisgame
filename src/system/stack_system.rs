use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadStorage, WriteExpect, Write},
    shrev::EventChannel,
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};
use crate::component::stt_block::StaticBlock;

#[derive(Debug)]
pub enum StackEvent {
    Stacked,
    None,
}

#[derive(SystemDesc, Default)]
pub struct StackSystem;

impl<'s> System<'s> for StackSystem {
    type SystemData = (
        WriteExpect<'s, DynBlockHandler>,
        ReadStorage<'s, DynamicBlock>,
        WriteStorage<'s, StaticBlock>,
        ReadStorage<'s, Transform>,
        Write<'s, EventChannel<StackEvent>>,
    );

    fn run(&mut self, (mut handler, dyn_blocks, mut stt_blocks, locals, mut event_channel): Self::SystemData) {
        if handler.blocks.len() == 0 {
            return;
        }

        let mut to_be_stacked = false;
        for (dyn_local, _, ()) in (&locals, &dyn_blocks, !&stt_blocks).join() {
            if dyn_local.global_matrix().m24 == 45.0 {
                to_be_stacked = true;
                break;
            }
            for (local, _) in (&locals, &stt_blocks).join() {
                if local.global_matrix().m24 == dyn_local.global_matrix().m24 - 45.0 
                    && local.global_matrix().m14 == dyn_local.global_matrix().m14 {
                    to_be_stacked = true;
                    break;
                }
            }
        }

        if to_be_stacked {
            for entity in &handler.blocks {
                stt_blocks.insert(*entity, StaticBlock).expect("ERR");
            }
            handler.blocks.clear();
            event_channel.single_write(StackEvent::Stacked);
            println!("Stacked!");
        }
    }
}
