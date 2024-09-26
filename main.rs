use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn pixel(x: usize, y: usize, c: usize, buffer: &mut Vec<u32>) {
    let temp;
    match c {
        0 => temp = 0x000000,
        1 => temp = 0xFFFFFF,
        _ => temp = 0xFF0000,
    }
    buffer[y * WIDTH + x] = temp;
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; // this is the screen buffer

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                pixel(x, y, (x + y) % 100, &mut buffer);
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
