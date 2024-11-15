pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 640;
use std::f32::consts::PI;

pub fn render(buffer: &mut Vec<u32>, player_x: f32, player_y: f32, player_angle: f32) {
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

    // let world_objects = vec![
    //     (0.0, 0.0),
    //     (10.0, 0.0),
    //     (0.0, 10.0),
    //     (10.0, 10.0),
    //     // (-10.0, 0.0),
    //     // (0.0, -10.0),
    //     // (-10.0, -10.0),
    //     // (0.0, 0.0),
    //     // (-10.0, 5.0),  // Left of player
    //     // (10.0, 5.0),   // Right of player
    //     // (0.0, 15.0),   // Straight ahead of player
    //     // (0.0, -5.0),   // Behind player
    // ];

    let screen_center_x = WIDTH as f32 / 2.0;
    let screen_center_y = HEIGHT as f32 / 2.0;

    // sine and cosine for rotation
    let angle_rad = player_angle * (PI / 180.0);
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();

    for (object_x, object_y) in world_objects {
        // calculate world coordinates relative to the player
        let relative_x = object_x - player_x;
        let relative_y = object_y - player_y;

        // calculate rotate coordinates based on player angle
        let rotated_x = relative_x * cos_angle - relative_y * sin_angle;
        let rotated_y = relative_x * sin_angle + relative_y * cos_angle;

        // calculate screen position using perspective projection
        if rotated_y > 0.0 {  // render objects in front of the player only
            let depth_scale = 1.0 / (rotated_y + 1.0).max(0.01); // ensure we don't divide by zero or get negative depth
            let screen_x = (screen_center_x + rotated_x * depth_scale * 100.0).clamp(0.0, WIDTH as f32 - 1.0);
            let screen_y = (screen_center_y - depth_scale * 100.0).clamp(0.0, HEIGHT as f32 - 1.0);

            // render a larger box centered around (screen_x, screen_y)
            let size = (10.0 * depth_scale).clamp(5.0, 20.0) as usize;
            let screen_x = screen_x as usize;
            let screen_y = screen_y as usize;

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