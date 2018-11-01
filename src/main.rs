extern crate sdl2;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

const SIZE_TILE  : (u8, u8) = ( 4, 4);
const SIZE_WORLD : (u8, u8) = (16, 8);

const DIR_UP    : (i8, i8) = ( 0, -1);
const DIR_DOWN  : (i8, i8) = ( 0,  1);
const DIR_LEFT  : (i8, i8) = (-1,  0);
const DIR_RIGHT : (i8, i8) = ( 1,  0);

const SPRITE_HEAD_UP           : (u8, u8) = (0, 0);
const SPRITE_HEAD_DOWN         : (u8, u8) = (0, 1);
const SPRITE_HEAD_LEFT         : (u8, u8) = (1, 0);
const SPRITE_HEAD_RIGHT        : (u8, u8) = (1, 1);
const SPRITE_BODY_HORIZONTAL   : (u8, u8) = (2, 0);
const SPRITE_BODY_VERTICAL     : (u8, u8) = (2, 1);
const SPRITE_BODY_TOP_LEFT     : (u8, u8) = (3, 0);
const SPRITE_BODY_TOP_RIGHT    : (u8, u8) = (4, 0);
const SPRITE_BODY_BOTTOM_LEFT  : (u8, u8) = (3, 1);
const SPRITE_BODY_BOTTOM_RIGHT : (u8, u8) = (4, 1);
// const SPRITE_BODY_TOP_LEFT     : (u8, u8) = (5, 0);
// const SPRITE_BODY_TOP_RIGHT    : (u8, u8) = (6, 0);
// const SPRITE_BODY_BOTTOM_LEFT  : (u8, u8) = (5, 1);
// const SPRITE_BODY_BOTTOM_RIGHT : (u8, u8) = (6, 1);
const SPRITE_APPLE             : (u8, u8) = (7, 0);
const SPRITE_ROCK              : (u8, u8) = (7, 1);

fn main() {
	let scale : u8 =   4;
	let wait  : u8 = 255;

	let mut direction = DIR_RIGHT;

	let mut position : (i8, i8) = (0, 0);
	let sprite = SPRITE_HEAD_RIGHT;

	let size_screen: (u16, u16) = (
		SIZE_TILE.0 as u16 * SIZE_WORLD.0 as u16 * scale as u16,
		SIZE_TILE.1 as u16 * SIZE_WORLD.1 as u16 * scale as u16,
	);

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem.window("Snake!?", size_screen.0 as u32, size_screen.1 as u32).position_centered().build().unwrap();
	let mut canvas = window.into_canvas().accelerated().build().unwrap();
	let texture_creator = canvas.texture_creator();

	canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));

	let mut event_pump = sdl_context.event_pump().unwrap();

	let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/spritesheet.bmp")).unwrap();
	let texture = texture_creator.create_texture_from_surface(&temp_surface).unwrap();

	let mut rect_source = Rect::new((sprite.0 * SIZE_TILE.0) as i32, (sprite.1 * SIZE_TILE.1) as i32, SIZE_TILE.0 as u32, SIZE_TILE.1 as u32);
	let mut rect_dest   = Rect::new(0, 0, (SIZE_TILE.0 * scale) as u32, (SIZE_TILE.0 * scale) as u32);

	let mut running = true;
	while running {
		for event in event_pump.poll_iter() {
			match event {
					Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
						running = false;
					},
					Event::KeyDown {keycode: Some( Keycode::Up    ), ..} => { direction = DIR_UP;    },
					Event::KeyDown {keycode: Some( Keycode::Down  ), ..} => { direction = DIR_DOWN;  },
					Event::KeyDown {keycode: Some( Keycode::Left  ), ..} => { direction = DIR_LEFT;  },
					Event::KeyDown {keycode: Some( Keycode::Right ), ..} => { direction = DIR_RIGHT; },
					_ => {}
			}
		}

		position.0 = (position.0 + direction.0) % SIZE_WORLD.0 as i8;
		position.1 = (position.1 + direction.1) % SIZE_WORLD.1 as i8;
		rect_dest.set_x(position.0 as i32 * scale as i32 * SIZE_TILE.0 as i32);
		rect_dest.set_y(position.1 as i32 * scale as i32 * SIZE_TILE.1 as i32);

		canvas.clear();
		canvas.copy_ex(&texture, Some(rect_source), Some(rect_dest), 0.0, None, false, false).unwrap();
		canvas.present();

		std::thread::sleep(Duration::from_millis(wait as u64));
	}
}