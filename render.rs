pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 360;

pub fn pixel(x: usize, y: usize, c: usize, buffer: &mut Vec<u32>) {
    let temp = match c {
        0 => 0x000000,
        1 => 0xFFFFFF,
        _ => 0xFF0000,
    };
    buffer[y * WIDTH + x] = temp;
}

pub fn render(buffer: &mut Vec<u32>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            pixel(x, y, (x + y) % 100, buffer);
        }
    }
}
