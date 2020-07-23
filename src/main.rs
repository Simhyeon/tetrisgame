use amethyst::{
    prelude::*,
    core::transform::TransformBundle,
    core::frame_limiter::FrameRateLimitStrategy,
    utils::application_root_dir,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    input::InputBundle,
};

mod state;
mod system;
mod component;
mod config;

use crate::system::{
    gravity_system::GravitySystem,
    keyinput_system::KeyInputSystem,
    stack_system::StackSystem,
    spawner_system::SpawnerSystem,
};

use crate::state::{
    main_state::MainState,
    loading_state::LoadingState,
};
use crate::config::MovementBindingTypes;

fn main() -> amethyst::Result<()> {

    // ** Initial Setup
    // Logger
    amethyst::start_logger(Default::default());

    // Configs
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let input_config = app_root.join("config").join("input.ron");
    let input_bundle = InputBundle::<MovementBindingTypes>::new()
        .with_bindings_from_file(input_config)?;

    // TODO 아직 키보드 바인딩은 필요없다. 
    //let binding_path = app_root.join("config").join("bindings.ron");
    //let input_bundle = InputBundle::<-CustomBindingTypes->::new()
    //.with_bindings_from_file(binding_path)?;

    // Spawn World
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                .with_clear([255.0, 255.0, 255.0, 0.0]),
            )
            .with_plugin(RenderFlat2D::default())
            //.with_plugin(RenderUi::default()) TODO 아직 UI는 필요없다. 
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?;
        //.with(KeyInputSystem::default(), "keyinput_system", &[])
        //.with(GravitySystem::default(), "gravity_system", &[])
        //.with(StackSystem::default(), "stack_system", &["gravity_system"])
        //.with(SpawnerSystem::default(), "spawner_system", &["stack_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, LoadingState::default(), game_data)?;
    game.run();

    Ok(())
}
