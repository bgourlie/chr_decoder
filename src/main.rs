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

mod renderer;
mod screen;
mod nes_gfx;

use screen::*;
use nes_gfx::{Rgb, ChrData, NesPixel};
use renderer::*;

fn main() {
    render_glium();
    render_sdl();
}

#[allow(dead_code)]
fn print_palette(screen: &mut Screen) {
    for color_index in 0..64 {
        let color = nes_gfx::PALETTE[color_index];
        let y_start = color_index * 3;
        let y_end = y_start + 3;
        for y in y_start..y_end + 1 {
            for x in 0..SCREEN_WIDTH {
                screen.put_pixel(x as usize, y as usize, color);
            }
        }
    }
}

fn fill_screen(screen: &mut Screen, chr: &ChrData) {
    let mut cur_line = -1;
        for pixel in chr.pixels {

            let color = match pixel {
                NesPixel::Background => Rgb::new(32, 32, 32),
                NesPixel::Palette1 => Rgb::new(64, 64, 64),
                NesPixel::Palette2 => Rgb::new(128, 128, 128),
                NesPixel::Palette3 => Rgb::new(255, 255, 255),
                _ => {
                    panic!("shouldn't get here");
                }
            };

            screen.put_pixel((tile_index * 8) + x as usize,
                             y + (cur_line * 8) as usize,
                             color);
        }
}

fn render_glium() {
    let chr = nes_gfx::ChrData::from_raw_file("chr.bin");
    let mut screen = ScreenRgba::new();
    fill_screen(&mut screen, &chr);
    // print_palette(&mut screen);

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
    let chr = nes_gfx::ChrData::from_raw_file("chr.bin");
    let mut screen = ScreenBgr::new();
    fill_screen(&mut screen, &chr);
    let sdl_context = sdl2::init().unwrap();
    let mut renderer =
        renderer::SdlRenderer::new(&sdl_context, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    let mut event_pump = sdl_context.event_pump().unwrap();

    start_loop(|| {
        renderer.render(&screen);
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
