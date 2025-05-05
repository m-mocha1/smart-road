extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::{event::Event, image::LoadTexture};
use std::time::Duration;

use super::syara::{CarTextures, Direction, Syara};

pub fn open_window() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Smart Intersection", 1000, 1000)
        // .position(2500, 250)
        // .position_centered()
        .position(900, 80)
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut car_texture = CarTextures {
        // 
        // C:/Users/admar/Desktop/smart-road/target/debug/cars/Police.png
        left: texture_creator.load_texture("C:/Users/admar/Desktop/smart-road/target/debug/cars/PoliceLeft.png")?,
        right: texture_creator.load_texture("C:/Users/admar/Desktop/smart-road/target/debug/cars/PoliceRight.png")?,
        up: texture_creator.load_texture("C:/Users/admar/Desktop/smart-road/target/debug/cars/Police.png")?,
        down: texture_creator.load_texture("C:/Users/admar/Desktop/smart-road/target/debug/cars/PoliceDown.png")?,
    };
    let mut event_pump = sdl_context.event_pump()?;
    let mut syarat: Vec<Syara> = Vec::new();


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
                //make syarat
                Event::KeyDown{
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let syara = Syara::new(
                        (900.0,390.0), // mid
                        Direction::Going_left,
                        super::syara::Lane::do5ry,
                        100.0,
                    );
                    syarat.push(syara);
                }
                Event::KeyDown{
                    keycode: Some(Keycode::UP),
                    ..
                } => {
                    let syara = Syara::new(
                        (900.0,390.0), // mid
                        Direction::Going_up,
                        super::syara::Lane::do5ry,
                        100.0,
                    );
                    syarat.push(syara);
                }
                Event::KeyDown{
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let syara = Syara::new(
                        (900.0,390.0), // mid
                        Direction::Going_right,
                        super::syara::Lane::do5ry,
                        100.0,
                    );
                    syarat.push(syara);
                }
                Event::KeyDown{
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let syara = Syara::new(
                        (900.0,390.0), // mid
                        Direction::Going_down,
                        super::syara::Lane::do5ry,
                        100.0,
                    );
                    syarat.push(syara);
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
            car.render(&mut canvas, &car_texture);
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
