use minifb::Key;
use std::time::Instant;

pub struct Game {
    last_update: Instant,
    pub render_map: bool,
    last_toggle_time: Instant,
}

const PLAYER_SPEED: f32 = 100.0;
const PLAYER_ROTATION_SPEED: f32 = 30.0;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
        }
    }

    pub fn move_x(&mut self, delta: f32) {
        self.x = self.x + delta;
    }

    pub fn move_y(&mut self, delta: f32) {
        self.y = self.y + delta;
    }

    pub fn rotate(&mut self, delta_angle: f32) {
        self.angle = (self.angle + delta_angle) % 360.0;
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            last_update: Instant::now(),
            render_map: false,
            last_toggle_time: Instant::now(),
        }
    }

    pub fn handle_input(&mut self, keys: &[Key], player: &mut Player) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        if keys.contains(&Key::W) {
            // Move forward in the direction the player is facing
            let angle_rad = player.angle.to_radians();
            let dx = angle_rad.cos() * PLAYER_SPEED * delta_time;
            let dy = angle_rad.sin() * PLAYER_SPEED * delta_time;
            player.move_x(dx);
            player.move_y(dy);
        }
        if keys.contains(&Key::S) {
            // Move backward in the opposite direction the player is facing
            let angle_rad = player.angle.to_radians();
            let dx = -angle_rad.cos() * PLAYER_SPEED * delta_time;
            let dy = -angle_rad.sin() * PLAYER_SPEED * delta_time;
            player.move_x(dx);
            player.move_y(dy);
        }
        if keys.contains(&Key::A) {
            // Move left, perpendicular to the direction the player is facing
            let angle_rad = (player.angle - 90.0).to_radians();
            let dx = angle_rad.cos() * PLAYER_SPEED * delta_time;
            let dy = angle_rad.sin() * PLAYER_SPEED * delta_time;
            player.move_x(dx);
            player.move_y(dy);
        }
        if keys.contains(&Key::D) {
            // Move right, perpendicular to the direction the player is facing
            let angle_rad = (player.angle + 90.0).to_radians();
            let dx = angle_rad.cos() * PLAYER_SPEED * delta_time;
            let dy = angle_rad.sin() * PLAYER_SPEED * delta_time;
            player.move_x(dx);
            player.move_y(dy);
        }
        if keys.contains(&Key::Left) {
            // Rotate left
            player.rotate(-PLAYER_ROTATION_SPEED * delta_time);
        }
        if keys.contains(&Key::Right) {
            // Rotate right
            player.rotate(PLAYER_ROTATION_SPEED * delta_time);
        }
        if keys.contains(&Key::M) {
            // Toggle map rendering
            if now.duration_since(self.last_toggle_time).as_millis() > 200 {
                self.render_map = !self.render_map;
                println!("Map rendering: {}", self.render_map);
                self.last_toggle_time = now;
            }
        }
        if keys.contains(&Key::Q) {
            panic!("Quitting the Game");
        }
    }
}
