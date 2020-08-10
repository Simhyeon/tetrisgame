use std::fmt::{self, Display};

use amethyst::assets::Handle;
use amethyst::input::BindingTypes;
use amethyst::renderer::SpriteSheet;
    
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
    Debug,
    NoGrav,
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
    pub locations: Vec<(f32, f32)>,
    pub origin: u32,
    pub offset : Option<Offset>,
    pub sub_offset : Option<Offset>
}

#[derive(Clone, Default,Debug, Deserialize, Serialize, Copy)]
pub struct Offset {
    pub right_rotate: (f32, f32),
    pub left_rotate: (f32, f32),
}

pub struct PaneSpriteHandle {
    pub sprite_sheet : Handle<SpriteSheet>
}

impl PaneSpriteHandle{
    pub fn new(sprite_sheet : Handle<SpriteSheet>) -> Self {
        Self {  
            sprite_sheet,
        }
    }
}

#[derive(Clone, Default,Debug, Deserialize, Serialize)]
pub struct PaneConfig {
    pub panes : Vec<Pane>,
}

#[derive(Clone, Default,Debug, Deserialize, Serialize)]
pub struct Pane {
    pub name : String,
    pub bottom_left : (f32, f32),
    pub top_right : (f32, f32),
}

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
