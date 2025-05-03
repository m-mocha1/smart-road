extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Smart Intersection", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        draw_intersection(&mut canvas)?;
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
fn draw_intersection(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
    const SCREEN_SIZE: u32 = 800;
    const LANE_WIDTH: u32 = 20;
    const ROAD_WIDTH: u32 = LANE_WIDTH * 6;
    const MID_WIDTH: i32 = ((SCREEN_SIZE / 2) - 140) as i32;

    let mid = (SCREEN_SIZE / 2) as i32;
    let half_road = (ROAD_WIDTH / 2) as i32;

    // Background
    canvas.set_draw_color(Color::RGB(30, 30, 30));
    canvas.clear();

    // Vertical road
    canvas.set_draw_color(Color::RGB(80, 80, 80));
    canvas.fill_rect(Rect::new(mid - half_road, 0, ROAD_WIDTH, SCREEN_SIZE))?;

    // Horizontal road
    canvas.fill_rect(Rect::new(0, MID_WIDTH, SCREEN_SIZE, ROAD_WIDTH))?;

    // Lane markings - optional: yellow/white lines
    canvas.set_draw_color(Color::RGB(100, 25, 250));
    for i in 1..3 {
        // Horizontal markings
        let offset = i * LANE_WIDTH;
        canvas.fill_rect(Rect::new(
            mid - half_road + offset as i32 - 1,
            0,
            2,
            SCREEN_SIZE,
        ))?;

        // Vertical markings
        canvas.fill_rect(Rect::new(
            0,
            mid - half_road + offset as i32 - 1,
            SCREEN_SIZE,
            2,
        ))?;
    }

    Ok(())
}
