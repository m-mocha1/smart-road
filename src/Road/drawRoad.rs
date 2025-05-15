extern crate sdl2;
use crate::Road::mafr2::build_occupancy_set;
use crate::Road::mafr2::cell_to_spawn_pos;
use crate::Road::mafr2::grid_cell;
use rand::prelude::*;
use rand::seq::SliceRandom;
use sdl2::keyboard::Keycode;
use sdl2::{event::Event, image::LoadTexture};
use std::collections::HashSet;
use std::env;
use std::time::{Duration, Instant};

use super::syara::{CarTextures, Direction, Lane, Syara};
use crate::Road::mafr2::draw_intersection;

pub fn open_window() -> Result<(), String> {
    let mut r = false;
    let mut v = false;

    let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
    println!("SDL2 initialized successfully!");
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Smart Intersection", 1000, 1000)
        // .position(2500, 10)
        .position_centered()
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
        println!("{}",syarat.len());
        for event in event_pump.poll_iter() {
            match event {
                //if window closed
                Event::Quit { .. } => break 'running,
                //esc key
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    syarat.clear();
                }

                //make syaratSv
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let now = Instant::now();
                    if now.duration_since(last_spawn_time) >= Duration::from_secs(1) {
                        let dir = Direction::Going_left;
                        let (mslk, mok3) = random_lane_and_pos(&dir);
                        let syara = Syara::new(mok3, dir, mslk, 100.0, false);
                        syarat.push(syara);
                        last_spawn_time = now;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::UP),
                    ..
                } => {
                    let now = Instant::now();
                    if now.duration_since(last_spawn_time) >= Duration::from_secs(1) {
                        let dir = Direction::Going_up;
                        let (mslk, mok3) = random_lane_and_pos(&dir);
                        let syara = Syara::new(mok3, dir, mslk, 100.0, false);
                        syarat.push(syara);
                        last_spawn_time = now;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let now = Instant::now();
                    if now.duration_since(last_spawn_time) >= Duration::from_secs(1) {
                        let dir = Direction::Going_right;
                        let (mslk, mok3) = random_lane_and_pos(&dir);
                        let syara = Syara::new(mok3, dir, mslk, 100.0, false);
                        syarat.push(syara);
                        last_spawn_time = now;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let now = Instant::now();
                    if now.duration_since(last_spawn_time) >= Duration::from_secs(1) {
                        let dir = Direction::Going_down;
                        let (mslk, mok3) = random_lane_and_pos(&dir);
                        let syara = Syara::new(mok3, dir, mslk, 100.0, false);
                        syarat.push(syara);
                        last_spawn_time = now;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    if !r {
                        r = true;
                    } else {
                        r = false;
                    }
                    println!("r : {}", r);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => {
                    if !v {
                        v = true;
                    } else {
                        v = false;
                    }
                }
                _ => {}
            }
        }
        if r { 
            let now = Instant::now();
            if now.duration_since(last_spawn_time) >= Duration::from_millis(500) &&  syarat.len() <= 12 {
                let dir = random_dir();
                let (mslk, mok3) = random_lane_and_pos(&dir);
                let syara = Syara::new(mok3, dir, mslk, 100.0, false);
                syarat.push(syara);
                last_spawn_time = now;
            }
        }
        let dt = 1.0 / 60.0;

        let mut reserved = HashSet::new();
        let occupied = build_occupancy_set(&syarat);
        for car in &mut syarat {
            let look = add_look((car.position.0 + 25.0, car.position.1 + 25.0), car);
            if let Some(curr) = grid_cell((car.position.0 + 25.0, car.position.1 + 25.0)) {
                reserved.insert(curr);
            }
            let path = predict_path(car, look);

            let dist_to_block = path
                .iter()
                .position(|cell| occupied.contains(cell) || reserved.contains(cell));

            match dist_to_block {
                Some(d) if d <= look => {
                    car.speed = (car.speed - 100.0 * dt).max(20.0);
                }
                _ => {
                    car.speed = (car.speed + 80.0 * dt).min(100.0);
                    for &cell in path.iter().take(look) {
                        reserved.insert(cell);
                    }
                }
            }

            car.update_position(dt);
        }
        syarat.retain(|car| {
            let center = (car.position.0 + 25.0, car.position.1 + 25.0);
            grid_cell(center).is_some()
        });

        draw_intersection(&mut canvas, &syarat, &reserved, v)?;
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
        (Lane::Left, Direction::Going_right) => cell_to_spawn_pos((11, 0)),
        (Lane::Do5ry, Direction::Going_right) => cell_to_spawn_pos((12, 0)),
        (Lane::Right, Direction::Going_right) => cell_to_spawn_pos((13, 0)),

        (Lane::Left, Direction::Going_up) => cell_to_spawn_pos((21, 11)),
        (Lane::Do5ry, Direction::Going_up) => cell_to_spawn_pos((21, 12)),
        (Lane::Right, Direction::Going_up) => cell_to_spawn_pos((21, 13)),

        (Lane::Left, Direction::Going_left) => cell_to_spawn_pos((8, 21)),
        (Lane::Do5ry, Direction::Going_left) => cell_to_spawn_pos((9, 21)),
        (Lane::Right, Direction::Going_left) => cell_to_spawn_pos((10, 21)),

        (Lane::Left, Direction::Going_down) => cell_to_spawn_pos((0, 8)),
        (Lane::Do5ry, Direction::Going_down) => cell_to_spawn_pos((0, 9)),
        (Lane::Right, Direction::Going_down) => cell_to_spawn_pos((0, 10)),
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

fn predict_path(car: &mut Syara, max_cells: usize) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    let mut seen = HashSet::new();

    let step_size = 0.5;
    let mut distance = 0.0;

    // start at the car’s center
    let mut x = car.position.0 + 20.0;
    let mut y = car.position.1 + 25.0;
    // mark current cell as seen so it's not pushed
    if let Some(curr) = grid_cell((x, y)) {
        seen.insert(curr);
    }

    let mut turned = false;
    let mut dir = car.direction;
    while path.len() < max_cells && distance < 300.0 {
        if !turned && !car.turned {
            let new_dir = detect_turn((x, y), dir, car.lane);
            if new_dir != dir {
                dir = new_dir;
                turned = true;
            }
        }

        // move one step
        let (dx, dy) = match dir {
            Direction::Going_up => (0.0, -step_size),
            Direction::Going_down => (0.0, step_size),
            Direction::Going_left => (-step_size, 0.0),
            Direction::Going_right => (step_size, 0.0),
        };
        x += dx;
        y += dy;
        distance += step_size;

        // now push only *new* cells
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
pub fn detect_turn(pos: (f32, f32), dir: Direction, lane: Lane) -> Direction {
    if let Some((row, col)) = grid_cell(pos) {
        match (dir, lane, row, col) {
            (Direction::Going_up, Lane::Left, 10, 11) => Direction::Going_left,
            (Direction::Going_up, Lane::Right, 13, 13) => Direction::Going_right,
            (Direction::Going_right, Lane::Left, 11, 11) => Direction::Going_up,
            (Direction::Going_right, Lane::Right, 13, 8) => Direction::Going_down,
            (Direction::Going_left, Lane::Left, 8, 13) => Direction::Going_up,
            (Direction::Going_left, Lane::Right, 10, 10) => Direction::Going_down,
            (Direction::Going_down, Lane::Left, 8, 8) => Direction::Going_left,
            (Direction::Going_down, Lane::Right, 11, 10) => Direction::Going_right,
            _ => dir,
        }
    } else {
        dir
    }
}
pub fn turn_now(pos: (f32, f32), dir: Direction, lane: Lane) -> Direction {
    if let Some((row, col)) = grid_cell(pos) {
        match (dir, lane, row, col) {
            (Direction::Going_up, Lane::Left, 10, 11) => Direction::Going_left,
            (Direction::Going_up, Lane::Right, 13, 13) => Direction::Going_right,
            (Direction::Going_right, Lane::Left, 11, 12) => Direction::Going_up,
            (Direction::Going_right, Lane::Right, 13, 9) => Direction::Going_down,
            (Direction::Going_left, Lane::Left, 8, 13) => Direction::Going_up,
            (Direction::Going_left, Lane::Right, 10, 10) => Direction::Going_down,
            (Direction::Going_down, Lane::Left, 9, 8) => Direction::Going_left,
            (Direction::Going_down, Lane::Right, 12, 10) => Direction::Going_right,
            _ => dir,
        }
    } else {
        dir
    }
}
pub fn add_look(pos: (f32, f32), car: &mut Syara) -> usize {
    if car.extLook {
        return 6;
    }
    match grid_cell(pos) {
        Some((row, col)) => match (row, col) {
            //left
             (11, 4)
            | (12, 4)
            //right
            | (9, 18)
            | (10, 18)
            // up 
            | (4, 9)    
            | (4, 10)    
            //down
            | (17, 12)
            | (17, 11) => {
                car.extLook = true;
                10
            }
            _ => 2,
        },
        None => 2,
    }
}
