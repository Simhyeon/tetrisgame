use amethyst::{
    prelude::*,
    ecs::{Dispatcher, DispatcherBuilder},
    core::ArcThreadPool,
};

use crate::system::{
    stack_system::StackSystem,
    spawner_system::SpawnerSystem,
    gravity_system::GravitySystem,
    keyinput_system::KeyInputSystem,
    collapse_system::CollapseSystem,
};

use crate::config::BlocksConfig;

#[derive(Default)]
pub struct MainState<'a, 'b>{
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for MainState<'a, 'b>{
    fn on_start(&mut self, mut data : StateData<'_, GameData<'_, '_>>) {
        let world = &mut data.world;

        let mut dispatcher_builder = DispatcherBuilder::new();
        //dispatcher_builder.add(GravitySystem::default(), "gravity_system", &[]);
        dispatcher_builder.add(KeyInputSystem::default(), "keyinput_system", &[]);
        // Backup for Stack system
        //dispatcher_builder.add(StackSystem::default(), "stack_system", &["gravity_system"]);
        dispatcher_builder.add(StackSystem::default(), "stack_system", &["keyinput_system"]);
        dispatcher_builder.add(CollapseSystem::new(world), "collapse_system", &["stack_system"]);
        dispatcher_builder.add(SpawnerSystem::default(), "spawner_system", &["stack_system"]);

        // Backup
        //
        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }
}
