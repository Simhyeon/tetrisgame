use amethyst::{
    //core::timing::Time,
    core::transform::Transform,
    assets::{AssetStorage, Loader, Handle},
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    prelude::*,
    ecs::World,
};

use crate::state::main_state::MainState;
use crate::component::dyn_block::DynBlockHandler;

#[derive(Default)]
pub struct LoadingState{
    sprite_sheet_store: Option<Handle<SpriteSheet>>,
}

impl SimpleState for LoadingState{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
        let world = data.world;

        // Initialize everythings.
        let sprite_sheet_store = load_sprite_sheet(world);
        self.sprite_sheet_store = Some(sprite_sheet_store.clone());
        world.insert(sprite_sheet_store);

        initialize_camera(world);
        world.insert(DynBlockHandler::default());
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) ->SimpleTrans{
        if self.sprite_sheet_store != None{ 
            println!("Load Complete");
            return Trans::Replace(Box::new(MainState::default()));
        }
        Trans::None
    }
}

pub const WIDTH: f32 = 450.0;
pub const HEIGHT: f32 = 900.0;

fn initialize_camera(world: &mut World){
    let mut transform = Transform::default();
    transform.set_translation_xyz(WIDTH / 2.0 - 22.5, HEIGHT / 2.0 + 22.5, 1.0);

    world.create_entity()
        .with(Camera::standard_2d(WIDTH, HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world : &mut World) -> Handle<SpriteSheet>{
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/blocks.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/block_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

