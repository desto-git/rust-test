use std::time::Duration;
use std::time::Instant;
use std::path::Path;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;

mod window;
mod types;
mod snake;

use window::*;
use types::*;
use snake::*;

const TITLE       : &str = "snake.rs";
const SPRITESHEET : &str = "assets/spritesheet.png";
const FONT        : &str = "assets/fnt/4Mono.ttf";

const SCALE : u8 =   4;
const WAIT  : u8 = 255;

const SIZE_TILE  : Dimension<u8> = Dimension { w:  4, h: 4 };
const SIZE_WORLD : Dimension<u8> = Dimension { w: 16, h: 8 };
const START_POSITION : Coordinate = Coordinate { x: 7, y: 3 };

const SPRITE_HEAD_UP           : (u8, u8) = (0, 0);
const SPRITE_HEAD_DOWN         : (u8, u8) = (0, 1);
const SPRITE_HEAD_LEFT         : (u8, u8) = (1, 0);
const SPRITE_HEAD_RIGHT        : (u8, u8) = (1, 1);
// const SPRITE_BODY_HORIZONTAL   : (u8, u8) = (2, 0);
// const SPRITE_BODY_VERTICAL     : (u8, u8) = (2, 1);
// const SPRITE_BODY_TOP_LEFT     : (u8, u8) = (3, 0);
// const SPRITE_BODY_TOP_RIGHT    : (u8, u8) = (4, 0);
// const SPRITE_BODY_BOTTOM_LEFT  : (u8, u8) = (3, 1);
// const SPRITE_BODY_BOTTOM_RIGHT : (u8, u8) = (4, 1);
// const SPRITE_BODY_TOP_LEFT     : (u8, u8) = (5, 0);
// const SPRITE_BODY_TOP_RIGHT    : (u8, u8) = (6, 0);
// const SPRITE_BODY_BOTTOM_LEFT  : (u8, u8) = (5, 1);
// const SPRITE_BODY_BOTTOM_RIGHT : (u8, u8) = (6, 1);
// const SPRITE_APPLE             : (u8, u8) = (7, 0);
// const SPRITE_ROCK              : (u8, u8) = (7, 1);

fn main() {
	let mut sprite = SPRITE_HEAD_RIGHT;

	let size_screen: Dimension<u16> = Dimension{
		w: (SIZE_TILE.w * SIZE_WORLD.w) as u16,
		h: (SIZE_TILE.h * SIZE_WORLD.h) as u16,
	};

	let sdl_context = sdl2::init().unwrap();

	let ttf_context = sdl2::ttf::init().unwrap();
	let font = ttf_context.load_font(FONT, 14).unwrap();

	let window = Window::new(&sdl_context, TITLE, size_screen, SCALE);
	let mut canvas = window.get_canvas();
	let texture_creator = canvas.texture_creator();
	let texture = texture_creator.load_texture(Path::new(SPRITESHEET)).unwrap();

	let mut event_pump = sdl_context.event_pump().unwrap();

	let mut rect_source = Rect::new((sprite.0 * SIZE_TILE.w) as i32, (sprite.1 * SIZE_TILE.h) as i32, SIZE_TILE.w as u32, SIZE_TILE.w as u32);
	let mut rect_dest   = Rect::new(0, 0, SIZE_TILE.w as u32, SIZE_TILE.w as u32);
	let mut rect_font   = Rect::new(0, -6, 0, 0);

	let mut snake = Snake::new(START_POSITION, Direction::Right);

	let wait_duration = Duration::from_millis(WAIT as u64);
	let mut timestamp = Instant::now() - wait_duration; // -wait so the first frame will be painted
	'gameloop: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
					break 'gameloop;
				},
				Event::KeyDown {keycode: Some( Keycode::Up    ), ..} => { snake.set_direction( Direction::Up    ); sprite = SPRITE_HEAD_UP;    },
				Event::KeyDown {keycode: Some( Keycode::Down  ), ..} => { snake.set_direction( Direction::Down  ); sprite = SPRITE_HEAD_DOWN;  },
				Event::KeyDown {keycode: Some( Keycode::Left  ), ..} => { snake.set_direction( Direction::Left  ); sprite = SPRITE_HEAD_LEFT;  },
				Event::KeyDown {keycode: Some( Keycode::Right ), ..} => { snake.set_direction( Direction::Right ); sprite = SPRITE_HEAD_RIGHT; },
				_ => {}
			}
		}

		if timestamp.elapsed() < wait_duration {
			std::thread::sleep(Duration::from_millis(10)); // no need to update half a million times a second
			continue;
		}

		snake.update();
		rect_source.set_x( (sprite.0 * SIZE_TILE.w) as i32 );
		rect_source.set_y( (sprite.1 * SIZE_TILE.h) as i32 );
		rect_dest.set_x(snake.get_position().x as i32 * SIZE_TILE.w as i32);
		rect_dest.set_y(snake.get_position().y as i32 * SIZE_TILE.h as i32);

		canvas.clear();

		let font_surface = font.render("Score:0").solid(Color::RGB(0x44, 0x44, 0x44)).unwrap();
		let font_texture = texture_creator.create_texture_from_surface(&font_surface).unwrap();
		let font_query = font_texture.query();
		rect_font.set_width ( font_query.width  );
		rect_font.set_height( font_query.height );

		canvas.copy(&texture, Some(rect_source), Some(rect_dest)).unwrap();
		canvas.copy(&font_texture, None, Some(rect_font)).unwrap();

		canvas.present();

		timestamp = Instant::now();
	}
}