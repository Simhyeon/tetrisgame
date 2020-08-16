use amethyst::{
    prelude::*,
    core::transform::TransformBundle,
//    core::frame_limiter::FrameRateLimitStrategy,
    utils::application_root_dir,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    shrev::EventChannel,
    input::{InputBundle},
    ui::{UiBundle, RenderUi},
};

mod state;
mod system;
mod component;
mod config;
mod utils;
mod world;
mod events;
mod commons;
mod consts;

use crate::state::{ loading_state::LoadingState, };
use crate::config::{MovementBindingTypes, BlocksConfig, PaneConfig};
use crate::world::{
    block_data::BlockData,
    blockage::Blockage,
    physics_queue::PhysicsQueue,
    input_cache::InputCache,
};
use crate::events::GameEvent;

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

    let blocks_config = app_root.join("config").join("blocks.ron");
    let blocks_config = BlocksConfig::load(&blocks_config)?;
    // TODO 아직 키보드 바인딩은 필요없다. 
    //let binding_path = app_root.join("config").join("bindings.ron");
    //let input_bundle = InputBundle::<-CustomBindingTypes->::new()
    //.with_bindings_from_file(binding_path)?;

    // Spawn World
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<MovementBindingTypes>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                .with_clear([0.0, 0.0, 0.0]),
                //.with_clear([255.0, 255.0, 255.0, 0.0]),
            )
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default()) 
        )?;

    let assets_dir = app_root.join("assets");
    // with_resource(blocks_config).
    let mut game = Application::build(assets_dir, LoadingState::default())?
        .with_resource(blocks_config)
        .with_resource(BlockData::new())
        .with_resource(Blockage::default())
        .with_resource(PhysicsQueue::default())
        .with_resource(InputCache::default())
        .with_resource(GameEvent::Normal)
        .build(game_data)?;
    //let mut game = Application::new(assets_dir, LoadingState::default(), game_data)?;
    game.run();

    Ok(())
}
