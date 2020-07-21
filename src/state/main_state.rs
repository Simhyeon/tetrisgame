use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    assets::{AssetStorage, Loader, Handle},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    prelude::*,
};

use crate::component::dyn_block;

const SPAWNTIME: f32 = 1.5;

#[derive(Default)]
pub struct MainState{
    spawn_delay: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for MainState{
    fn on_start(&mut self, data:StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.spawn_delay.replace(0.0);
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        // Initialize everythings.
        initialize_camera(world);
        print_sprite(world, self.sprite_sheet_handle.clone().unwrap());
    }

//    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
//        if let Some(mut timer) = self.spawn_delay.take() {
//            {
//                let time = data.world.fetch::<Time>();
//                timer += time.delta_seconds();
//            }
//            if timer >= SPAWNTIME {
//                println!("SPAWN");
//                print_sprite(data.world, self.sprite_sheet_handle.clone().unwrap());
//            } else {
//                self.spawn_delay.replace(timer);
//            }
//        }
//        Trans::None
//    }
}

pub const WIDTH: f32 = 450.0;
pub const HEIGHT: f32 = 900.0;

fn initialize_camera(world: &mut World){
    let mut transform = Transform::default();
    transform.set_translation_xyz(WIDTH / 2.0, HEIGHT / 2.0, 1.0);

    world.create_entity()
        .with(Camera::standard_2d(WIDTH, HEIGHT))
        .with(transform)
        .build();
}

fn print_sprite(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>){

    // Transform setup
    let mut block_transforms = vec![Transform::default(); 4];
    let mut yoffset = 45.0; // which is the size of block

    for item in &mut block_transforms {
        item.set_translation_xyz(WIDTH/ 2.0, HEIGHT - yoffset, 0.0);
        yoffset += 45.0;
    }

    // SpriteSheet setup
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    let mut dyn_handler = dyn_block::DynBlockHandler{blocks: vec![]};

    for item in block_transforms {
        dyn_handler.blocks.push(
            world
                .create_entity()
                .with(item)
                .with(sprite_render.clone())
                .with(dyn_block::DynamicBlock)
                .build()
        );
    }
    world.insert(dyn_handler);
}

fn load_sprite_sheet(world : &mut World) -> Handle<SpriteSheet>{
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/block.png",
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
