extern crate sdl2;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
	let size_tile  : (u8, u8) = ( 4,  4);
	let size_world : (u8, u8) = (10, 10);
	let scale : u8 =   4;
	let wait  : u8 = 255;

	let dir_up    : (i8, i8) = ( 0, -1);
	let dir_down  : (i8, i8) = ( 0,  1);
	let dir_left  : (i8, i8) = (-1,  0);
	let dir_right : (i8, i8) = ( 1,  0);
	let mut direction = dir_right;

	let mut position : (i8, i8) = (0, 0);

	let sprite_head_up           : (u8, u8) = (0, 0);
	let sprite_head_down         : (u8, u8) = (0, 1);
	let sprite_head_left         : (u8, u8) = (1, 0);
	let sprite_head_right        : (u8, u8) = (1, 1);
	let sprite_body_horizontal   : (u8, u8) = (2, 0);
	let sprite_body_vertical     : (u8, u8) = (2, 1);
	let sprite_body_top_left     : (u8, u8) = (3, 0);
	let sprite_body_top_right    : (u8, u8) = (4, 0);
	let sprite_body_bottom_left  : (u8, u8) = (3, 1);
	let sprite_body_bottom_right : (u8, u8) = (4, 1);
	// let sprite_body_top_left     : (u8, u8) = (5, 0);
	// let sprite_body_top_right    : (u8, u8) = (6, 0);
	// let sprite_body_bottom_left  : (u8, u8) = (5, 1);
	// let sprite_body_bottom_right : (u8, u8) = (6, 1);
	let sprite_apple             : (u8, u8) = (7, 0);
	let sprite_rock              : (u8, u8) = (7, 1);
	let sprite = sprite_head_right;



	let size_screen = ( size_tile.0 * size_world.0 * scale, size_tile.1 * size_world.1 * scale );

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem.window("SDL2", size_screen.0 as u32, size_screen.1 as u32).position_centered().build().unwrap();
	let mut canvas = window.into_canvas().accelerated().build().unwrap();
	let texture_creator = canvas.texture_creator();

	canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));

	let mut event_pump = sdl_context.event_pump().unwrap();

	let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/spritesheet.bmp")).unwrap();
	let texture = texture_creator.create_texture_from_surface(&temp_surface).unwrap();

	let mut rect_source = Rect::new((sprite.0 * size_tile.0) as i32, (sprite.1 * size_tile.1) as i32, size_tile.0 as u32, size_tile.1 as u32);
	let mut rect_dest   = Rect::new(0, 0, (size_tile.0 * scale) as u32, (size_tile.0 * scale) as u32);

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

		position.0 = (position.0 + direction.0) % size_world.0 as i8;
		position.1 = (position.1 + direction.1) % size_world.1 as i8;
		rect_dest.set_x(position.0 as i32 * scale as i32 * size_tile.0 as i32);
		// rect_dest.set_y(position.1 as i32 * scale as i32 * size_tile.1 as i32);

		canvas.clear();
		canvas.copy_ex(&texture, Some(rect_source), Some(rect_dest), 0.0, None, false, false).unwrap();
		canvas.present();

		std::thread::sleep(Duration::from_millis(wait as u64));
	}
}