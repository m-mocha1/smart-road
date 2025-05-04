pub struct syara {
    // hada syara
    position: (f32, f32),
    direction: Direction,
    lane: Lane,
    speed: f32,
}
pub enum Lane {
    // hada maslk
    left,
    right,
    do5ry,
}
pub enum Direction {
    // hada itijahat
    Going_up,
    Going_down,
    Going_left,
    Going_right,
}
impl syara {}
