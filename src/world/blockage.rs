#[derive(Default)]
pub struct Blockage {
    pub axis_right : bool,
    pub axis_left : bool,
    pub axis_down : bool,
    pub rotate_right : bool,
    pub rotate_left : bool,
    pub shoot: bool,
}

impl Blockage {
    pub fn clear (&mut self) {
        self.axis_right = false;
        self.axis_left = false;
        self.axis_down = false;
        self.rotate_right = false;
        self.rotate_left = false;
        self.shoot = false;
    }

    pub fn block_rotation(&mut self) {
        self.rotate_left = false;
        self.rotate_right = false;
    }
}
