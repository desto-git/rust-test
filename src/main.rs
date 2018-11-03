use std::time::Duration;
use std::path::Path;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;

const TITLE: &str = "snake.rs";
const SPRITESHEET: &str = "assets/spritesheet.png";

const SCALE : u8 =   4;
const WAIT  : u8 = 255;

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
	let mut length : u8 = 3;
	let mut position : (i8, i8) = (7, 3);
	let mut direction = DIR_RIGHT;
	let mut sprite = SPRITE_HEAD_RIGHT;

	let size_screen: (u16, u16) = (
		SIZE_TILE.0 as u16 * SIZE_WORLD.0 as u16 * SCALE as u16,
		SIZE_TILE.1 as u16 * SIZE_WORLD.1 as u16 * SCALE as u16,
	);

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window(TITLE, size_screen.0 as u32, size_screen.1 as u32).position_centered().build().unwrap();
	let mut canvas = window.into_canvas().accelerated().build().unwrap();
	let texture_creator = canvas.texture_creator();
	let texture = texture_creator.load_texture(Path::new(SPRITESHEET)).unwrap();
	canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));

	let mut event_pump = sdl_context.event_pump().unwrap();

	let mut rect_source = Rect::new((sprite.0 * SIZE_TILE.0) as i32, (sprite.1 * SIZE_TILE.1) as i32, SIZE_TILE.0 as u32, SIZE_TILE.1 as u32);
	let mut rect_dest   = Rect::new(0, 0, (SIZE_TILE.0 * SCALE) as u32, (SIZE_TILE.0 * SCALE) as u32);

	let mut running = true;
	while running {
		for event in event_pump.poll_iter() {
			match event {
					Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
						running = false;
					},
					Event::KeyDown {keycode: Some( Keycode::Up    ), ..} => { direction = DIR_UP;    sprite = SPRITE_HEAD_UP;    },
					Event::KeyDown {keycode: Some( Keycode::Down  ), ..} => { direction = DIR_DOWN;  sprite = SPRITE_HEAD_DOWN;  },
					Event::KeyDown {keycode: Some( Keycode::Left  ), ..} => { direction = DIR_LEFT;  sprite = SPRITE_HEAD_LEFT;  },
					Event::KeyDown {keycode: Some( Keycode::Right ), ..} => { direction = DIR_RIGHT; sprite = SPRITE_HEAD_RIGHT; },
					_ => {}
			}
		}

		position.0 = if (position.0 + direction.0) < 0 { SIZE_WORLD.0 as i8 - 1 } else { (position.0 + direction.0) % SIZE_WORLD.0 as i8 };
		position.1 = if (position.1 + direction.1) < 0 { SIZE_WORLD.1 as i8 - 1 } else { (position.1 + direction.1) % SIZE_WORLD.1 as i8 };
		rect_source.set_x( (sprite.0 * SIZE_TILE.0) as i32 );
		rect_source.set_y( (sprite.1 * SIZE_TILE.1) as i32 );
		rect_dest.set_x(position.0 as i32 * SCALE as i32 * SIZE_TILE.0 as i32);
		rect_dest.set_y(position.1 as i32 * SCALE as i32 * SIZE_TILE.1 as i32);

		canvas.clear();
		canvas.copy_ex(&texture, Some(rect_source), Some(rect_dest), 0.0, None, false, false).unwrap();
		canvas.present();

		std::thread::sleep(Duration::from_millis(WAIT as u64));
	}
}