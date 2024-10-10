use minifb::Key;

use std::ops::{AddAssign, SubAssign};

pub struct Game;

const PLAYER_SPEED: f32 = 2.;
const PLAYER_ROTATION_SPEED: f32 = 10.;
const PLAYER_MAX_SPEED: f32 = 10.;

struct BoundedFloat {
    value: f32,
    min: f32,
    max: f32,
}

impl BoundedFloat {
    pub fn new(value: f32, min: f32, max: f32) -> Self {
        BoundedFloat { value, min, max }
    }

    fn check_bounds(&mut self) {
        if self.value < self.min {
            self.value = self.min;
        }
        if self.value > PLAYER_MAX_SPEED {
            self.value = PLAYER_MAX_SPEED;
        }
    }
}

impl AddAssign<f32> for BoundedFloat {
    fn add_assign(&mut self, rhs: f32) {
        self.value += rhs;
        self.check_bounds();
    }
}

impl SubAssign<f32> for BoundedFloat {
    fn sub_assign(&mut self, rhs: f32) {
        self.value -= rhs;
        self.check_bounds();
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
            println!("W is pressed");
            player.increase_y();
        }
        if keys.contains(&Key::A) {
            println!("A is pressed");
            player.decrease_x();
        }
        if keys.contains(&Key::S) {
            println!("S is pressed");
            player.decrease_y();
        }
        if keys.contains(&Key::D) {
            println!("D is pressed");
            player.increase_x();
        }
        if keys.contains(&Key::Left) {
            println!("Left is pressed");
            player.rotate_left();
        }
        if keys.contains(&Key::Right) {
            println!("Right is pressed");
            player.rotate_right();
        }
        println!(
            "Player x: {}, y: {}, a:{}",
            player.x, player.y, player.angle
        );
    }
}
