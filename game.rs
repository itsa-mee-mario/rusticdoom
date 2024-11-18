use minifb::Key;
use std::time::Instant;

pub struct Game {
    last_update: Instant,
    pub render_map: bool,
    last_toggle_time: Instant,
}

const PLAYER_SPEED: f32 = 100.0; // Pixels per second
const PLAYER_ROTATION_SPEED: f32 = 180.0; // Degrees per second

pub struct BoundedFloat {
    value: f32,
    min: f32,
    max: f32,
}

impl BoundedFloat {
    pub fn new(value: f32, min: f32, max: f32) -> Self {
        BoundedFloat { value, min, max }
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn add(&mut self, amount: f32) {
        self.value = (self.value + amount).clamp(self.min, self.max);
    }
}

impl std::fmt::Display for BoundedFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct Player {
    pub x: BoundedFloat,
    pub y: BoundedFloat,
    pub angle: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: BoundedFloat::new(0., -10000., 10000.),
            y: BoundedFloat::new(0., -10000., 10000.),
            angle: 0.0,
        }
    }

    pub fn move_x(&mut self, delta: f32) {
        self.x.add(delta);
    }

    pub fn move_y(&mut self, delta: f32) {
        self.y.add(delta);
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
            player.move_y(PLAYER_SPEED * delta_time);
            println!(
                "W is pressed. Player y increased to: {}",
                player.y.get_value()
            );
        }
        if keys.contains(&Key::A) {
            player.move_x(-PLAYER_SPEED * delta_time);
            println!(
                "A is pressed. Player x decreased to: {}",
                player.x.get_value()
            );
        }
        if keys.contains(&Key::S) {
            player.move_y(-PLAYER_SPEED * delta_time);
            println!(
                "S is pressed. Player y decreased to: {}",
                player.y.get_value()
            );
        }
        if keys.contains(&Key::D) {
            player.move_x(PLAYER_SPEED * delta_time);
            println!(
                "D is pressed. Player x increased to: {}",
                player.x.get_value()
            );
        }
        if keys.contains(&Key::Left) {
            player.rotate(-PLAYER_ROTATION_SPEED * delta_time);
            println!("Left is pressed. Player angle: {}", player.angle);
        }
        if keys.contains(&Key::Right) {
            player.rotate(PLAYER_ROTATION_SPEED * delta_time);
            println!("Right is pressed. Player angle: {}", player.angle);
        }
        if keys.contains(&Key::M) {
            if now.duration_since(self.last_toggle_time).as_millis() > 200 {
                // Toggle map rendering when 'M' is pressed
                self.render_map = !self.render_map;
                println!("Map rendering: {}", self.render_map);
                self.last_toggle_time = now;
            }
        }
    }
}
