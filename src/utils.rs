use std::f32::consts::PI;
use std::cmp::Ordering;

pub fn invert_f32_tuple(value: (f32,f32)) -> (f32, f32) {
    let (mut start, mut end) = value;
    (-end, -start)
}

pub fn get_y_absolute_move(euler_angle: (f32, f32, f32), move_amount: f32) -> (f32, f32, f32){
    let euler_angle = euler_angle.2;
    // This is up
    if let Ordering::Equal = euler_angle.partial_cmp(&0.0).unwrap() {
        return (0.0, move_amount, 0.0);
    }
    // This is Right
    if let Ordering::Equal = euler_angle.partial_cmp(&(-0.5 * PI)).unwrap() {
        return (-move_amount, 0.0, 0.0);
    }
    // This is down
    if let Ordering::Equal = euler_angle.partial_cmp(&PI).unwrap() {
        return (0.0, -move_amount, 0.0);
    }
    // This is left
    if let Ordering::Equal = euler_angle.partial_cmp(&(0.5 * PI)).unwrap() {
        return (move_amount, 0.0, 0.0);
    }
    (0.0, 0.0, 0.0)
}
