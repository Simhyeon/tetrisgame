use crate::commons::Rotation;

pub enum AxisType {
    Right,
    Left,
    Down,
    None,
}

pub struct InputCache {
    pub axis : AxisType, 
    pub rotation : Rotation,
    pub shoot : bool,
}
