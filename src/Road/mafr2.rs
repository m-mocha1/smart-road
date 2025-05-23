use crate::Road::syara::Syara;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashSet;

pub fn draw_intersection(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    syrat: &Vec<Syara>,
    reserved: &HashSet<(usize, usize)>,
    v: bool,
) -> Result<(), String> {
    const SCREEN_SIZE: u32 = 1000;
    const LANE: u32 = 47;
    const ROAD_WIDTH: u32 = LANE * 6;
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
    if v {
        let occupied = build_occupancy_set(&syrat);
        const GRID_ROWS: i32 = 22;
        const GRID_COLS: i32 = 22;
        let grid_w = GRID_COLS * LANE as i32;
        let grid_h = GRID_ROWS * LANE as i32;
        let origin_x = mid - grid_w / 2;
        let origin_y = mid - grid_h / 2;
        canvas.set_draw_color(Color::RGB(128, 0, 128)); // purple
        for row in 0..GRID_ROWS {
            for col in 0..GRID_COLS {
                let x = origin_x + col * LANE as i32;
                let y = origin_y + row * LANE as i32;

                // Outline
                canvas.set_draw_color(Color::RGB(128, 0, 128));
                canvas.draw_rect(Rect::new(x, y, LANE, LANE))?;

                if reserved.contains(&(row as usize, col as usize)) {
                    canvas.set_draw_color(Color::RGBA(255, 0, 0, 96)); // red
                    canvas.fill_rect(Rect::new(x, y, LANE, LANE))?;
                }

                if occupied.contains(&(row as usize, col as usize)) {
                    canvas.set_draw_color(Color::RGBA(128, 0, 128, 96));
                    canvas.fill_rect(Rect::new(x, y, LANE, LANE))?;
                }
            }
        }
    }

    for i in 0..7 {
        // vertical
        let offset = i * LANE;
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

        // horizon
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
pub fn build_occupancy_set(cars: &[Syara]) -> HashSet<(usize, usize)> {
    let mut s = HashSet::new();
    for car in cars {
        let car_center = (car.position.0 + 10.0, car.position.1 + 10.0);
        if let Some(cell) = grid_cell(car_center) {
            s.insert(cell);
        }
    }
    s
}

pub fn grid_cell(pos: (f32, f32)) -> Option<(usize, usize)> {
    const SCREEN: u32 = 1000;
    const LANE: u32 = 47;
    const GRID_ROWS: usize = 22;
    const GRID_COLS: usize = 22;

    let mid = (SCREEN / 2) as i32;
    let grid_w = (GRID_COLS as u32 * LANE) as i32;
    let grid_h = (GRID_ROWS as u32 * LANE) as i32;
    let origin_x = mid - grid_w / 2;
    let origin_y = mid - grid_h / 2;

    let x = pos.0.round() as i32;
    let y = pos.1.round() as i32;

    if x < origin_x || x >= origin_x + grid_w || y < origin_y || y >= origin_y + grid_h {
        return None;
    }

    let col = ((x - origin_x) / LANE as i32) as usize;
    let row = ((y - origin_y) / LANE as i32) as usize;

    if row < GRID_ROWS && col < GRID_COLS {
        Some((row, col))
    } else {
        None
    }
}
pub fn cell_to_spawn_pos((row, col): (usize, usize)) -> (f32, f32) {
    const SCREEN: f32 = 1000.0;
    const LANE: f32 = 47.0;
    const GRID_ROWS: usize = 22;
    const GRID_COLS: usize = 22;
    const SPRITE_HALF: f32 = 25.0;
    // 1) compute grid origin in world coords
    let mid = SCREEN / 2.0;
    let grid_w = GRID_COLS as f32 * LANE;
    let grid_h = GRID_ROWS as f32 * LANE;
    let origin_x = mid - grid_w / 2.0;
    let origin_y = mid - grid_h / 2.0;

    // 2) compute the pixel‐coords of the *center* of that cell
    let center_x = origin_x + (col as f32 + 0.5) * LANE;
    let center_y = origin_y + (row as f32 + 0.5) * LANE;

    // 3) subtract half the sprite size to get the top‐left
    (center_x - SPRITE_HALF, center_y - SPRITE_HALF)
}
