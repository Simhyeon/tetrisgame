use amethyst::{
    core::timing::Time,
    core::transform::{Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadStorage, WriteExpect, Write, Read, World, LazyUpdate},
    shrev::{EventChannel, ReaderId},
    shred::PanicHandler,
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};
use crate::component::stt_block::StaticBlock;
use crate::world::{
    key_int::KeyInt,
    gravity_status::GravityStatus,
    block_data::BlockData,
    stack_status::StackStatus,
    collapse_status::CollapseStatus,
};
use crate::events::GameEvent;

const STACKDELAY: f32 = 0.3;
const SHOOTDFRAME: usize = 2;
const KEYINTDELAY: f32 = 0.05;

#[derive(Debug)]
pub enum StackEvent {
    Stacked,
    ToBeStacked,
    Free,
    IgnoreDelay,
}

#[derive(SystemDesc)]
pub struct StackSystem {
    stack_delay: f32,
    shoot_delay_frame: usize,
    reader_id : ReaderId<StackEvent>,
}

impl StackSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<StackEvent>>().register_reader();
        Self {  
            stack_delay: STACKDELAY,
            shoot_delay_frame : SHOOTDFRAME,
            reader_id,
        }
    }
}

// Simple arrangements before refactoring Codes
// StackSystem first checks if dynamic blocsk are in the state to be stacked. which is one block
// upward ground or static blocks.
// If stacked stated are detected send to_be_stacked event to channel
// Gravity read the event and stop gravity logic.
// After stack delay has passed stack system really stack the blocks and send
// stacked event to channel
//
// And what I should refactor is about chaning event logics so that  
// to_be_stacked event can be reverted when stack is not detected.
// which also should retrigger gravity system 
impl<'s> System<'s> for StackSystem {
    type SystemData = (
        WriteExpect<'s, DynBlockHandler>,
        ReadStorage<'s, DynamicBlock>,
        WriteStorage<'s, StaticBlock>,
        WriteStorage<'s, Transform>,
        Write<'s, EventChannel<StackEvent>>,
        Write<'s, EventChannel<KeyInt>>,
        Read<'s, Time>,
        WriteExpect<'s, BlockData>,
        WriteExpect<'s, GameEvent>,
        Read<'s, LazyUpdate>,
        WriteExpect<'s, GravityStatus>,
        WriteExpect<'s, StackStatus>,
        WriteExpect<'s, CollapseStatus>,
        WriteExpect<'s, KeyInt>,
    );

    // TODO Change to_be_stacked value as some kind of trigger
    // TODO THIS CODE IS SHIT FUCK ME
    fn run(&mut self, (mut handler, dyn_blocks, 
            mut stt_blocks, 
            mut locals, 
            mut stack_event, 
            mut write_key_event, 
            time, 
            mut block_data, 
            mut game_event,
            updater,
            mut gravity_status,
            mut stack_status,
            mut collapse_status,
            mut key_int
    ): Self::SystemData) {

        if handler.blocks.len() == 0 {
            return;
        }

        match *stack_status {
            StackStatus::Stacked | StackStatus::ShootStack => {

                //println!("Stacking Call ");
                if let StackStatus::ShootStack = *stack_status {
                    if self.shoot_delay_frame != 0 {
                        self.shoot_delay_frame -=1;
                        return;
                    } else {
                        self.shoot_delay_frame = SHOOTDFRAME;
                    }
                }
                // TODO Currently parent entity is not removed while entity is very resource light and
                // doesn't get calculated at all so this is not that bad
                // However memory is getting leaked definitely


                // TODO
                // Currently something worng... But I dont know
                let mut soundness: bool = true; // Assume the blocks are sound.
                for entity in &handler.blocks {
                    let local_matrix = locals.get(*entity).unwrap().global_matrix();

                    // When y value is over height of screen which means, in this case,
                    // game over
                    if local_matrix.m24 >= 900.0  {
                        *game_event = GameEvent::GameOver;
                        return;
                    }

                    if local_matrix.m24.round() <= 0.0 {
                        locals.get_mut(handler.parent.unwrap()).unwrap().append_translation_xyz(0.0, 45.0, 0.0);
                        return;
                    }

                    if block_data.find_block(local_matrix.m14, local_matrix.m24) {
                        // Now we have to recalibrate the blocks
                        //println!("Checking soundness of block duplication ---");
                        //println!("While it is ({}, {})", local_matrix.m14, local_matrix.m24);
                        soundness = false;
                        break;
                    }
                }
                if !soundness {
                    let parent_entity = handler.parent.unwrap();
                    let current = locals.get(parent_entity).unwrap().global_matrix().m24;
                    locals.get_mut(parent_entity).unwrap().set_translation_y(current + 45.0);
                    return;
                }

                // Now stack the blocks
                for entity in &handler.blocks {

                    stt_blocks.insert(*entity, StaticBlock).expect("ERR");
                    // Add block to block_data
                    let matrix_m = locals.get(*entity).unwrap().global_matrix();
                    match block_data.add_block(matrix_m.m14.round(), matrix_m.m24.round(), entity.clone()) {
                        Ok(_) => (),
                        Err(_) => {
                            *game_event = GameEvent::GameOver;
                        }
                    }
                }

                // Reset variables
                handler.blocks.clear();
                handler.parent.take();
                self.stack_delay = STACKDELAY;
                *stack_status = StackStatus::None;
                *key_int = KeyInt::None;
                *gravity_status = GravityStatus::On;
                *collapse_status = CollapseStatus::Triggered;
                //println!("Stacked!");
            }
            StackStatus::TobeStacked | StackStatus::None => {
                if let StackStatus::TobeStacked = *stack_status {
                    self.stack_delay -= time.delta_seconds();
                    if self.stack_delay <= 0.0 {
                        //println!("Execute stack");
                        *stack_status = StackStatus::Stacked;
                        return;
                    } else if self.stack_delay <= KEYINTDELAY {
                        *key_int = KeyInt::Stack;
                    }
                }

                'outer :for (dyn_local, _, ()) in (&locals, &dyn_blocks, !&stt_blocks).join() {
                    if dyn_local.global_matrix().m24.round() == 45.0 { // this is when to be stacked
                        //println!("To be Stacked");
                        *stack_status = StackStatus::TobeStacked;
                        *gravity_status = GravityStatus::Off;
                        break 'outer;
                    }

                    for (local, _) in (&locals, &stt_blocks).join() {
                        if local.global_matrix().m24.round() == dyn_local.global_matrix().m24.round() - 45.0 
                            && local.global_matrix().m14.round() == dyn_local.global_matrix().m14.round() {
                                //println!("To be Stacked");
                                *stack_status = StackStatus::TobeStacked;
                                *gravity_status = GravityStatus::Off;
                                break 'outer;
                            }
                    }

                    // No break has been called which means Free state
                    if let StackStatus::TobeStacked = *stack_status {
                        //println!("Free from to be stacked");
                        *stack_status = StackStatus::Free;
                    }
                }
            }
            StackStatus::Free => {
                //println!("Free stack event");
                self.stack_delay = STACKDELAY;
                *gravity_status = GravityStatus::On;
                *stack_status = StackStatus::None;
                *key_int = KeyInt::None;
                return;
            }
        }
    }
}
