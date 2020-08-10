use amethyst::{
    core::timing::Time,
    core::transform::{Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadStorage, WriteExpect, Write, Read, World, LazyUpdate},
    shrev::{EventChannel, ReaderId},
};

use crate::component::dyn_block::{DynamicBlock, DynBlockHandler};
use crate::component::stt_block::StaticBlock;
use crate::system::keyinput_system::KeyInt;
use crate::world::{
    gravity_status::GravityStatus,
    block_data::BlockData
};
use crate::events::GameEvent;

const STACKDELAY: f32 = 0.3;
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
    to_be_stacked: bool,
    stack_delay: f32,
    no_delay: bool,
    reader_id : ReaderId<StackEvent>,
}

impl StackSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<StackEvent>>().register_reader();
        Self {  
            to_be_stacked : false,
            stack_delay: STACKDELAY,
            no_delay: false,
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
            mut gravity_status
    ): Self::SystemData) {
        if handler.blocks.len() == 0 {
            return;
        }

        let mut stack_confirm = false;

        for event in stack_event.read(&mut self.reader_id) {
            match event {
                StackEvent::IgnoreDelay => {
                    println!("----------");
                    println!("IGNOREING DELAY");
                    println!("----------");
                    self.no_delay = true;
                    *gravity_status = GravityStatus::Off;
                    //self.to_be_stacked = false;
                },
                _ => (),
            }
        } 

        if self.to_be_stacked && !stack_confirm {
            //Wait for certain times and 
            self.stack_delay -= time.delta_seconds();
            if self.stack_delay <= 0.0 || self.no_delay{
                stack_confirm = true;
            } else if self.stack_delay <= KEYINTDELAY {
                // This else if statement is to prevent from user to give input at the same time
                // block is stacked. Which makes block stacked on air. Which is not desired
                // actions.
                // However this method is not good at all. Since sending key event is based on
                // delta time which is time between continous function calls. and such delta
                // time can be different among other devices and enviorments.
                // For example when os failed to allocate enough resource for ths program
                // then this functionality might fail. 
                // (While it is also highly expected to fail to get user input anyway.)
                write_key_event.single_write(KeyInt::Stack);
            }
        }

        let mut to_free : bool = true;
        if !stack_confirm {
            'outer :for (dyn_local, _, ()) in (&locals, &dyn_blocks, !&stt_blocks).join() {
                if dyn_local.global_matrix().m24.round() == 45.0 { // this is when to be stacked
                    if !self.to_be_stacked {
                        self.to_be_stacked = true;
                        println!("TOBESTACKED for walls");
                        stack_event.single_write(StackEvent::ToBeStacked);
                        *gravity_status = GravityStatus::Off;
                    }
                    to_free = false;
                    break 'outer;
                }

                for (local, _) in (&locals, &stt_blocks).join() {
                    if local.global_matrix().m24.round() == dyn_local.global_matrix().m24.round() - 45.0 
                        && local.global_matrix().m14.round() == dyn_local.global_matrix().m14.round() {
                            if !self.to_be_stacked {
                                self.to_be_stacked = true;
                                println!("TOBESTACKED for blocks");
                                stack_event.single_write(StackEvent::ToBeStacked);
                                *gravity_status = GravityStatus::Off;
                            }
                            to_free = false;
                            break 'outer;
                        }
                }
            }

            //// Another calibration... feels bad but it's reality
            //if self.no_delay && !self.to_be_stacked {
                //locals.get_mut(handler.parent.unwrap()).unwrap().prepend_translation_y(-45.0);
            //}

        } else {

            // TODO Currently parent entity is not removed while entity is very resource light and
            // doesn't get calculated at all so this is not that bad
            // However memory is getting leaked definitely

            // Reset variables
            self.stack_delay = STACKDELAY;
            self.to_be_stacked = false;
            self.no_delay = false;

            // Calibrate undefined duplication
            let mut do_calibrate = false;
            'cal :for entity in &handler.blocks { 
                let entity_matrix = locals.get(*entity).unwrap().global_matrix();
                for (local, _, _) in (&locals, &dyn_blocks, &stt_blocks).join() {
                    if local.global_matrix() == entity_matrix {
                        do_calibrate = true;
                        break 'cal;
                    }
                }
            }

            if do_calibrate {
                let current = locals.get_mut(handler.parent.unwrap()).unwrap().global_matrix().m24.round();
                locals.get_mut(handler.parent.unwrap()).unwrap().set_translation_y(current + 45.0);
                println!("\\\\\\\\\\\\\\\\\\");
                println!("\\\\\\\\\\\\\\\\\\");
                println!("\\\\\\\\\\\\\\\\\\");
                println!("\\\\\\\\\\\\\\\\\\");
                println!("\\\\\\\\\\\\\\\\\\");
                println!("\\\\\\\\\\\\\\\\\\");
                println!("\\\\\\\\\\\\\\\\\\");
                println!("Calibrated");
            }
        
            // Now stack the blocks
            for entity in &handler.blocks {

                stt_blocks.insert(*entity, StaticBlock).expect("ERR");
                // Add block to block_data
                let matrix_m = locals.get(*entity).unwrap().global_matrix();
                println!(" Stacking with X :{}, Y : {}", (matrix_m.m14.round() + 45.0 / 45.0).round() - 1.0, (matrix_m.m24.round() / 45.0).round());
                println!("Real Value is X :{}, Y :{}", matrix_m.m14.round(), matrix_m.m24.round());
                match block_data.add_block(matrix_m.m14.round(), matrix_m.m24.round(), entity.clone()) {
                    Ok(_) => (),
                    Err(_) => {
                        // This is for debuggin purpose since adding is not yet compelte... in
                        // terms of bug free.
                        println!("{}", *block_data);
                        // Send game over event channel
                        *game_event = GameEvent::GameOver;
                    }
                }
            }
            handler.blocks.clear();
            stack_event.single_write(StackEvent::Stacked);
            write_key_event.single_write(KeyInt::None);
            *gravity_status = GravityStatus::On;
            println!("Stacked!");
        }

        // if gravtiy free condition has been met and also 
        // to_be_stacked was already called which means stack system priorly
        // detected stack call  and now it is not detected.
        if self.to_be_stacked && to_free {
            println!("Free stack event");
            stack_event.single_write(StackEvent::Free);
            self.stack_delay = STACKDELAY;
            self.to_be_stacked = false;
            self.no_delay = false;
            *gravity_status = GravityStatus::On;
            return;
        }
    }
}
