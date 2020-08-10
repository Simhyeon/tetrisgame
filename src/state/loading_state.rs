use amethyst::{
    //core::timing::Time,
    core::transform::Transform,
    assets::{AssetStorage, Loader, Handle},
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture, SpriteRender},
    prelude::*,
    utils::application_root_dir,
    ui::{ Anchor, UiButtonBuilder, },
};

use std::{thread, time};
use crate::state::main_state::MainState;
use crate::component::dyn_block::DynBlockHandler;
use crate::config::{PaneConfig, Pane};
use crate::world::score_text::ScoreText;

#[derive(Default)]
pub struct LoadingState{
    sprite_sheet_store: Option<Handle<SpriteSheet>>,
}

impl SimpleState for LoadingState{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>){
        let world = data.world;

        let app_root = application_root_dir().expect("Failed to oepn application root");
        let pane_config = app_root.join("config").join("display_pane.ron");
        let pane_config = PaneConfig::load(&pane_config).unwrap();

        let panes : Vec<Pane> = pane_config.panes.clone();

        let (width, height) = panes[1].top_right;

        world.insert(pane_config);

        // Initialize everythings.
        // Spawn Pane sprites
        let pane_sprite_sheet = load_pane_sprite_sheet(world);
        let sprite_render = SpriteRender {
            sprite_sheet: pane_sprite_sheet.clone(),
            sprite_number: 0,
        };

        // Build entity with sprite render
        let mut transform = Transform::default();
        transform.set_translation_xyz( width / 2.0 - 22.5, height / 2.0 + 22.5, -1.0);
        world.entities()
            .build_entity()
            .with(sprite_render.clone(), &mut world.write_storage())
            .with(transform, &mut world.write_storage())
            .build();

        UiButtonBuilder::<(), u32>::new("000000".to_string())
            .with_font_size(30.0)
            .with_position(-75.0, -75.0)
            .with_size(64.0 * 6.0, 64.0)
            .with_anchor(Anchor::TopRight)
            .with_text_color([255.0,255.0,255.0, 1.0])
            //.with_image(UiImage::SolidColor([0.8, 0.6, 0.3, 1.0]))
            //.with_hover_image(UiImage::SolidColor([0.1, 0.1, 0.1, 0.5]))
            .build_from_world(&world);

        let score_text = ScoreText::new();
        world.insert(score_text);

        // Get block sprites
        let sprite_sheet_store = load_block_sprite_sheet(world);
        self.sprite_sheet_store = Some(sprite_sheet_store.clone());
        world.insert(sprite_sheet_store);

        // INitialize camera literally...
        initialize_camera(world, width, height);

        // Insert DynmaicBlockHandler which is used to handle
        // hot loaded entities of dynamic blocks.
        world.insert(DynBlockHandler::default());
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) ->SimpleTrans{
        if self.sprite_sheet_store != None{ 
            thread::sleep(time::Duration::from_millis(500));
            println!("Load Complete");
            return Trans::Replace(Box::new(MainState::default()));
        }
        Trans::None
    }
}

fn initialize_camera(world: &mut World, width :f32, height: f32){
    let mut transform = Transform::default();
    transform.set_translation_xyz( width / 2.0 - 22.5, height / 2.0 + 22.5, 1.0);

    world.create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}

fn load_block_sprite_sheet(world : &mut World) -> Handle<SpriteSheet>{
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

fn load_pane_sprite_sheet(world : &mut World) -> Handle<SpriteSheet>{
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/panes.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pane_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
