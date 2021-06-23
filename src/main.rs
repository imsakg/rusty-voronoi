use rand::Rng;
use std::{fs::File, io::Write, vec};

#[allow(
    unused,
    clippy::enum_clike_unportable_variant,
    unused_imports,
    dead_code,
    non_snake_case
)]
#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
enum Color {
    Red = 0xFF0000,
    Green = 0x00FF00,
    Blue = 0x0000FF,
    White = 0xFFFFFF,
    Black = 0x000000,
    Yellow = 0xFFFF00,
    Cyan = 0x00FFFF,
    Magenta = 0xFF00FF,
    ORANGE = 0xFFA500,
    PURPLE = 0x800080,
    BROWN = 0xA52A2A,
    PINK = 0xFFC0CB,
    GREY = 0x808080,
    GOLD = 0xFFD700,
    SILVER = 0xC0C0C0,
    BACKGROUND_COLOR = 0x181818,
    GRUVBOX_BRIGHT_RED = 0xFB4934,
    GRUVBOX_BRIGHT_GREEN = 0xB8BB26,
    GRUVBOX_BRIGHT_YELLOW = 0xFABD2F,
    GRUVBOX_BRIGHT_BLUE = 0x83A598,
    GRUVBOX_BRIGHT_PURPLE = 0xD3869B,
    GRUVBOX_BRIGHT_AQUA = 0x8EC07C,
    GRUVBOX_BRIGHT_ORANGE = 0xFE8019,
    GRUVBOX_DARK_RED = 0xCC241D,
    GRUVBOX_DARK_GREEN = 0x98971A,
    GRUVBOX_DARK_YELLOW = 0xD79921,
    GRUVBOX_DARK_BLUE = 0x458588,
    GRUVBOX_DARK_PURPLE = 0xB16286,
    GRUVBOX_DARK_AQUA = 0x689D6A,
    GRUVBOX_DARK_ORANGE = 0xD65D0E,
}

type Image = [[u32; WIDTH]; HEIGHT];
type Seeds = [Point; SEEDS_COUNT];
type Palette = [Color; PALETTE_SIZE];

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const FILENAME: &str = "image.ppm";
const SEEDS_COUNT: usize = 10;
const SEEDS_MARKER_RADIUS: usize = 10;
const SEEDS_MARKER_COLOR: Color = Color::Black;
const PALETTE_SIZE: usize = 7;

fn main() {
    let mut image = [[Color::BACKGROUND_COLOR as u32; WIDTH]; HEIGHT];
    let mut seeds = [Point { x: 0, y: 0 }; SEEDS_COUNT];
    let Palette = [
        Color::GRUVBOX_BRIGHT_RED,
        Color::GRUVBOX_BRIGHT_GREEN,
        Color::GRUVBOX_BRIGHT_YELLOW,
        Color::GRUVBOX_BRIGHT_BLUE,
        Color::GRUVBOX_BRIGHT_PURPLE,
        Color::GRUVBOX_BRIGHT_AQUA,
        Color::GRUVBOX_BRIGHT_ORANGE,
    ];

    fill_image(&mut image, Color::BACKGROUND_COLOR);
    generate_random_seeds(&mut seeds);
    render_varonoi(&mut image, &seeds, &Palette);
    render_seed_markers(&mut image, &seeds);
    save_image_as_ppm(&image, FILENAME);
}

fn render_varonoi(image: &mut Image, seeds: &Seeds, palette: &Palette) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut j = 0;
            for i in 1..SEEDS_COUNT {
                if sqr_dst(seeds[i].x, seeds[i].y, x, y) < sqr_dst(seeds[j].x, seeds[j].y, x, y) {
                    j = i;
                }
            }
            image[y][x] = palette[j % PALETTE_SIZE] as u32;
        }
    }
}

fn render_seed_markers(image: &mut Image, seeds: &Seeds) {
    for seed in seeds.iter() {
        fill_circle(
            seed.x,
            seed.y,
            SEEDS_MARKER_RADIUS,
            SEEDS_MARKER_COLOR,
            image,
        );
    }
}

fn fill_circle(cx: usize, cy: usize, radius: usize, color: Color, image: &mut Image) {
    // .......
    // ..***..
    // ..*@*..
    // ..***..
    // .......

    let x0 = cx - radius;
    let y0 = cy - radius;
    let x1 = cx + radius;
    let y1 = cy + radius;

    for x in x0..x1 {
        if (0..WIDTH).contains(&x) {
            for y in y0..y1 {
                if (0..HEIGHT).contains(&y) {
                    if (sqr_dst(cx, cy, x, y) <= radius * radius) {
                        image[y][x] = color as u32;
                    }
                }
            }
        }
    }
}

fn generate_random_seeds(seeds: &mut Seeds) {
    let mut rng = rand::thread_rng();
    for seed in seeds.iter_mut() {
        seed.x = rng.gen_range(0..WIDTH);
        seed.y = rng.gen_range(0..HEIGHT);
    }
}

fn save_image_as_ppm(image: &Image, filename: &str) {
    let mut file = File::create(filename).unwrap();

    let header = format!("P6\n{} {} 255\n", WIDTH, HEIGHT);
    let mut data: Vec<u8> = vec![];

    assert!(
        file.write_all(header.as_bytes()).is_ok(),
        "Failed to write header",
    );

    for row in image.iter() {
        for pixel in row.iter() {
            data.push(((pixel & 0xFF0000) >> 8 * 2) as u8);
            data.push(((pixel & 0x00FF00) >> 8 * 1) as u8);
            data.push(((pixel & 0x0000FF) >> 8 * 0) as u8);
        }
    }
    assert!(file.write_all(&data).is_ok(), "Error writing to file");
}

fn fill_image(image: &mut Image, color: Color) {
    for row in image.iter_mut() {
        for pixel in row.iter_mut() {
            *pixel = color as u32;
        }
    }
}

fn sqr_dst(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let dx = x1 - x2;
    let dy = y1 - y2;
    dx * dx + dy * dy
}
