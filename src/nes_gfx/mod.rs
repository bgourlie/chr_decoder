use std::fs::File;
use std::io::Read;
use std::ops::Deref;

const PRG_ROM_PAGE_SIZE: usize = 8192;

// This is kinda arbitrary at the moment.  The idea is to allocate enough to hold chr for any rom
const PRG_TILE_BUFFER_SIZE: usize = PRG_ROM_PAGE_SIZE / 16;

#[derive(Copy, Clone)]
enum NesPixel {
    Background,
    Palette1,
    Palette2,
    Palette3,
}

impl NesPixel {
    pub fn new(plane0: u8, plane1: u8, pixel_index: u8) -> Self {
        let bit0 = (plane0 >> (7 - (pixel_index % 8))) & 0x1;
        let bit1 = (plane1 >> (7 - (pixel_index % 8))) & 0x1;
        let index = (bit1 << 1) | bit0;

        match index {
            0 => NesPixel::Background,
            1 => NesPixel::Palette1,
            2 => NesPixel::Palette2,
            3 => NesPixel::Palette3,
            _ => panic!("Unexpected palette index"),
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub static PALETTE: [Rgb; 64] = [
    Rgb { r: 124, g: 124, b: 124},
    Rgb { r: 0, g: 0, b: 252},
    Rgb { r: 0, g: 0, b: 188},
    Rgb { r: 68, g: 40, b: 188},
    Rgb { r: 148, g: 0, b: 132},
    Rgb { r: 168, g: 0, b: 32},
    Rgb { r: 168, g: 16, b: 0},
    Rgb { r: 136, g: 20, b: 0},
    Rgb { r: 80, g: 48, b: 0},
    Rgb { r: 0, g: 120, b: 0},
    Rgb { r: 0, g: 104, b: 0},
    Rgb { r: 0, g: 88, b: 0},
    Rgb { r: 0, g: 64, b: 88},
    Rgb { r: 0, g: 0, b: 0},
    Rgb { r: 0, g: 0, b: 0},
    Rgb { r: 0, g: 0, b: 0},
    Rgb { r: 188, g: 188, b: 188},
    Rgb { r: 0, g: 120, b: 248},
    Rgb { r: 0, g: 88, b: 248},
    Rgb { r: 104, g: 68, b: 252},
    Rgb { r: 216, g: 0, b: 204},
    Rgb { r: 228, g: 0, b: 88},
    Rgb { r: 248, g: 56, b: 0},
    Rgb { r: 228, g: 92, b: 16},
    Rgb { r: 172, g: 124, b: 0},
    Rgb { r: 0, g: 184, b: 0},
    Rgb { r: 0, g: 168, b: 0},
    Rgb { r: 0, g: 168, b: 68},
    Rgb { r: 0, g: 136, b: 136},
    Rgb { r: 0, g: 0, b: 0},
    Rgb { r: 0, g: 0, b: 0},
    Rgb { r: 0, g: 0, b: 0},
    Rgb { r: 248, g: 248, b: 248},
    Rgb { r: 60, g: 188, b: 252},
    Rgb { r: 104, g: 136, b: 252},
    Rgb { r: 152, g: 120, b: 248},
    Rgb { r: 248, g: 120, b: 248},
    Rgb { r: 248, g: 88, b: 152},
    Rgb { r: 248, g: 120, b: 88},
    Rgb { r: 252, g: 160, b: 68},
    Rgb { r: 248, g: 184, b: 0},
    Rgb { r: 184, g: 248, b: 24},
    Rgb { r: 88, g: 216, b: 84},
    Rgb { r: 88, g: 248, b: 152},
    Rgb { r: 0, g: 232, b: 216},
    Rgb { r: 120, g: 120, b: 120},
    Rgb { r: 0, g: 0, b: 0},
    Rgb { r: 0, g: 0, b: 0},
    Rgb { r: 252, g: 252, b: 252},
    Rgb { r: 164, g: 228, b: 252},
    Rgb { r: 184, g: 184, b: 248},
    Rgb { r: 216, g: 184, b: 248},
    Rgb { r: 248, g: 184, b: 248},
    Rgb { r: 248, g: 164, b: 192},
    Rgb { r: 240, g: 208, b: 176},
    Rgb { r: 252, g: 224, b: 168},
    Rgb { r: 248, g: 216, b: 120},
    Rgb { r: 216, g: 248, b: 120},
    Rgb { r: 184, g: 248, b: 184},
    Rgb { r: 184, g: 248, b: 216},
    Rgb { r: 0, g: 252, b: 252},
    Rgb { r: 248, g: 216, b: 248},
    Rgb { r: 0, g: 0, b: 0},
    Rgb { r: 0, g: 0, b: 0}
];

#[derive(Copy, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct ChrData {
    pixels: [NesPixel; PRG_TILE_BUFFER_SIZE],
}

impl ChrData {
    pub fn from_raw_file(file_name: &'static str) -> ChrData {
        let mut buf: Vec<u8> = Vec::new();
        let mut f = File::open(file_name).unwrap();
        f.read_to_end(&mut buf).unwrap();

        if buf.len() % PRG_ROM_PAGE_SIZE != 0 {
            panic!("Invalid CHR data.");
        }

        let mut pixels: [NesPixel; PRG_TILE_BUFFER_SIZE] =
            [NesPixel::Background; PRG_TILE_BUFFER_SIZE];

        for tile_index in 0..buf.len() / 16 {
            let plane0_start_index = tile_index * 16;
            let plane0_end_index = plane0_start_index + 8;
            let plane1_start_index = plane0_end_index;
            let plane1_end_index = plane1_start_index + 8;

            let plane0 = &buf[plane0_start_index..plane0_end_index];
            let plane1 = &buf[plane1_start_index..plane1_end_index];

            for i in 0..16 {
                pixels[(tile_index * 16) + i] =
                    NesPixel::new(plane0[tile_index], plane1[tile_index], i as u8);
            }
        }

        ChrData { pixel: pixels }
    }
}

impl Deref for ChrData {
    type Target = [NesPixel; PRG_TILE_BUFFER_SIZE];

    fn deref(&self) -> &[NesPixel; PRG_TILE_BUFFER_SIZE] {
        &self.chr
    }
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r: r, g: g, b: b }
    }
}

// Delete and use ChrInfo impl
pub fn compute_color_index(plane0: u8, plane1: u8, pixel_index: u8) -> u8 {
    let bit0 = (plane0 >> (7 - (pixel_index % 8))) & 0x1;
    let bit1 = (plane1 >> (7 - (pixel_index % 8))) & 0x1;
    (bit1 << 1) | bit0
}
