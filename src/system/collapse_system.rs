use amethyst::{
    core::transform::{Transform, Parent},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage, WriteExpect, LazyUpdate, Write, ReadExpect, Entity, Entities},
    shrev::{ReaderId, EventChannel},
    ui::UiText,
};

use crate::component::stt_block::StaticBlock;
use crate::world::{block_data::BlockData, score_text::ScoreText, collapse_status::CollapseStatus};
use crate::utils;

#[derive(Default, SystemDesc)]
pub struct CollapseSystem;

impl<'s> System<'s> for CollapseSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Parent>,
        ReadStorage<'s, StaticBlock>,
        WriteExpect<'s, BlockData>,
        WriteExpect<'s, ScoreText>,
        WriteExpect<'s, CollapseStatus>,
        WriteStorage<'s, UiText>
    );

    fn run(&mut self, (entities, mut locals, parents, stt_blocks, mut block_data, mut score_text, mut collapse_status,mut ui_text) : Self::SystemData) {
        if let CollapseStatus::Triggered = *collapse_status {
            // Collapse logic
            'outer : loop {
                'inner : for index in 0..20 {
                    let col_index = (index +1) as f32 * 45.0;
                    //println!("Checking fullness of index : {}  restul : {}", index , block_data.check_full(col_index));
                    if block_data.check_full(col_index)  {

                        // Delete entity values that entity vector contains not entity itself
                        // acutally entity itsefl is not a value rather an indicator.
                        let block_entities = block_data.get_row(col_index);
                        for entity in block_entities {
                            // Unwrap should not fail because data_length is full.
                            entities.delete(entity.unwrap()).expect("Failed to delete entity");
                        }

                        // Remove collaped row and move all uppers rows down by 1 row.
                        // And get merged entity vector and use the vector to really move value
                        // downward
                        let to_be_moved = block_data.remove_lows(col_index);
                        for item in to_be_moved {
                            if let Some(entity) = item {
                                let parent_entity = parents.get(entity).unwrap().entity;
                                let (x, y, z) = utils::get_y_absolute_move(locals.get(parent_entity).unwrap().euler_angles(), -45.0);
                                locals.get_mut(entity).unwrap().append_translation_xyz(x, y, z);
                            }
                        }

                        // This is hard code af but I could know how to find a specific text
                        // easily... 
                        for ui in (&mut ui_text).join() {
                            score_text.add_score(1000);
                            ui.text = score_text.score_text.clone();
                        }

                        // Break out of "For index in 0..20 loop" which is inner loop
                        // But stay in outer loop to check from start
                        continue 'outer;
                    }
                }
                // Break out of outer loop if no col_index is detected;
                *collapse_status = CollapseStatus::None;
                break 'outer;
            }
        }
    }
}
