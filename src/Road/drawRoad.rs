extern crate sdl2;
use rand::seq::SliceRandom;
use sdl2::keyboard::Keycode;
use sdl2::{event::Event, image::LoadTexture};
use crate::Road::mafr2::grid_cell;
use crate::Road::mafr2::build_occupancy_set;
use std::collections::HashSet;
use std::env;
use std::time::{Duration, Instant};
    use rand::prelude::*;

// use std::time::Duration;

use super::syara::{CarTextures, Direction, Syara, Lane};
use crate::Road::mafr2::draw_intersection;

pub fn open_window() -> Result<(), String> {
    let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
    println!("SDL2 initialized successfully!");
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Smart Intersection", 1000, 1000)
        .position(2500, 10)
        // .position_centered()
        // .position(900, 80)
        // .position(1500, 80)
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
    let soar = CarTextures {
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
                Event::KeyDown{
                    keycode: Some(Keycode::R),
                    ..
                } => {  
                     let now = Instant::now();
                    if now.duration_since(last_spawn_time) >= Duration::from_secs(1) {
                        let dir = random_dir();
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







        let dt = 1.0 / 60.0;

       
        
      
      
        let mut reserved = HashSet::new();
        let occupied = build_occupancy_set(&syarat);

        for car in &mut syarat {    
            let path = predict_path(car, 3);
             let mut blocked = false;
  let oci = path.iter().any(|cell| occupied.contains(cell));
        for cell in path.iter().take(5) {
        if reserved.contains(cell) {
            blocked = true;
            break;
        }
    }
    if oci {
        car.speed = (car.speed - 100.0 * dt).max(10.0); // slow down
    }

    if blocked {
        car.speed = (car.speed - 100.0 * dt).max(10.0); // slow down
    } else {
        car.speed = (car.speed + 80.0 * dt).min(100.0); // restore speed
      for cell in path.iter().take(4) {
    reserved.insert(*cell);
    }
    }
        car.speed = (car.speed + 80.0 * dt).min(100.0); // restore speed
    

    car.update_position(dt);
        }

        draw_intersection(&mut canvas, &syarat,&reserved)?;
        for car in &syarat {
            car.render(&mut canvas, &soar);
        }
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

fn random_lane_and_pos(direction: &Direction) -> (super::syara::Lane, (f32, f32)) {
    use super::syara::{Direction, Lane};
    use rand::prelude::*;

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
        (Lane::Do5ry, Direction::Going_left) => (900.0, 390.0),

        (Lane::Left, Direction::Going_down) => (440.0, 0.0),
        (Lane::Right, Direction::Going_down) => (345.0, 0.0),
        (Lane::Do5ry, Direction::Going_down) => (391.0, 0.0),
    };

    (lane, pos)
}
fn random_dir() -> Direction {

    let mut rng = thread_rng();
    let dir = vec![
        Direction::Going_down,
        Direction::Going_left,
        Direction::Going_right,
        Direction::Going_up,
    ];
    *dir.choose(&mut rng).unwrap()
}


fn predict_path(car: &Syara, max_cells: usize) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    let mut seen = HashSet::new();

    let step_size = 1.0;
    let mut distance = 0.0;

    let mut x = car.position.0 + 20.0;
    let mut y = car.position.1 + 20.0;
    let mut dir = car.direction;

    while path.len() < max_cells && distance < 300.0 {
        // 👇 Apply turning logic
        if is_in_intersection_center((x, y)) {
            dir = match (car.lane, dir) {
                (Lane::Do5ry, Direction::Going_up) => Direction::Going_left,
                (Lane::Do5ry, Direction::Going_down) => Direction::Going_right,
                (Lane::Do5ry, Direction::Going_left) => Direction::Going_down,
                (Lane::Do5ry, Direction::Going_right) => Direction::Going_up,
                _ => dir, // no turn
            };
        }

        // Advance position in current direction
        let (dx, dy) = match dir {
            Direction::Going_up => (0.0, -step_size),
            Direction::Going_down => (0.0, step_size),
            Direction::Going_left => (-step_size, 0.0),
            Direction::Going_right => (step_size, 0.0),
        };

        x += dx;
        y += dy;
        distance += step_size;

        if let Some(cell) = grid_cell((x, y)) {
            if seen.insert(cell) {
                path.push(cell);
            }
        } else {
            break;
        }
    }
    path
}
fn is_in_intersection_center(pos: (f32, f32)) -> bool {
    if let Some((row, col)) = grid_cell(pos) {
        row >= 4 && row <= 9 && col >= 4 && col <= 9 // adjust as needed
    } else {
        false
    }
}

  