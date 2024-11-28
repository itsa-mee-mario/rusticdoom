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
    vertices: &Vec<(f32, f32)>,  // Decoded VERTEXES
    linedefs: Vec<LineDef>,      // Decoded LINEDEFS
) {
    const WIDTH: i32 = 640;
    const HEIGHT: i32 = 480;
    const FOV: f32 = 120.0 * PI / 180.0; // Wider Field of View (120 degrees)
    const WALL_HEIGHT: f32 = 64.0;       // Wall height for rendering
    const NEAR_PLANE: f32 = 0.1;         // Minimum view distance
    const SCALE: f32 = 0.1;              // Scaling factor for world space to view space

    // Clear buffer to black
    for pixel in buffer.iter_mut() {
        *pixel = 0x000000; // Black background
    }

    let half_width = WIDTH as f32 / 2.0;
    let half_height = HEIGHT as f32 / 2.0;

    for linedef in linedefs {
        if let (Some(&(x1, y1)), Some(&(x2, y2))) = (
            vertices.get(linedef.start_vertex[0] as usize),
            vertices.get(linedef.end_vertex[0] as usize),
        ) {
            // Translate vertices relative to the player
            let dx1 = (x1 - player_x) * SCALE;
            let dy1 = (y1 - player_y) * SCALE;
            let dx2 = (x2 - player_x) * SCALE;
            let dy2 = (y2 - player_y) * SCALE;

            // Rotate vertices around the player's angle
            let (view_x1, view_z1) = rotate_around_player(dx1, dy1, player_angle);
            let (view_x2, view_z2) = rotate_around_player(dx2, dy2, player_angle);

            // Debug: Log view space coordinates
            // println!(
            //     "View space debug: v1=({:.2}, {:.2}), v2=({:.2}, {:.2})",
            //     view_x1, view_z1, view_x2, view_z2
            // );

            // Skip lines that are completely behind the player
            if view_z1 <= NEAR_PLANE && view_z2 <= NEAR_PLANE {
                // println!("Line skipped: Both vertices behind the near plane");
                continue;
            }

            // Clip near-plane intersections if required
            let (view_x1, view_z1) = if view_z1 <= NEAR_PLANE {
                let t = (NEAR_PLANE - view_z1) / (view_z2 - view_z1);
                (
                    view_x1 + t * (view_x2 - view_x1),
                    NEAR_PLANE,
                )
            } else {
                (view_x1, view_z1)
            };

            let (view_x2, view_z2) = if view_z2 <= NEAR_PLANE {
                let t = (NEAR_PLANE - view_z2) / (view_z1 - view_z2);
                (
                    view_x2 + t * (view_x1 - view_x2),
                    NEAR_PLANE,
                )
            } else {
                (view_x2, view_z2)
            };

            // Perspective projection to screen coordinates
            let screen_x1 = (half_width + (view_x1 / view_z1) * half_width / FOV.tan()) as i32;
            let screen_x2 = (half_width + (view_x2 / view_z2) * half_width / FOV.tan()) as i32;

            let height1 = (WALL_HEIGHT * half_height / view_z1) as i32;
            let height2 = (WALL_HEIGHT * half_height / view_z2) as i32;

            // Debug: Log screen projection
            // println!(
            //     "Screen projection: x1_screen={}, x2_screen={}, height1={}, height2={}",
            //     screen_x1, screen_x2, height1, height2
            // );

            // Clip line to screen bounds
            if screen_x1 < 0 && screen_x2 < 0 || screen_x1 >= WIDTH && screen_x2 >= WIDTH {
                // println!("Line skipped: Outside screen bounds");
                continue;
            }

            let clipped_x1 = screen_x1.clamp(0, WIDTH - 1);
            let clipped_x2 = screen_x2.clamp(0, WIDTH - 1);

            // Render the vertical lines of the wall
            for x in clipped_x1..=clipped_x2 {
                let t = if screen_x2 != screen_x1 {
                    (x - screen_x1) as f32 / (screen_x2 - screen_x1) as f32
                } else {
                    0.0
                };

                // Interpolate wall height
                let interpolated_height = height1 as f32 + t * (height2 as f32 - height1 as f32);
                let y_start = (half_height - interpolated_height) as i32;
                let y_end = (half_height + interpolated_height) as i32;

                // Debug: Log line rendering
                // println!(
                //     "Rendering vertical line: x={}, y_start={}, y_end={}",
                //     x, y_start, y_end
                // );

                // Fill the buffer with the wall's color
                for y in y_start.max(0)..=y_end.min(HEIGHT - 1) {
                    let buffer_index = (y * WIDTH + x) as usize;
                    buffer[buffer_index] = 0xFFFFFF; // White color for walls
                }
            }
        }
    }
}

// Function to rotate a point around the player's angle
fn rotate_around_player(x: f32, y: f32, angle: f32) -> (f32, f32) {
    let sin_angle = angle.sin();
    let cos_angle = angle.cos();
    (
        x * cos_angle - y * sin_angle, // Correct rotation
        x * sin_angle + y * cos_angle
    )
}



pub fn render_linedef(
    buffer: &mut Vec<u32>,
    world_objects: &Vec<(f32, f32)>,
    linedefs: Vec<LineDef>,
) {
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

    for linedef in linedefs {
        if let (Some(&(x1_screen, y1_screen)), Some(&(x2_screen, y2_screen))) = (
            screen_objects.get(linedef.start_vertex[0] as usize),
            screen_objects.get(linedef.end_vertex[0] as usize),
        ) {
            // println!("Drawing line: start=({:?}, {:?}), end=({:?}, {:?})", x1_screen, y1_screen, x2_screen, y2_screen);
            draw_line(
                buffer,
                x1_screen as i32,
                y1_screen as i32,
                x2_screen as i32,
                y2_screen as i32,
                0xFFFFFF,
            );
        }
    }
}
