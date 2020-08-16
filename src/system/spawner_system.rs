use amethyst::{
    assets::Handle,
    core::transform::{Transform, Parent},
    derive::SystemDesc,
    ecs::prelude::{Read, System, SystemData, WriteExpect, LazyUpdate, ReadExpect, Entities, Entity, WriteStorage},
    renderer::{SpriteRender, SpriteSheet},

};
use rand::prelude::*;

pub const WIDTH: f32 = 450.0;
pub const HEIGHT: f32 = 900.0;

use crate::component::dyn_block::{DynBlockHandler, DynamicBlock};
use crate::commons::Rotation;
use crate::config::BlocksConfig;

#[derive(SystemDesc, Default)]
pub struct SpawnerSystem {
    next_index: usize,
    next_parent_entity: Option<Entity>,
}

// TODO 현재 Spanwer는 블록이 겹칠 경우를 상정하지 않고 있다. 구현되어야 한다.

impl<'s> System<'s> for SpawnerSystem{
    type SystemData = (
        Entities<'s>,
        WriteExpect<'s, DynBlockHandler>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, DynamicBlock>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Parent>,
        ReadExpect<'s, Handle<SpriteSheet>>,
        Read<'s, BlocksConfig>,
    );

    fn run(&mut self, (
            entities, 
            mut handler, 
            mut locals,
            mut dyn_blocks, 
            mut renders,
            mut parents,
            sprite_sheet_handle, 
            block_config
    ): Self::SystemData){
        if handler.blocks.len() == 0 {
            //println!("SPawning");

            // TODO I'm not sure but how does next block is implemented? I can't remember well... 
            let mut rng = thread_rng();
            let block_index: usize;

            // Remove prior next transform
            if let Some(entity) = self.next_parent_entity { // This is not the first time.
                entities.delete(entity).expect("\nFailed to delete next_parent_entity\n");
                block_index = self.next_index;
            } else { // This is the first time 
                block_index = rng.gen_range(0, 7);
            }
            self.next_index = rng.gen_range(0, 7);

            // Get Config 
            let local_config = &block_config.blocks[block_index];
            let next_local_config = &block_config.blocks[self.next_index];

            // Transform setup
            let mut block_transforms = vec![Transform::default(); 4];
            let mut next_block_transforms = vec![Transform::default(); 4];

            // Backup Codes  SHould Delete after other translation succeeds
            //let mut yoffset = -90.0; // which is the size of block

            //for item in &mut block_transforms {
                //item.set_translation_xyz(0.0, 0.0 + yoffset, 0.0);
                //yoffset += 45.0;
            //}

            // Set children's transform according to given offsets of local_config
            for (index, (x_offset, y_offset)) in local_config.locations.iter().enumerate() {
                block_transforms[index].set_translation_xyz(0.0 + x_offset, 0.0 + y_offset, 0.0);
            }

            for (index, (x_offset, y_offset)) in next_local_config.locations.iter().enumerate() {
                next_block_transforms[index].set_translation_xyz(0.0 + x_offset, 0.0 + y_offset, 0.0);
            }

            // SpriteSheet setup
            // Geter First sprite from spritesheet
            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle.clone(),
                sprite_number: block_index,
            };

            let next_sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle.clone(),
                sprite_number: self.next_index,
            };

            // Set Parent with origin index which is read from local_config
            let parent = entities.create();
            let mut parent_pos = Transform::default();
            let origin_index = local_config.origin;

            let next_parent = entities.create();
            let mut next_parent_pos = Transform::default();
            let next_origin_index = local_config.origin;

            // Set parent transform to that of origin transform's. 
            parent_pos = block_transforms[origin_index as usize].clone();
            parent_pos.append_translation_xyz(WIDTH - 45.0 * 5.0, HEIGHT - 45.0 * 2.0, 0.0);

            next_parent_pos = block_transforms[next_origin_index as usize].clone();
            next_parent_pos.append_translation_xyz(505.0 , HEIGHT - 45.0 * 5.0, 0.0);

            // Update entity with new transform component
            locals.insert(
                parent,
                parent_pos,
            ).expect("Faeild to add parent");

            locals.insert(
                next_parent,
                next_parent_pos,
            ).expect("");
            self.next_parent_entity.replace(next_parent);

            // Set required informations to dynamic block handlers
            handler.parent = Some(parent);
            handler.rotation = Rotation::Up;
            handler.config = local_config.clone();

            // Spawn child blocks and attach to parent transform
            for item in block_transforms {
                let new_block = entities.create();
                // Transform
                locals.insert(
                    new_block,
                    item,
                ).expect("");
                //DynamicBlock
                dyn_blocks.insert(
                    new_block,
                    DynamicBlock,
                ).expect("");
                // Sprite Texture
                renders.insert(
                    new_block,
                    sprite_render.clone(),
                ).expect("");

                parents.insert(
                    new_block,
                    Parent::new(parent),
                ).expect("");

                handler.blocks.push(new_block);
            }

            // For next blocks
            for item in next_block_transforms {
                let next_new_block = entities.create();
                // Transform
                locals.insert(
                    next_new_block,
                    item,
                ).expect("");
                // Sprite Texture
                renders.insert(
                    next_new_block,
                    next_sprite_render.clone(),
                ).expect("");

                parents.insert(
                    next_new_block,
                    Parent::new(next_parent),
                ).expect("");
            }
        }
    }


}
