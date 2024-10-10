// main.rs
mod game;

mod render;

use game::Game;
use game::Player;
use minifb::{Key, Window, WindowOptions};
use render::{render, HEIGHT, WIDTH};

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; // Screen buffer
    let mut window = Window::new(
        "This will become a game!",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(20);

    let game = Game::new();
    let mut player = Player::new();

    // Main game loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // handle input and update player position
        let keys = window.get_keys();
        game.handle_input(&keys, &mut player);

        // render
        render(&mut buffer, player.x.get_value(), player.y.get_value(), player.angle);

        // update window with the new buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
