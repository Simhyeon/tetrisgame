use amethyst::{
    prelude::*,
    ecs::{Entity, World},
};
// hread '<unnamed>' panicked at 'index out of bounds: the len is 20 but the index is 20', src/world/block_data.rs:119:23
use std::fmt;
use std::fmt::{Display, Formatter};

const BLOCK_HEIGHT: usize = 20;
const BLOCK_WIDTH: usize = 10;

pub struct BlockData {
    data_length: Vec<usize>,
    pub data: Vec<Vec<Option<Entity>>>, // Debug for pub
}

impl Display for BlockData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for item in self.data.iter().rev() {
            for sub_item in item.iter() {
                match sub_item {
                    Some(_) => {
                        write!(f, "ㅁ")?;
                    },
                    None => {
                        write!(f, "ㅡ")?;
                    }
                }
            }
            write!(f, "\n")?;
        }
        //for (index, item) in self.data_length.iter().rev().enumerate() {
            //write!(f, "{}th\t->\t{}\n", index, item)?;
        //}
        write!(f, "")
    }
}

impl BlockData {
    pub fn new() -> Self {
        let mut data_length: Vec<usize> = vec![];
        for _ in 0..BLOCK_HEIGHT {
            data_length.push(0);
        }

        let mut data = vec![];

        for _ in 0..BLOCK_HEIGHT {
            let mut row: Vec<Option<Entity>> = vec![];
            for _ in 0..BLOCK_WIDTH {
                row.push(None);
            }
            data.push(row);
        }

        Self {  
            data_length,
            data,
        }
    }

    // Indexing functions 
    pub fn get_col_index_from_m24(matrix_m24: f32) -> usize {
        (matrix_m24 / 45.0) as usize - 1
    }

    pub fn get_row_index_from_m14(matrix_m14: f32) -> usize {
        ((matrix_m14 + 45.0) / 45.0) as usize - 1
    }

    pub fn check_full(&self, matrix_m24: f32) -> bool {
        let index = Self::get_col_index_from_m24(matrix_m24);
        if self.data_length[index] == BLOCK_WIDTH {
            true
        } else {
            false
        }
    }

    pub fn get_top_block(&self, x_value: f32, y_value: f32) -> Option<Entity>{
        let index = Self::get_row_index_from_m14(x_value);
        let col_index = Self::get_col_index_from_m24(y_value);
        let mut ent: Option<Entity> = None;
        for col in 0..col_index {
            if let Some(entity) = self.data[col][index] {
                println!("Calculaing get_top_block X: {}, y: {}", index, col);
                ent.replace(entity);
            }
        }

        ent
    }

    pub fn remove_lows(&mut self, matrix_m24: f32) -> Vec<Option<Entity>> {
        let index = Self::get_col_index_from_m24(matrix_m24);
        let mut upper_vec: Vec<Option<Entity>> = vec![];
        for count in index+1..BLOCK_HEIGHT {
            // Move count's vector to lower count's vector
            // which is technically same as some kind of bubble sort
            // Is "Cloning" a good idea? I dont' know (Mabye not good)
            // but it works. TODO check this line if you want efficient codes
            self.data[count-1].clear();
            self.data[count-1] = self.data[count].clone();
            self.data_length[count-1] = self.data_length[count];
            upper_vec.extend(&self.data[count]);
        }
        // Clear top most items which is not cleared by moving items
        for item in self.data[BLOCK_HEIGHT-1].iter_mut() {
            item.take();
        }
        self.data_length[BLOCK_HEIGHT-1] = 0;

        upper_vec
    }
    
    pub fn find_block(&self, matrix_m14:f32, matrix_m24: f32) -> bool {
        let col_index = Self::get_col_index_from_m24(matrix_m24);
        let row_index = Self::get_row_index_from_m14(matrix_m14);
        if let None = self.data[col_index][row_index] {
            false
        } else {
            true
        } 
    }

    pub fn add_block(&mut self, matrix_m14:f32, matrix_m24: f32, entity: Entity) -> Result<(), &str> {
        let col_index = Self::get_col_index_from_m24(matrix_m24);
        let row_index = Self::get_row_index_from_m14(matrix_m14);

        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("<ADD BLOCK>");
        println!("Adding to ({}, {})", col_index, row_index);
        if let None = self.data[col_index][row_index] {
            self.data[col_index][row_index].replace(entity);
            self.data_length[col_index] += 1;
        } else {
            println!("Already existing in ... ");
            println!(" X :{} Y : {}", row_index, col_index);
            return Err("Game Over ... OR Hardly detected problem occured.");
        } 

        Ok(())
    }

    // Keep in mind that m14, or x value can be 0 
    // while m24, or y value must be either 45 or bigger.
    pub fn get_row(&self, matrix_m24: f32) -> Vec<Option<Entity>> {
        let index = Self::get_col_index_from_m24(matrix_m24);
        self.data[index].clone()
    }

    pub fn clear_row(&mut self, matrix_m24: f32) {
        let index = Self::get_col_index_from_m24(matrix_m24);
        for item in self.data[index].iter_mut() {
            *item = None;
        }
        self.data_length[index] = 0;
    }
} 
