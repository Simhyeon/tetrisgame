use amethyst::{
    core::math::Vector3,
    core::SystemDesc,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadStorage, ReadExpect, Read, WriteExpect, Entity}
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};
use crate::component::stt_block::StaticBlock;

#[derive(SystemDesc, Default)]
pub struct StackSystem;


impl<'s> System<'s> for StackSystem {
  type SystemData = (
      WriteExpect<'s, DynBlockHandler>,
      ReadStorage<'s, DynamicBlock>,
      WriteStorage<'s, StaticBlock>,
      ReadStorage<'s, Transform>,
  );

  fn run(&mut self, (mut handler, dyn_blocks, mut stt_blocks, locals): Self::SystemData) {
      if handler.blocks.len() == 0 {
        return;
      }
      'outer: for static_local in locals.join(){
          for (_, local, ()) in (&dyn_blocks, &locals, !&stt_blocks).join() {
              // get 이 consume 하는 지를 확인해 보자. 
              if local.translation().y == 45.0 {
                  //stack = true;
                  println!("CHECKED ground!");
                  for entity in handler.blocks.clone() {
                      stt_blocks.insert(entity, StaticBlock);
                  }
                  handler.blocks.clear();
                  break 'outer;
              }
              // Check all locals without dynamicBlocks
          }
      }
  }
}
