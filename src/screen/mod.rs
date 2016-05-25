use consts::{SCREEN_WIDTH, SCREEN_HEIGHT};
use nes_gfx::Rgb;

pub trait Screen {
    fn put_pixel(&mut self, x: usize, y: usize, color: Rgb);
}

pub struct ScreenSdl {
    pub buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
}

impl ScreenSdl {
    pub fn new() -> Self {
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

pub struct ScreenGlium {
    pub buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
}

impl ScreenGlium {
    pub fn new() -> Self {
        ScreenGlium { buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 4] }
    }
}

impl Screen for ScreenGlium {
    fn put_pixel(&mut self, x: usize, y: usize, color: Rgb) {
        self.buffer[(y * SCREEN_WIDTH + x) * 4] = color.b;
        self.buffer[(y * SCREEN_WIDTH + x) * 4 + 1] = color.g;
        self.buffer[(y * SCREEN_WIDTH + x) * 4 + 2] = color.r;
        self.buffer[(y * SCREEN_WIDTH + x) * 4 + 3] = 255;
    }
}
