use amethyst::{
    assets::Handle,
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage, WriteExpect, LazyUpdate, Write, ReadExpect, Entity, Entities},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},

};

pub const WIDTH: f32 = 450.0;
pub const HEIGHT: f32 = 900.0;
use crate::component::dyn_block::{DynBlockHandler, DynamicBlock};

#[derive(SystemDesc, Default)]
pub struct SpawnerSystem;



impl<'s> System<'s> for SpawnerSystem{
    type SystemData = (
        Entities<'s>,
        WriteExpect<'s, DynBlockHandler>,
        Read<'s, LazyUpdate>,
        ReadExpect<'s, Handle<SpriteSheet>>,
    );

    fn run(&mut self, (entities, mut handler, updater, sprite_sheet_handle): Self::SystemData){
        if handler.blocks.len() == 0 {
            println!("SPawning");
            // Transform setup
            let mut block_transforms = vec![Transform::default(); 4];
            let mut yoffset = 45.0; // which is the size of block

            for item in &mut block_transforms {
                item.set_translation_xyz(WIDTH/ 2.0, HEIGHT - yoffset, 0.0);
                yoffset += 45.0;
            }

            // SpriteSheet setup
            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle.clone(),
                sprite_number: 0,
            };

            for item in block_transforms {
                let new_block = entities.create();
                // Transform
                updater.insert(
                    new_block,
                    item,
                );
                //DynamicBlock
                updater.insert(
                    new_block,
                    DynamicBlock,
                );
                // Sprite Texture
                updater.insert(
                    new_block,
                    sprite_render.clone(),
                );
                handler.blocks.push(new_block);
            }
        }
    }


}
