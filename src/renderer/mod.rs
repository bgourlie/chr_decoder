use sdl2;
use sdl2::pixels::PixelFormatEnum;
// use glium::texture::RawImage2d;
// use glium::glutin::{WindowBuilder, Window};
// use glium::{Texture2d, DisplayBuild, Surface};
// use glium::framebuffer::SimpleFrameBuffer;
// use glium::glutin;
// use glium::backend::glutin_backend::GlutinFacade;
use screen::Screen;

pub trait Renderer {
    fn render(&mut self, screen: &Screen);
}

pub struct SdlRenderer<'a> {
    renderer: sdl2::render::Renderer<'a>,
    texture: sdl2::render::Texture,
    pitch: usize,
}

impl<'a> SdlRenderer<'a> {
    pub fn new(sdl_context: &sdl2::Sdl, screen_width: u32, screen_height: u32) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window =
            video_subsystem.window("rust-sdl2 demo: Video", screen_width * 3, screen_height * 3)
                .position_centered()
                .opengl()
                .build()
                .unwrap();

        let renderer = window.renderer().accelerated().present_vsync().build().unwrap();

        let texture =
            renderer.create_texture_streaming(PixelFormatEnum::BGR24, screen_width, screen_height)
                .unwrap();

        // pitch is the number of bytes in a row of pixel data, including padding between lines
        let pitch = texture.query().width as usize * 3 as usize;

        SdlRenderer {
            renderer: renderer,
            texture: texture,
            pitch: pitch,
        }
    }
}

impl<'a> Renderer for SdlRenderer<'a> {
    fn render(&mut self, screen: &Screen) {
        self.texture.update(None, &screen.get_buffer(), self.pitch).unwrap();
        self.renderer.clear();
        self.renderer.copy(&self.texture, None, None);
        self.renderer.present();
    }
}

// pub struct GliumRenderer {
//    window: GlutinFacade,
//    frame_buffer: SimpleFrameBuffer
// }
//
// impl GliumRenderer {
//    pub fn new(screen: &Screen, screen_width: u32, screen_height: u32) -> Self {
//
//        let display = WindowBuilder::new()
//        .with_vsync()
//        .build_glium()
//        .unwrap();
//
//        let dimensions = (screen_width, screen_height);
//        let screen_texture = RawImage2d::from_raw_rgba_reversed(screen.get_buffer().to_vec(),
//                                                                        dimensions);
//        let screen_texture = Texture2d::new(&display, screen_texture).unwrap();
//
//        let frame_buffer = screen_texture.as_surface();
//
//        GliumRenderer {
//            window: window,
//            screen_texture: screen_texture
//        }
//    }
// }
//
// impl Renderer for GliumRenderer {
//   fn render(&mut self, screen: &Screen) {
//
//   }
// }
