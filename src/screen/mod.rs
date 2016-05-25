use consts::{SCREEN_WIDTH, SCREEN_HEIGHT};
use nes_gfx::Rgb;

pub trait Screen {
    fn put_pixel(&mut self, x: usize, y: usize, color: Rgb);
}

pub struct ScreenBgr {
    pub buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
}

impl ScreenBgr {
    pub fn new() -> Self {
        ScreenBgr { buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3] }
    }
}

impl Screen for ScreenBgr {
    fn put_pixel(&mut self, x: usize, y: usize, color: Rgb) {
        self.buffer[(y * SCREEN_WIDTH + x) * 3] = color.r;
        self.buffer[(y * SCREEN_WIDTH + x) * 3 + 1] = color.g;
        self.buffer[(y * SCREEN_WIDTH + x) * 3 + 2] = color.b;
    }
}

pub struct ScreenRgba {
    pub buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
}

impl ScreenRgba {
    pub fn new() -> Self {
        ScreenRgba { buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 4] }
    }
}

impl Screen for ScreenRgba {
    fn put_pixel(&mut self, x: usize, y: usize, color: Rgb) {
        self.buffer[(y * SCREEN_WIDTH + x) * 4] = color.b;
        self.buffer[(y * SCREEN_WIDTH + x) * 4 + 1] = color.g;
        self.buffer[(y * SCREEN_WIDTH + x) * 4 + 2] = color.r;
        self.buffer[(y * SCREEN_WIDTH + x) * 4 + 3] = 255;
    }
}
