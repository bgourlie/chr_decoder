extern crate sdl2;

#[macro_use]
extern crate glium;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use glium::{DisplayBuild, Surface};
use glium::texture::RawImage2d;
use glium::glutin;

use std::fs::File;
use std::io::Read;
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;
pub const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

#[derive(Copy, Clone)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r: r, g: g, b: b }
    }
}

pub trait Screen {
    fn put_pixel(&mut self, x: usize, y: usize, color: Rgb);

    fn print_tile(&mut self, tiles: &[u8], tile_index: usize) {
        let plane0_start_index = tile_index * 8;
        let plane0_end_index = plane0_start_index + 8;
        let plane1_start_index = plane0_end_index;
        let plane1_end_index = plane1_start_index + 8;

        let plane0 = &tiles[plane0_start_index..plane0_end_index];
        let plane1 = &tiles[plane1_start_index..plane1_end_index];

        for line in 0..8 {
            for pixel in 0..8 {
                let val = compute_color_index(plane0[line], plane1[line], pixel);
                let color = match val {
                    0 => Rgb::new(0, 0, 0),
                    1 => Rgb::new(64, 64, 64),
                    2 => Rgb::new(128, 128, 128),
                    3 => Rgb::new(255, 255, 255),
                    _ => {
                        panic!("shouldn't get here");
                    }
                };

                self.put_pixel((tile_index * 8) + pixel as usize, line as usize, color);
            }
        }
    }
}

struct ScreenSdl {
    buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
}

impl ScreenSdl {
    fn new() -> Self {
        ScreenSdl { buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3] }
    }
}

impl Screen for ScreenSdl {
    fn put_pixel(&mut self, x: usize, y: usize, color: Rgb) {
        self.buffer[(y * SCREEN_WIDTH + x) * 3] = color.r;
        self.buffer[(y * SCREEN_WIDTH + x) * 3 + 1] = color.g;
        self.buffer[(y * SCREEN_WIDTH + x) * 3 + 2] = color.b;
    }
}

struct ScreenGlium {
    buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
}

impl ScreenGlium {
    fn new() -> Self {
        ScreenGlium { buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 4] }
    }
}

impl Screen for ScreenGlium {
    fn put_pixel(&mut self, x: usize, y: usize, color: Rgb) {
        self.buffer[(y * SCREEN_WIDTH + x) * 4] = 255;
        self.buffer[(y * SCREEN_WIDTH + x) * 4 + 1] = color.r;
        self.buffer[(y * SCREEN_WIDTH + x) * 4 + 2] = color.g;
        self.buffer[(y * SCREEN_WIDTH + x) * 4 + 3] = color.b;
    }
}

fn main() {
    render_sdl();
}

fn get_chr_bytes() -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut f = File::open("chr.bin").unwrap();
    f.read_to_end(&mut buf);
    if buf.len() % 16 != 0 {
        panic!("Invalid CHR data.");
    }

    buf
}

fn populate_screen(screen: &mut Screen, buf: &Vec<u8>) {
    for n in 0..16 {
        screen.print_tile(&buf, n);
    }
}

// fn render_glium() {
//    let chr_bytes = get_chr_bytes();
//    let screen = ScreenGlium::new();
//
//    let display = glutin::WindowBuilder::new()
//        .with_vsync()
//        .build_glium()
//        .unwrap();
//
//    let dimensions = ((SCREEN_WIDTH * 3) as u32, (SCREEN_HEIGHT * 3) as u32);
//    let screen2 = RawImage2d::from_raw_rgba_reversed(&screen.buffer, dimensions);
//
// }

fn render_sdl() {
    let chr_bytes = get_chr_bytes();
    let mut screen = ScreenSdl::new();
    populate_screen(&mut screen, &chr_bytes);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video",
                SCREEN_WIDTH as u32 * 3,
                SCREEN_HEIGHT as u32 * 3)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().accelerated().present_vsync().build().unwrap();

    let mut texture = renderer.create_texture_streaming(PixelFormatEnum::BGR24,
                                  SCREEN_WIDTH as u32,
                                  SCREEN_HEIGHT as u32)
        .unwrap();

    texture.update(None, &screen.buffer, SCREEN_WIDTH * 3).unwrap();
    renderer.clear();
    renderer.copy(&texture, None, None);
    renderer.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }
}

fn compute_color_index(plane0: u8, plane1: u8, pixel_index: u8) -> u8 {
    let bit0 = (plane0 >> (7 - (pixel_index % 8))) & 0x1;
    let bit1 = (plane1 >> (7 - (pixel_index % 8))) & 0x1;
    (bit1 << 1) | bit0
}
