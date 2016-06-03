use std::fs::File;
use std::io::Read;

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

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r: r, g: g, b: b }
    }
}

pub fn compute_color_index(plane0: u8, plane1: u8, pixel_index: u8) -> u8 {
    let bit0 = (plane0 >> (7 - (pixel_index % 8))) & 0x1;
    let bit1 = (plane1 >> (7 - (pixel_index % 8))) & 0x1;
    (bit1 << 1) | bit0
}

pub fn read_chr(file_name: &'static str) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut f = File::open(file_name).unwrap();
    f.read_to_end(&mut buf).unwrap();

    if buf.len() % 16 != 0 {
        panic!("Invalid CHR data.");
    }

    buf
}
