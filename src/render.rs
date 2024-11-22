pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 640;
use std::f32::consts::PI;

use crate::wad_reader::LineDef;

fn clear_buffer(buffer: &mut Vec<u32>) {
    for i in 0..WIDTH * HEIGHT {
        buffer[i] = 0xff000000;
    }
}

pub fn draw_line(buffer: &mut Vec<u32>, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
    // clear_buffer(buffer);
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs(); // delta x and y
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 }; // step directions
    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    loop {
        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
            buffer[(y * WIDTH as i32 + x) as usize] = color;
        }

        if x == x2 && y == y2 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

pub fn perspective_render(
    buffer: &mut Vec<u32>,
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    world_objects: &Vec<(f32, f32)>,
) {
    for i in buffer.iter_mut() {
        *i = 0x000000;  
    }

    // renderes cross in front of the player
    let num_points_per_line = 5;
    let spacing = 4.0;
    let mut world_objects = vec![];

    for i in -num_points_per_line..=num_points_per_line {
        let object_y = (i as f32) * spacing + 15.0;
        world_objects.push((0.0, object_y));
    }
    for i in -num_points_per_line..=num_points_per_line {
        let object_x = (i as f32) * spacing;
        world_objects.push((object_x, 15.0));
    }



    let screen_center_x = WIDTH as f32 / 2.0;
    let screen_center_y = HEIGHT as f32 / 2.0;

    // sine and cosine for rotation
    let angle_rad = player_angle * (PI / 180.0);
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();

    let (min_x, max_x) = world_objects
        .iter()
        .map(|(x, _)| x)
        .fold((f32::MAX, f32::MIN), |(min, max), &x| {
            (min.min(x), max.max(x))
        });
    let (min_y, max_y) = world_objects
        .iter()
        .map(|(_, y)| y)
        .fold((f32::MAX, f32::MIN), |(min, max), &y| {
            (min.min(y), max.max(y))
        });

    let scale_x = WIDTH as f32 / (max_x - min_x).max(1.0);
    let scale_y = HEIGHT as f32 / (max_y - min_y).max(1.0);
    let scale = scale_x.min(scale_y) * 0.9; // 90% of screen to avoid clipping

    let screen_objects: Vec<(usize, usize)> = world_objects
        .iter()
        .map(|(object_x, object_y)| {
            // Normalize and scale coordinates
            let normalized_x = ((object_x - min_x) * scale) as usize
                + ((WIDTH as f32 - scale * (max_x - min_x)) / 2.0) as usize;
            let normalized_y = ((object_y - min_y) * scale) as usize
                + ((HEIGHT as f32 - scale * (max_y - min_y)) / 2.0) as usize;

            (normalized_x, normalized_y)
        })
        .collect();

    for (object_x, object_y) in &screen_objects {
        // calculate world coordinates relative to the player
        let f_object_x = *object_x as f32;
        let f_object_y = *object_y as f32;
        let relative_x = f_object_x - player_x;
        let relative_y = f_object_y - player_y;

        // calculate rotate coordinates based on player angle
        let rotated_x = relative_x * cos_angle - relative_y * sin_angle;
        let rotated_y = relative_x * sin_angle + relative_y * cos_angle;

        // println!("rotated coordinates: x {} and y {}", rotated_x, rotated_y);

        // calculate screen position using perspective projection
        if rotated_y > 0.0 {
            // render objects in front of the player only
            let depth_scale = 1.0 / (rotated_y + 1.0).max(0.01); // ensure we don't divide by zero or get negative depth
            let screen_x =
                (screen_center_x + rotated_x * depth_scale * 100.0).clamp(0.0, WIDTH as f32 - 1.0);
            let screen_y = (screen_center_y - depth_scale * 100.0).clamp(0.0, HEIGHT as f32 - 1.0);

            // render a larger box centered around (screen_x, screen_y)
            let size = (10.0 * depth_scale).clamp(5.0, 20.0) as usize;
            let screen_x = screen_x as usize;
            let screen_y = screen_y as usize;

            // println!(
            //     "Object at ({}, {}) -> Screen position: ({}, {})",
            //     object_x, object_y, screen_x, screen_y
            // );

            // draw a larger box if it's within bounds
            for dx in 0..size {
                for dy in 0..size {
                    let x = screen_x + dx.saturating_sub(size / 2);
                    let y = screen_y + dy.saturating_sub(size / 2);

                    if x < WIDTH && y < HEIGHT {
                        buffer[y * WIDTH + x] = 0xFFFFFF; // white (object color)
                    }
                }
            }
        }
    }
}

pub fn render_raw(buffer: &mut Vec<u32>, world_objects: &Vec<(f32, f32)>, linedefs: Vec<LineDef>) {
    // Clear the buffer first
    for i in buffer.iter_mut() {
        *i = 0x000000;
    }

    // Scale and center the map vertices
    let (min_x, max_x) = world_objects
        .iter()
        .map(|(x, _)| x)
        .fold((f32::MAX, f32::MIN), |(min, max), &x| {
            (min.min(x), max.max(x))
        });
    let (min_y, max_y) = world_objects
        .iter()
        .map(|(_, y)| y)
        .fold((f32::MAX, f32::MIN), |(min, max), &y| {
            (min.min(y), max.max(y))
        });

    let scale_x = WIDTH as f32 / (max_x - min_x).max(1.0);
    let scale_y = HEIGHT as f32 / (max_y - min_y).max(1.0);
    let scale = scale_x.min(scale_y) * 0.9; // 90% of screen to avoid clipping

    let screen_objects: Vec<(usize, usize)> = world_objects
        .iter()
        .map(|(object_x, object_y)| {
            // Normalize and scale coordinates
            let normalized_x = ((object_x - min_x) * scale) as usize
                + ((WIDTH as f32 - scale * (max_x - min_x)) / 2.0) as usize;
            let normalized_y = ((object_y - min_y) * scale) as usize
                + ((HEIGHT as f32 - scale * (max_y - min_y)) / 2.0) as usize;

            (normalized_x, normalized_y)
        })
        .collect();

    // Render vertices
    for (screen_x, screen_y) in &screen_objects {
        if *screen_x < WIDTH && *screen_y < HEIGHT {
            buffer[screen_y * WIDTH + screen_x] = 0xFFFFFF; // white (object color)
        }
    }
    // println!("Starting to render linedefs, total linedefs: {}", linedefs.len());
    
    for linedef in linedefs{
        if let (Some(&(x1_screen, y1_screen)), Some(&(x2_screen, y2_screen))) = (
            screen_objects.get(linedef.start_vertex[0] as usize),
            screen_objects.get(linedef.end_vertex[0] as usize),
        ) {
            // println!("Drawing line: start=({:?}, {:?}), end=({:?}, {:?})", x1_screen, y1_screen, x2_screen, y2_screen);
            draw_line(buffer, x1_screen as i32, y1_screen as i32, x2_screen as i32, y2_screen as i32, 0xFFFFFF);
        }
    }
}
