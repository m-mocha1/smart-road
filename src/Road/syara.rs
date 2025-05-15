use crate::Road::drawRoad::detect_turn;
use crate::Road::drawRoad::turn_now;
use crate::Road::mafr2::build_occupancy_set;
use crate::Road::mafr2::grid_cell;
use sdl2::render::Texture;
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Syara {
    // hada syara
    pub position: (f32, f32),
    pub direction: Direction,
    pub lane: Lane,
    pub speed: f32,
    pub turned: bool,
    pub pri: bool,
    pub extLook: bool,
}

pub struct CarTextures<'a> {
    pub left: Texture<'a>,
    pub right: Texture<'a>,
    pub up: Texture<'a>,
    pub down: Texture<'a>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Lane {
    // hada maslk
    Left,
    Right,
    Do5ry,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Direction {
    // hada itijahat
    Going_up,
    Going_down,
    Going_left,
    Going_right,
}
impl Syara {
    //make a new car
    pub fn new(
        position: (f32, f32),
        direction: Direction,
        lane: Lane,
        speed: f32,
        turned: bool,
    ) -> Self {
        Self {
            position,
            direction,
            lane,
            speed,
            turned: false,
            pri: false,
            extLook: false,
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
        let x = self.position.0 + 40.0;
        let y = self.position.1 + 40.0;
        if !self.turned {
            let new = turn_now((x, y), self.direction, self.lane);
            if new != self.direction {
                self.turned = true;
                self.direction = new;
            }
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
            50, // width of the car
            50, // height of the car
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
pub fn is_close(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}
