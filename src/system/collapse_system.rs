use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage, WriteExpect, LazyUpdate, Write, ReadExpect, Entity, Entities},
    shrev::{ReaderId, EventChannel},
};
use priority_queue::PriorityQueue;

use crate::component::stt_block::StaticBlock;
use crate::system::stack_system::StackEvent;


#[derive(SystemDesc)]
pub struct CollapseSystem{
    reader_id : ReaderId<StackEvent>,
}

impl CollapseSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<StackEvent>>().register_reader();
        Self { reader_id }
    }
}

impl<'s> System<'s> for CollapseSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, StaticBlock>,
        Read<'s, EventChannel<StackEvent>>,
    );

    fn run(&mut self, (entities, locals, stt_blocks, event_channel) : Self::SystemData) {
        for event in event_channel.read(&mut self.reader_id) {
            println!("Received an event: {:?}", event);
        }
    }
}
