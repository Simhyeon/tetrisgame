use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage, WriteExpect, LazyUpdate, Write, ReadExpect, Entity, Entities},
    shrev::{ReaderId, EventChannel},
};
use std::collections::HashMap;

use crate::component::stt_block::StaticBlock;
use crate::system::stack_system::StackEvent;

use std::cmp::Ordering;

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

struct Container {
    height: f32,
    id: Entity,
}

impl Container {
    pub fn new(height: f32, id: Entity) -> Self {
        Self { 
            height,
            id,
        }
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
            if let StackEvent::Stacked = event {
                // Stack Event called

                let mut vector: Vec<Container> = Vec::new();
                for (entity ,local, _) in (&entities, &locals, &stt_blocks).join() {
                    vector.push(Container::new(local.translation().y, entity));
                }

                // Debug Code
                //println!("Block size is : {}", vector.len());

                // Sort vector array
                vector.sort_unstable_by(|a,b| a.height.partial_cmp(&b.height).unwrap());

                // Check Count
                let mut prior: f32 = -1.0; // Inital value is -1
                for counter in 0..vector.len() {
                    if prior.partial_cmp(&vector[counter].height).unwrap() != Ordering::Equal {
                        // Update index
                        prior = vector[counter].height;
                        //println!("Found new transform : {}", prior);

                        // Find all occurence
                        let new_vec: Vec<&Container> = vector.iter().filter(|&x| x.height == prior).collect();

                        // If line is full delte all entities
                        if new_vec.len() == 10 { // Hard coded for now TODO SHould be soft coded
                            println!("Full line collapsing which is {}", prior);
                            for item in new_vec {
                                entities.delete(item.id).expect("ERR");
                            }
                        }
                    } else {
                        continue;
                    }
                }
            }
        }
    }
}
