use amethyst::{
    assets::Handle,
    core::transform::{Transform, Parent},
    derive::SystemDesc,
    ecs::prelude::{Read, System, SystemData, WriteExpect, LazyUpdate, ReadExpect, Entities},
    renderer::{SpriteRender, SpriteSheet},

};

pub const WIDTH: f32 = 450.0;
pub const HEIGHT: f32 = 900.0;
use crate::component::dyn_block::{DynBlockHandler, DynamicBlock};

#[derive(SystemDesc, Default)]
pub struct SpawnerSystem;


// TODO 현재 Spanwer는 블록이 겹칠 경우를 상정하지 않고 있다. 구현되어야 한다.

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
            let mut yoffset = 0.0; // which is the size of block

            //for item in &mut block_transforms {
                //item.set_translation_xyz(WIDTH - 45.0 * 5.0, HEIGHT - yoffset, 0.0);
                //yoffset += 45.0;
            //}

            for item in &mut block_transforms {
                item.set_translation_xyz(0.0, 0.0 - yoffset, 0.0);
                yoffset += 45.0;
            }

            // SpriteSheet setup
            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle.clone(),
                sprite_number: 0,
            };

            // Set Parent
            let parent = entities.create();
            let mut parent_pos = Transform::default();
            parent_pos.set_translation_xyz(0.0 + 45.0 * 2.0, HEIGHT - 45.0, 0.0);

            updater.insert(
                parent,
                parent_pos,
            );

            handler.parent = Some(parent);

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

                updater.insert(
                    new_block,
                    Parent::new(parent),
                );

                handler.blocks.push(new_block);
            }
            println!("{}", handler.blocks.len());
        }
    }


}
