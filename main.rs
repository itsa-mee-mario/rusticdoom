mod game;
mod render;
mod wad_reader;
use game::Game;
use game::Player;
use minifb::{Key, Window, WindowOptions};
use render::{render_raw, HEIGHT, WIDTH};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use wad_reader::DoomEngine;
use wad_reader::WadData;

// Structure to hold game state
struct GameState {
    buffer: Vec<u32>,
    should_exit: bool,
    world_objects: Vec<(f32, f32)>,
}

fn main() {
    let mut doomengine = DoomEngine::new("wad/doom1.wad");
    println!("Loading WAD file: {}", doomengine.wad_path);
    let wad = doomengine.load_wad().unwrap();
    let wad_data = WadData::new(doomengine);
    let world_objects = wad_data.read_vertexes().unwrap();

    // Shared game state
    let game_state = Arc::new(Mutex::new(GameState {
        buffer: vec![0; WIDTH * HEIGHT],
        should_exit: false,
        world_objects: world_objects.clone(),
    }));

    let mut window = Window::new(
        "This will become a game!",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));
    window.set_target_fps(60);

    // Shared game objects
    let game = Arc::new(Mutex::new(Game::new()));
    let player = Arc::new(Mutex::new(Player::new()));

    // Channels for communication
    let (input_tx, input_rx) = mpsc::channel::<Vec<Key>>();
    let (render_tx, render_rx) = mpsc::channel::<()>();

    // Clone references for threads
    let game_clone = Arc::clone(&game);
    let player_clone = Arc::clone(&player);
    let game_state_clone = Arc::clone(&game_state);

    // Input handling thread
    let input_thread = thread::spawn(move || {
        while !game_state_clone.lock().unwrap().should_exit {
            if let Ok(keys) = input_rx.try_recv() {
                let mut game = game_clone.lock().unwrap();
                let mut player = player_clone.lock().unwrap();
                game.handle_input(&keys, &mut player);
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Render thread
    let render_thread = {
        let player = Arc::clone(&player);
        let game_state = Arc::clone(&game_state);
        let game = Arc::clone(&game);

        thread::spawn(move || {
            while !game_state.lock().unwrap().should_exit {
                if let Ok(()) = render_rx.try_recv() {
                    let mut state = game_state.lock().unwrap();
                    let game = game.lock().unwrap();
                    let player = player.lock().unwrap();

                    // Clear buffer
                    for i in state.buffer.iter_mut() {
                        *i = 0x000000;
                    }

                    if game.render_map {
                        // Render WAD vertices when map rendering is enabled
                        let mut world_objects_copy = state.world_objects.clone();
                        render_raw(&mut state.buffer, &mut world_objects_copy);
                    }
                }
                thread::sleep(Duration::from_millis(1));
            }
        })
    };

    // Main game loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let keys = window.get_keys();

        input_tx.send(keys).unwrap();

        // Trigger render
        render_tx.send(()).unwrap();

        // Update window with the latest buffer
        {
            let state = game_state.lock().unwrap();
            window
                .update_with_buffer(&state.buffer, WIDTH, HEIGHT)
                .unwrap();
        }

        thread::sleep(Duration::from_millis(16)); // Cap at ~60 FPS
    }

    // Cleanup
    {
        let mut state = game_state.lock().unwrap();
        state.should_exit = true;
    }

    // Wait for threads to finish
    input_thread.join().unwrap();
    render_thread.join().unwrap();
}
