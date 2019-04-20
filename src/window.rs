use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

pub struct Window {
	title: &'static str,
	size: (u16, u16),

	sdl_window: sdl2::video::Window,
}

impl Window {
	pub fn new( sdl_context: &Sdl, title: &'static str, size: (u16, u16) ) -> Window {
		let sdl_video = sdl_context.video().unwrap();
		let sdl_window = sdl_video.window(title, size.0 as u32, size.1 as u32).position_centered().build().unwrap();

		Window {
			title: title,
			size: size,
			sdl_window: sdl_window,
		}
	}

	pub fn get_canvas( self ) -> WindowCanvas {
		let mut sdl_canvas = self.sdl_window.into_canvas().accelerated().build().unwrap();
		sdl_canvas.set_draw_color(Color::RGB(0,0,0));
		sdl_canvas
	}
}