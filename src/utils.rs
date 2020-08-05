pub fn invert_f32_tuple(value: (f32,f32)) -> (f32, f32) {
    let (mut start, mut end) = value;
    (-end, -start)
}
