// main.rs
mod game;

mod render;

use game::Game;
use minifb::{Key, Window, WindowOptions};
use render::{render, HEIGHT, WIDTH};

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; // screen buffer
    let mut window = Window::new(
        "This will become a game!",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    let game = Game::new();

    let mut player = game::Player::new();

    // main game loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle input
        let keys = window.get_keys();
        game.handle_input(&keys, &mut player);

        // Render the frame
        render(&mut buffer);

        // Update window with the new buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
