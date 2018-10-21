extern crate sdl2;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
	let screen_size = ( 320, 180 );

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem.window("SDL2", screen_size.0, screen_size.1).position_centered().build().unwrap();
	let mut canvas = window.into_canvas().accelerated().build().unwrap();
	let texture_creator = canvas.texture_creator();

	canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

	let mut timer = sdl_context.timer().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();

	let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/spritesheet.bmp")).unwrap();
	let texture = texture_creator.create_texture_from_surface(&temp_surface).unwrap();

	let frames_per_anim = 8;
	let sprite_tile_size = ( 4, 4 );
	let sprite_tile_position = (sprite_tile_size.0 as i32, sprite_tile_size.1 as i32);
	let scale = 4;

	let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.1);
	let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0 * scale, sprite_tile_size.0 * scale);

	let mut source_rect_1 = Rect::new(0, sprite_tile_position.1, sprite_tile_size.0, sprite_tile_size.1);
	let mut dest_rect_1 = Rect::new(0, sprite_tile_position.1 * scale as i32, sprite_tile_size.0 * scale, sprite_tile_size.0 * scale);

	let mut running = true;
	while running {
		for event in event_pump.poll_iter() {
			match event {
					Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
						running = false;
					},
					_ => {}
			}
		}

		let ticks = timer.ticks() as i32;
		let wait = 100;

		// set the current frame for time
		source_rect_0.set_x(sprite_tile_position.0 * ((ticks / wait) % frames_per_anim));
		dest_rect_0.set_x(1 * ((ticks / 10) % (screen_size.0 + sprite_tile_size.0) as i32));

		source_rect_1.set_x(sprite_tile_position.0 * ((ticks / wait) % frames_per_anim));
		dest_rect_1.set_x(1 * ((ticks / 10) % (screen_size.0 + sprite_tile_size.0) as i32));

		canvas.clear();
		// copy the frame to the canvas
		canvas.copy_ex(&texture, Some(source_rect_0), Some(dest_rect_0), 0.0, None, false, false).unwrap();
		canvas.copy_ex(&texture, Some(source_rect_1), Some(dest_rect_1), 0.0, None, false, false).unwrap();
		canvas.present();

		std::thread::sleep(Duration::from_millis(wait as u64));
	}
}