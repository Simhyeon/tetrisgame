use amethyst::{
    prelude::*,
    ecs::{Dispatcher, DispatcherBuilder, World},
    core::ArcThreadPool,
};

use crate::system::{
    stack_system::StackSystem,
    spawner_system::SpawnerSystem,
    gravity_system::GravitySystem,
    keyinput_system::KeyInputSystem,
    collapse_system::CollapseSystem,
    physics_system::PhysicsSystem,
    env_allocator::EnvAllocator,
};

use crate::events::GameEvent;
use crate::state::game_over::GameOverState;

#[derive(Default)]
pub struct MainState<'a, 'b>{
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for MainState<'a, 'b>{
    fn on_start(&mut self, mut data : StateData<'_, GameData<'_, '_>>) {
        let world = &mut data.world;
        //self.reader_id = Some(world.fetch_mut::<EventChannel<GameState>>().register_reader());

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(SpawnerSystem::default(), "spawner_system", &[]);
        dispatcher_builder.add(KeyInputSystem::new(), "keyinput_system", &[]);
        dispatcher_builder.add(GravitySystem::new(world), "gravity_system", &[]);
        dispatcher_builder.add(PhysicsSystem::default(), "physics_system", &["keyinput_system" , "spawner_system", "gravity_system"]);
        //dispatcher_builder.add(PhysicsSystem::default(), "physics_system", &["keyinput_system" , "spawner_system"]);
        dispatcher_builder.add(StackSystem::new(), "stack_system", &["physics_system"]);
        dispatcher_builder.add(CollapseSystem::default(), "collapse_system", &["stack_system"]);
        dispatcher_builder.add(EnvAllocator::default(), "env_allocator", &["stack_system"]);
        //dispatcher_builder.add(EnvAllocator::default(), "env_allocator", &["collapse_system"]);

        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {

        let world = &data.world;

        if let GameEvent::GameOver = *world.read_resource::<GameEvent>() {
            return Trans::Replace(Box::new(GameOverState::default()));
        }

        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        Trans::None
    }
}
