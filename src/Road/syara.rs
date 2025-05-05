use sdl2::render::Texture;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Syara {
    // hada syara
    pub position: (f32, f32),
    pub direction: Direction,
    pub lane: Lane,
    pub speed: f32,
}

pub struct CarTextures<'a> {
    pub left: Texture<'a>,
    pub right: Texture<'a>,
    pub up: Texture<'a>,
    pub down: Texture<'a>,
}

#[derive(Debug, PartialEq, PartialOrd)]

pub enum Lane {
    // hada maslk
    left,
    right,
    do5ry,
}
#[derive(Debug, PartialEq, PartialOrd)]

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
        sor: &CarTextures,
    ) {
        let rect = sdl2::rect::Rect::new(
            self.position.0 as i32,
            self.position.1 as i32,
            80, // width of the car
            80, // height of the car
        );

        let sora = match self.direction {
            Direction::Going_left => &sor.left,
            Direction::Going_right => &sor.right,
            Direction::Going_up => &sor.up,
            Direction::Going_down => &sor.down,
        };
        let _ = canvas.copy(sora, None, rect);
    }
}
