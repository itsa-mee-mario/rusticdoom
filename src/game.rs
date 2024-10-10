use minifb::Key;

// use std::ops::{AddAssign, SubAssign}; // unused

pub struct Game;

const PLAYER_SPEED: f32 = 2.;
const PLAYER_ROTATION_SPEED: f32 = 10.;
// const PLAYER_MAX_SPEED: f32 = 10.; // unused

pub struct BoundedFloat {
    value: f32,
    min: f32,
    max: f32,
}

impl BoundedFloat {
    pub fn new(value: f32, min: f32, max: f32) -> Self {
        BoundedFloat { value, min, max }
    }
    
    // unused
    // fn check_bounds(&mut self) {
    //     if self.value < self.min {
    //         self.value = self.min;
    //     }
    //     if self.value > PLAYER_MAX_SPEED {
    //         self.value = PLAYER_MAX_SPEED;
    //     }
    // }

    pub fn get_value(&self) -> f32 {
        self.value
    }

}

impl std::ops::AddAssign<f32> for BoundedFloat {
    fn add_assign(&mut self, other: f32) {
        self.value = (self.value + other).clamp(self.min, self.max);
    }
}

impl std::ops::SubAssign<f32> for BoundedFloat {
    fn sub_assign(&mut self, other: f32) {
        self.value = (self.value - other).clamp(self.min, self.max);
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
            x: BoundedFloat::new(0., -10., 10.),
            y: BoundedFloat::new(0., -10., 10.),
            angle: 0.0,
        }
    }

    pub fn increase_x(&mut self) {
        self.x += PLAYER_SPEED;
    }
    pub fn decrease_x(&mut self) {
        self.x -= PLAYER_SPEED;
    }
    pub fn increase_y(&mut self) {
        self.y += PLAYER_SPEED;
    }
    pub fn decrease_y(&mut self) {
        self.y -= PLAYER_SPEED;
    }

    pub fn rotate_left(&mut self) {
        self.angle -= PLAYER_ROTATION_SPEED;
        self.angle %= 360.;
    }

    pub fn rotate_right(&mut self) {
        self.angle += PLAYER_ROTATION_SPEED;
        self.angle %= 360.;
    }
}

impl Game {
    pub fn new() -> Self {
        Game
    }

    pub fn handle_input(&self, keys: &[Key], player: &mut Player) {
        if keys.contains(&Key::W) {
            player.increase_y();
            println!("W is pressed. Player y increased to: {}", player.y.get_value());
        }
        if keys.contains(&Key::A) {
            player.decrease_x();
            println!("A is pressed. Player x decreased to: {}", player.x.get_value());
        }
        if keys.contains(&Key::S) {
            player.decrease_y();
            println!("S is pressed. Player y decreased to: {}", player.y.get_value());
        }
        if keys.contains(&Key::D) {
            player.increase_x();
            println!("D is pressed. Player x increased to: {}", player.x.get_value());
        }
        if keys.contains(&Key::Left) {
            player.rotate_left();
            println!("Left is pressed. Player angle: {}", player.angle);
        }
        if keys.contains(&Key::Right) {
            player.rotate_right();
            println!("Right is pressed. Player angle: {}", player.angle);
        }
    }    
    
}
