use crate::commons::Rotation;

#[derive(Debug)]
pub enum AxisType {
    Right,
    Left,
    Down,
    None,
}

#[derive(Debug)]
pub struct InputCache {
    pub axis : AxisType, 
    pub rotation : Rotation,
    pub shoot : bool,
}

impl Default for InputCache {
    fn default() -> Self {
        Self {
            axis: AxisType::None,
            rotation : Rotation::None,
            shoot: false,
        }
    }
}

impl InputCache {
    pub fn update_input(&mut self, horizontal: f32, vertical: f32, right_rotate: bool, left_rotate: bool, shoot: bool) {

        //Every Inputs are mutually exclsive
        if horizontal > 0.0 {
            self.axis = AxisType::Right;
        } else if horizontal < 0.0 {
            self.axis = AxisType::Left;
        } else if vertical < 0.0 {
            self.axis = AxisType::Down;
        } else  if right_rotate {
            self.rotation = Rotation::Right;
        } else if left_rotate {
            self.rotation = Rotation::Left;
        } else if shoot {
            self.shoot = true;
        }
    }

    pub fn clear(&mut self) {
        self.axis = AxisType::None;
        self.rotation = Rotation::None;
        self.shoot = false;
    }
}
