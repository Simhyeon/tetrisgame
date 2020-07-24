use amethyst::{
    core::math::Translation3,
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
        for (_, local, ()) in (&dyn_blocks, &locals, !&stt_blocks).join() {
            // get 이 consume 하는 지를 확인해 보자. 
            if local.translation().y == 45.0 {
                //println!("{}", local.translation().y);
                //stack = true;
                for entity in handler.blocks.clone() {
                    stt_blocks.insert(entity, StaticBlock).expect("ERR");
                }
                handler.blocks.clear();
                event_channel.single_write(StackEvent::Stacked);
                return;
            }
            // Check all locals without dynamicBlocks
        }

        let mut do_stack = false;
        for (local, _) in (&locals, &stt_blocks).join() {
            for entity in handler.blocks.clone() {
                // Tempoary Values
                let mut new_target = local.clone();
                let target_pos = new_target.append_translation_xyz(0.0,45.0,0.0).translation();
                let block_pos = locals.get(entity).unwrap().translation();

                // 솔직히 왜 아직도 y 값이 같을 때 인지 잘 모르겠다... 분명 45 만큼 차이가 있는 게
                // 정상 아닌가?
                if block_pos.x == target_pos.x && block_pos.y == target_pos.y {
                    do_stack = true;
                    println!("Stack upon other blocks");
                    event_channel.single_write(StackEvent::Stacked);
                    break;
                }
            }
        }

        if do_stack {
            for entity in handler.blocks.clone() {
                stt_blocks.insert(entity, StaticBlock).expect("ERR");
            }
            handler.blocks.clear();
        }
    }
}
