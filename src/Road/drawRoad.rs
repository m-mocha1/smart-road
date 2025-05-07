extern crate sdl2;
use std::env;
use std::time::{Duration, Instant};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::{event::Event, image::LoadTexture};
// use std::time::Duration;

use super::syara::{CarTextures, Direction, Syara};

pub fn open_window() -> Result<(), String> {
    let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
println!("SDL2 initialized successfully!");
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Smart Intersection", 1000, 1000)
        // .position(2500, 250)
        // .position_centered()
        // .position(900, 80)
        .position(1500, 80)

        .build()
        .map_err(|e| e.to_string())?;
        println!("Window created successfully!");

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    // cars img
    let texture_creator = canvas.texture_creator();

    
    let mut path = env::current_dir().unwrap();
    println!("Current working dir: {:?}", env::current_dir());


        path.push("img");
        // C:/Users/admar/Desktop/smart-road/target/debug/cars/Police.png
        let  soar = CarTextures{
            left: texture_creator.load_texture(path.join("left.png"))?,
            right: texture_creator.load_texture(path.join("right.png"))?,
            up: texture_creator.load_texture(path.join("up.png"))?,
            down: texture_creator.load_texture(path.join("down.png"))?,
};

    let mut event_pump = sdl_context.event_pump()?;
    let mut syarat: Vec<Syara> = Vec::new();
    let mut last_spawn_time = Instant::now() - Duration::from_secs(7); // So the first keypress works



    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                //if window closed
                Event::Quit { .. }
                //esc key
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                //make syaratS
                Event::KeyDown{
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let now = Instant::now();
    if now.duration_since(last_spawn_time) >= Duration::from_secs(1) {
                    let dir=Direction::Going_left;
                    let (mslk,mok3)=random_lane_and_pos(&dir);
                    let syara = Syara::new(
                        mok3,
                        dir,
                        mslk,
                        100.0,
                    );
                    syarat.push(syara);
                    last_spawn_time = now;
                }
                }
                Event::KeyDown{
                    keycode: Some(Keycode::UP),
                    ..
                } => {
                    let now = Instant::now();
                    if now.duration_since(last_spawn_time) >= Duration::from_secs(1) {
                    let dir=Direction::Going_up;
                    let (mslk,mok3)=random_lane_and_pos(&dir);
                    let syara = Syara::new(
                        mok3,
                        dir,
                        mslk,
                        100.0,
                    );
                    syarat.push(syara);
                    last_spawn_time = now;
                }
                }
                Event::KeyDown{
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let now = Instant::now();
                    if now.duration_since(last_spawn_time) >= Duration::from_secs(1) {
                    let dir=Direction::Going_right;
                    let (mslk,mok3)=random_lane_and_pos(&dir);
                    let syara = Syara::new(
                        mok3,
                        dir,
                        mslk,
                        100.0,
                    );
                    syarat.push(syara);
                    last_spawn_time = now;
                }
                }
                Event::KeyDown{
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let now = Instant::now();
                    if now.duration_since(last_spawn_time) >= Duration::from_secs(1) {
                    let dir=Direction::Going_down;
                    let (mslk,mok3)=random_lane_and_pos(&dir);
                    let syara = Syara::new(
                        mok3,
                        dir,
                        mslk,
                        100.0,
                    );
                    syarat.push(syara);
                    last_spawn_time = now;
                }
                }




                _ => {}
            }
        }
        let dt = 1.0 / 60.0; // 60 FPS frame time
        for car in &mut syarat {
            car.update_position(dt);
        }

        draw_intersection(&mut canvas)?;
        for car in &syarat {
            car.render(&mut canvas, &soar);
        }
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

pub fn draw_intersection(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) -> Result<(), String> {
    const SCREEN_SIZE: u32 = 1000;
    const LANE_WIDTH: u32 = 47;
    const ROAD_WIDTH: u32 = LANE_WIDTH * 6;
    const MID_WIDTH: i32 = ((SCREEN_SIZE / 2) - 140) as i32;

    let top_lines = (SCREEN_SIZE - 640) as u32;

    let mid = (SCREEN_SIZE / 2) as i32;
    let half_road = (ROAD_WIDTH / 2) as i32;

    // Background
    canvas.set_draw_color(Color::RGB(30, 30, 30));
    canvas.clear();

    //Road color
    canvas.set_draw_color(Color::RGB(60, 60, 60));

    // Vertical road
    canvas.fill_rect(Rect::new(mid - half_road, 0, ROAD_WIDTH, SCREEN_SIZE))?;

    // Horizontal road
    canvas.fill_rect(Rect::new(0, MID_WIDTH, SCREEN_SIZE, ROAD_WIDTH))?;

    // Lane markings - optional: yellow/white lines
    canvas.set_draw_color(Color::RGB(255, 255, 0));

    for i in 0..7 {
        // Vertical markings
        let offset = i * LANE_WIDTH;
        if i == 0 || i == 6 {
            canvas.fill_rect(Rect::new(
                mid - half_road + offset as i32 - 1,
                0,
                2,
                SCREEN_SIZE,
            ))?;
        } else if i == 3 {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.fill_rect(Rect::new(
                mid - half_road + offset as i32 - 1,
                mid + half_road,
                2,
                top_lines,
            ))?;
            canvas.fill_rect(Rect::new(
                mid - half_road + offset as i32 - 1,
                0,
                2,
                top_lines,
            ))?;
        } else {
            // top lines
            canvas.set_draw_color(Color::RGB(255, 255, 0));
            canvas.fill_rect(Rect::new(
                mid - half_road + offset as i32 - 1,
                0,
                2,
                top_lines,
            ))?;
            canvas.fill_rect(Rect::new(
                mid - half_road + offset as i32 - 1,
                mid + half_road,
                2,
                top_lines,
            ))?;
        }

        // Horizontal markings
        if i == 0 || i == 6 {
            canvas.fill_rect(Rect::new(
                0,
                mid - half_road + offset as i32 - 1,
                SCREEN_SIZE,
                2,
            ))?;
        }
        canvas.fill_rect(Rect::new(
            0,
            mid - half_road + offset as i32 - 1,
            top_lines,
            2,
        ))?;

        canvas.fill_rect(Rect::new(
            mid + half_road,
            mid + half_road - offset as i32 + 1,
            top_lines,
            2,
        ))?;
    }

    Ok(())
}
fn random_lane_and_pos(direction: &Direction) -> (super::syara::Lane, (f32, f32)) {
    use rand::prelude::*;
    use super::syara::{Lane, Direction};

    let mut rng = thread_rng();
    let lanes = [Lane::Left, Lane::Right, Lane::Do5ry];
    let lane = *lanes.choose(&mut rng).unwrap();

    let pos = match (&lane, direction) {
        (Lane::Left, Direction::Going_right) => (0.0, 484.0),
        (Lane::Right, Direction::Going_right) => (0.0, 578.0),
        (Lane::Do5ry, Direction::Going_right) => (0.0, 530.0),

        (Lane::Left, Direction::Going_up) => (485.0, 980.0),
        (Lane::Right, Direction::Going_up) => (580.0, 980.0),
        (Lane::Do5ry, Direction::Going_up) => (530.0, 980.0),

        (Lane::Left, Direction::Going_left) => (980.0, 440.0),
        (Lane::Right, Direction::Going_left) => (980.0, 345.0),
        (Lane::Do5ry, Direction::Going_left) => (900.0,390.0),

        (Lane::Left, Direction::Going_down) => (440.0, 0.0),
        (Lane::Right, Direction::Going_down) => (345.0, 0.0),
        (Lane::Do5ry, Direction::Going_down) => (391.0,0.0),
    };

    (lane, pos)
}