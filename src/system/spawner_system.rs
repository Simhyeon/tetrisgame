use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::component::Block;

#[derive(SystemDesc)]
pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem{
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData){
        let world = data.world;
    }
}
