use crate::commons::Rotation;

pub enum AxisType {
    Right,
    Left,
    Down,
    None,
}

pub struct InputCache {
    axis : AxisType, 
    rotation : Rotation,
    shoot : bool,
}
