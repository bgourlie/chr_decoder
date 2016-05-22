extern crate sdl2;
extern crate clock_ticks;

#[macro_use]
extern crate glium;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use glium::{DisplayBuild, Surface};
use glium::glutin;

use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::Read;

mod consts;
mod screen;

use consts::{SCREEN_WIDTH, SCREEN_HEIGHT};
use screen::*;

fn main() {
    render_glium();
    render_sdl();
}

fn fill_screen(screen: &mut Screen, buf: &Vec<u8>) {
    for tile_index in 0..16 {
        let plane0_start_index = tile_index * 8;
        let plane0_end_index = plane0_start_index + 8;
        let plane1_start_index = plane0_end_index;
        let plane1_end_index = plane1_start_index + 8;

        let plane0 = &buf[plane0_start_index..plane0_end_index];
        let plane1 = &buf[plane1_start_index..plane1_end_index];

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

                screen.put_pixel((tile_index * 8) + pixel as usize, line as usize, color);
            }
        }
    }
}

fn get_chr_bytes() -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut f = File::open("chr.bin").unwrap();
    f.read_to_end(&mut buf).unwrap();

    if buf.len() % 16 != 0 {
        panic!("Invalid CHR data.");
    }

    buf
}

fn render_glium() {
    let chr_bytes = get_chr_bytes();
    let mut screen = ScreenGlium::new();
    fill_screen(&mut screen, &chr_bytes);

    let display = glutin::WindowBuilder::new()
        .with_vsync()
        .build_glium()
        .unwrap();

    let dimensions = (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    let screen = glium::texture::RawImage2d::from_raw_rgba_reversed(screen.buffer[..].to_vec(),
                                                                    dimensions);
    let screen = glium::Texture2d::new(&display, screen).unwrap();

    start_loop(|| {
        let target = display.draw();
        screen.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return Action::Stop,
                _ => {}
            }
        }

        Action::Continue
    });

}

fn render_sdl() {
    let chr_bytes = get_chr_bytes();
    let mut screen = ScreenSdl::new();
    fill_screen(&mut screen, &chr_bytes);
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

    start_loop(|| {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Action::Stop,
                _ => {}
            }
        }

        Action::Continue
    });
}

fn compute_color_index(plane0: u8, plane1: u8, pixel_index: u8) -> u8 {
    let bit0 = (plane0 >> (7 - (pixel_index % 8))) & 0x1;
    let bit1 = (plane1 >> (7 - (pixel_index % 8))) & 0x1;
    (bit1 << 1) | bit0
}

pub fn start_loop<F>(mut callback: F)
    where F: FnMut() -> Action
{
    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => (),
        };

        let now = clock_ticks::precise_time_ns();
        accumulator += now - previous_clock;
        previous_clock = now;

        const FIXED_TIME_STAMP: u64 = 16666667;
        while accumulator >= FIXED_TIME_STAMP {
            accumulator -= FIXED_TIME_STAMP;

            // if you have a game, update the state here
        }

        thread::sleep(Duration::from_millis(((FIXED_TIME_STAMP - accumulator) / 1000000) as u64));
    }
}

pub enum Action {
    Stop,
    Continue,
}
