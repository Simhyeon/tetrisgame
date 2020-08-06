use std::fmt::{self, Display};

use amethyst::input::BindingTypes;
    
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {
    Shoot,
    RotateRight,
    RotateLeft,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct BlocksConfig {
    pub blocks: Vec<Block>,
}

impl BlocksConfig {
    pub fn new() -> Self {
        Self { 
            blocks: vec![Block::default()],
        }
    }
}

#[derive(Clone, Default,Debug, Deserialize, Serialize)]
pub struct Block {
    //pub locations: Vec<(f32, f32)>,
    pub origin: u32,
    pub offset : Offset,
}

#[derive(Clone, Default,Debug, Deserialize, Serialize)]
pub struct Offset {
    pub right_rotate: (f32, f32),
    pub left_rotate: (f32, f32),
}

//origin: 2,
//offset: (
    //right_rotate: (-2, 1)
    //left_rotate: (-1, 2)
//),


impl Display for AxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for ActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct MovementBindingTypes;

impl BindingTypes for MovementBindingTypes {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}
