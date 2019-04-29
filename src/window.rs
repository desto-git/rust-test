use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use types::Dimension;

pub struct Window {
	scale: u8,

	sdl_window: sdl2::video::Window,
}

impl Window {
	pub fn new( sdl_context: &Sdl, title: &'static str, size: Dimension<u16>, scale: u8 ) -> Window {
		let sdl_video = sdl_context.video().unwrap();
		let physical_size = (
			size.w as u32 * scale as u32,
			size.h as u32 * scale as u32,
		);
		let sdl_window = sdl_video.window(title, physical_size.0, physical_size.1).position_centered().build().unwrap();

		Window {
			scale: scale,
			sdl_window: sdl_window,
		}
	}

	pub fn get_canvas( self ) -> WindowCanvas {
		let mut sdl_canvas = self.sdl_window.into_canvas().accelerated().build().unwrap();
		sdl_canvas.set_draw_color(Color::RGB(0,0,0));
		sdl_canvas.set_scale(self.scale as f32, self.scale as f32).unwrap();
		sdl_canvas
	}
}