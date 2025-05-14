use crate::Road::mafr2::build_occupancy_set;
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
        println!("{}", turned);
        Self {
            position,
            direction,
            lane,
            speed,
            turned,
            pri: false,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let displacement = self.speed * dt;
        //------------left turn vars----------------
        let upLeftTurn: (f32, f32) = (485.0, 435.0); //g
        let leftLeftTurn: (f32, f32) = (435.0, 440.0); //g
        let rightLeftTurn: (f32, f32) = (489.0, 484.0); //g
        let downLeftTurn: (f32, f32) = (440.0, 487.0); //g

        //-------right turn vars ----------------------------------
        let upRightTurn: (f32, f32) = (580.0, 580.0); //g
        let leftRightTurn: (f32, f32) = (345.0, 578.0); //g
        let rightRightTurn: (f32, f32) = (578.0, 345.0); //g
        let downRightTurn: (f32, f32) = (345.0, 341.0); //g
        // coming up and turning right
        if self.direction == Direction::Going_up
            && !self.turned
            && is_close(self.position.0, upRightTurn.0, 2.0)
            && is_close(self.position.1, upRightTurn.1, 2.0)
        {
            self.direction = Direction::Going_right;
            self.turned = true;
        }
        // coming from left and turning right
        if self.direction == Direction::Going_right
            && !self.turned
            && is_close(self.position.0, leftRightTurn.0, 2.0)
            && is_close(self.position.1, leftRightTurn.1, 2.0)
        {
            self.direction = Direction::Going_down;
        }
        // coming from right and turning right
        if self.direction == Direction::Going_left
            && !self.turned
            && is_close(self.position.0, rightRightTurn.0, 2.0)
            && is_close(self.position.1, rightRightTurn.1, 2.0)
        {
            self.direction = Direction::Going_up;
            self.turned = true;
        }
        // coming from up and turning right
        if self.direction == Direction::Going_down
            && !self.turned
            && is_close(self.position.0, downRightTurn.0, 2.0)
            && is_close(self.position.1, downRightTurn.1, 2.0)
        {
            self.direction = Direction::Going_left;
        }
        //--------------------------------------------------------------
        //-------left turn----------------------------------
        if self.direction == Direction::Going_up
            && !self.turned
            && is_close(self.position.0, upLeftTurn.0, 2.0)
            && is_close(self.position.1, upLeftTurn.1, 2.0)
        {
            self.direction = Direction::Going_left;
            self.turned = true;
        }
        if self.direction == Direction::Going_left
            && !self.turned
            && is_close(self.position.0, leftLeftTurn.0, 2.0)
            && is_close(self.position.1, leftLeftTurn.1, 2.0)
        {
            self.direction = Direction::Going_down;
            self.turned = true;
        }
        if self.direction == Direction::Going_right
            && !self.turned
            && is_close(self.position.0, rightLeftTurn.0, 2.0)
            && is_close(self.position.1, rightLeftTurn.1, 2.0)
        {
            self.direction = Direction::Going_up;
            self.turned = true;
        }
        if self.direction == Direction::Going_down
            && !self.turned
            && is_close(self.position.0, downLeftTurn.0, 2.0)
            && is_close(self.position.1, downLeftTurn.1, 2.0)
        {
            self.direction = Direction::Going_right;
            self.turned = true;
        }

        //----------------------------------------------------------------
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
            70, // width of the car
            70, // height of the car
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
