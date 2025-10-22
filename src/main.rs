use minifb::{Window, WindowOptions};
use noise::NoiseFn;

const LOGICAL_WIDTH: usize = 756;
const LOGICAL_HEIGHT: usize = 565;
const SCALE: usize = 1;

const WIDTH: usize = LOGICAL_WIDTH * SCALE;
const HEIGHT: usize = LOGICAL_HEIGHT * SCALE;

fn main() {
    let mut window = Window::new(
        "terrain",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    let seed = 000;
    let perlin = noise::Perlin::new(seed);
    let mut terrain: Vec<[f64; LOGICAL_WIDTH]> = Vec::new();

    for i in 0..LOGICAL_HEIGHT {
        terrain.push(generate(&perlin, i as u32));
    }

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    for y in 0..LOGICAL_HEIGHT {
        for x in 0..LOGICAL_WIDTH {
            let color = match terrain[y][x] {
                0.0 => 0xFF00FF00, // gren
                1.0 => 0xFF0000FF, // blu
                _ => 0xFF000000,
            };

            for dy in 0..SCALE {
                for dx in 0..SCALE {
                    let px = x * SCALE + dx;
                    let py = y * SCALE + dy;
                    buffer[py * WIDTH + px] = color;
                }
            }
        }
    }

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn generate(perlin: &noise::Perlin, offset: u32) -> [f64; LOGICAL_WIDTH] {
    let mut result = [0.0; LOGICAL_WIDTH];
    for i in 0..LOGICAL_WIDTH {
        let x = i as f64 * 0.1;
        let y = 0.0;
        let z = offset as f64 * 0.1;

        let noise = perlin.get([x, y, z]);
        result[i] = if noise >= 0.0 { 0.0 } else { 1.0 };
    }
    result
}
