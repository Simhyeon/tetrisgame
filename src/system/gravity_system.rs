use amethyst::{
//    prelude::*,
    core::timing::Time,
//    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{ReadExpect, System, Read, SystemData, WriteStorage, World, ReadStorage, Join, WriteExpect},
    shrev::{EventChannel, ReaderId},
};

use crate::world::physics_queue::PhysicsQueue;

pub enum Gravity{
    Reset,
    None,
}

#[derive(SystemDesc)]
pub struct GravitySystem{
    pub time_delay: f32,
    //stop_gravity: bool,
    move_delay: f32,
    reader_id : ReaderId<Gravity>,
}

impl GravitySystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<Gravity>>().register_reader();
        Self { 
            time_delay : 0.0,
            move_delay : 1.0,
            //stop_gravity : false,
            reader_id 
        }
    }
}

impl<'s> System<'s> for GravitySystem{
    type SystemData = (
        Read<'s, Time>,
        WriteExpect<'s, PhysicsQueue>,
        Read<'s, EventChannel<Gravity>>,
    );

    fn run(&mut self, (time, mut queue, gravity_event): Self::SystemData){

        for event in gravity_event.read(&mut self.reader_id) {
            match event {
                Gravity::Reset => {
                    self.time_delay = 0.0;
                    break;
                }
                _ => break
            }
        }

        // Increase time_delay count
        self.time_delay += time.delta_seconds();

        // if time ha reached then move downward
        if self.time_delay >= self.move_delay {
            queue.add_to_queue((0.0, -45.0));

            self.time_delay = 0.0;

            if self.move_delay >= 0.3 {
                self.move_delay -= 0.005;
            }
        }

    }
}
