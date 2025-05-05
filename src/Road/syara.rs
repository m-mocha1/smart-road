#[derive(Debug)]
pub struct Syara {
    // hada syara
    position: (f32, f32),
    direction: Direction,
    lane: Lane,
    speed: f32,
}

#[derive(Debug)]
pub enum Lane {
    // hada maslk
    left,
    right,
    do5ry,
}
#[derive(Debug)]
pub enum Direction {
    // hada itijahat
    Going_up,
    Going_down,
    Going_left,
    Going_right,
}
impl Syara {
    //make a new car
    pub fn new(position: (f32, f32), direction: Direction, lane: Lane, speed: f32) -> Self {
        Self {
            position,
            direction,
            lane,
            speed,
        }
    }
    pub fn update_position(&mut self, dt: f32) {
        let displacement = self.speed * dt;
        match self.direction {
            Direction::Going_left => self.position.0 -= displacement,
            Direction::Going_right => self.position.0 += displacement,
            Direction::Going_up => self.position.1 -= displacement,
            Direction::Going_down => self.position.1 += displacement,
        }
    }
    pub fn render(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture: &sdl2::render::Texture,
    ) {
        let rect = sdl2::rect::Rect::new(
            self.position.0 as i32,
            self.position.1 as i32,
            80, // width of the car
            80, // height of the car
        );
        let _ = canvas.copy(texture, None, rect);
    }
}
