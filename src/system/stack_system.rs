use amethyst::{
    core::math::Vector3,
    core::SystemDesc,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadStorage, ReadExpect, Read, WriteExpect}
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};

#[derive(SystemDesc, Default)]
pub struct StackSystem;


impl<'s> System<'s> for StackSystem {
  type SystemData = (
      WriteExpect<'s, DynBlockHandler>,
      ReadStorage<'s, DynamicBlock>,
      //ReadStorage<'s, StaticBlock>,
      ReadStorage<'s, Transform>,
  );

  fn run(&mut self, (mut handler, dyn_blocks, locals): Self::SystemData) {
      if handler.blocks.len() == 0 {
        return;
      }
      //let mut stack: bool = false;
      'outer: for static_local in locals.join(){
          for (_, local) in (&dyn_blocks, &locals).join() {
              // get 이 consume 하는 지를 확인해 보자. 
              if local.translation().y == 45.0 {
                  //stack = true;
                  println!("CHECKED ground!");
                  handler.blocks.clear();
                  break 'outer;
              }
              // Check all locals without dynamicBlocks
          }
      }
  }
}
