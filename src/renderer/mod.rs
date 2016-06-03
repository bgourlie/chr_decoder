use sdl2;
use sdl2::pixels::PixelFormatEnum;
use screen::Screen;

pub trait Renderer {
    fn render(&mut self, screen: &Screen);
}

pub struct SdlRenderer<'a> {
    renderer: sdl2::render::Renderer<'a>,
    texture: sdl2::render::Texture,
    texture_width: u32,
    texture_height: u32,
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

        let texture_query = texture.query();

        SdlRenderer {
            renderer: renderer,
            texture: texture,
            texture_width: texture_query.width,
            texture_height: texture_query.height,
        }
    }
}

impl<'a> Renderer for SdlRenderer<'a> {
    fn render(&mut self, screen: &Screen) {
        // pitch is the number of bytes in a row of pixel data, including padding between lines
        let pitch = self.texture_width as usize * 3 as usize;
        self.texture.update(None, &screen.get_buffer(), pitch).unwrap();
        self.renderer.clear();
        self.renderer.copy(&self.texture, None, None);
        self.renderer.present();
    }
}
