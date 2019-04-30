use std::time::Duration;
use std::time::Instant;
use std::path::Path;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;

mod window;
mod types;
mod traits;
mod entity;
mod renderer;
mod snake;

use window::*;
use types::*;
use entity::*;
use renderer::*;
use snake::*;

const TITLE       : &str = "snake.rs";
const SPRITESHEET : &str = "assets/spritesheet.png";
const FONT        : &str = "assets/fnt/4Mono.ttf";

const SCALE : u8 =   4;
const WAIT  : u8 = 255;

const SIZE_TILE  : Dimension<u8> = Dimension { w:  4, h:  4 };
const SIZE_WORLD : Dimension<u8> = Dimension { w: 16, h: 12 };
const START_POSITION : Coordinate<u8> = Coordinate { x: 7, y: 3 };

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
	let size_screen: Dimension<u16> = Dimension{
		w: (SIZE_TILE.w * SIZE_WORLD.w) as u16,
		h: (SIZE_TILE.h * SIZE_WORLD.h) as u16,
	};

	let sdl_context = sdl2::init().unwrap();

	let ttf_context = sdl2::ttf::init().unwrap();
	let font = ttf_context.load_font(FONT, 4).unwrap();

	let window = Window::new(&sdl_context, TITLE, size_screen, SCALE);
	let canvas = window.get_canvas();
	let texture_creator = canvas.texture_creator();
	let texture = texture_creator.load_texture(Path::new(SPRITESHEET)).unwrap();

	let mut renderer = Renderer::new(canvas, texture, font);

	let mut event_pump = sdl_context.event_pump().unwrap();

	let mut snake = Snake::new(START_POSITION, Direction::Right);
	let apple = Entity::new(Coordinate { x: 0, y: 2 }, Coordinate { x:7, y:0 });

	let mut barrier: Vec<Entity> = Vec::new();
	for x in 0..SIZE_WORLD.w {
		barrier.push( Entity::new(Coordinate { x: x, y: 1 }, Coordinate { x:6, y:1 }) );
	}
	for x in 0..SIZE_WORLD.w {
		barrier.push( Entity::new(Coordinate { x: x, y: SIZE_WORLD.h-2 }, Coordinate { x:6, y:1 }) );
	}

	let wait_duration = Duration::from_millis(WAIT as u64);
	let mut timestamp = Instant::now() - wait_duration; // -wait so the first frame will be painted
	'gameloop: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
					break 'gameloop;
				},
				Event::KeyDown {keycode: Some( Keycode::Up    ), ..} => { snake.set_direction( Direction::Up    ); },
				Event::KeyDown {keycode: Some( Keycode::Down  ), ..} => { snake.set_direction( Direction::Down  ); },
				Event::KeyDown {keycode: Some( Keycode::Left  ), ..} => { snake.set_direction( Direction::Left  ); },
				Event::KeyDown {keycode: Some( Keycode::Right ), ..} => { snake.set_direction( Direction::Right ); },
				_ => {}
			}
		}

		if timestamp.elapsed() < wait_duration {
			std::thread::sleep( Duration::from_millis(10) ); // no need to update half a million times a second
			continue;
		}

		snake.update();

		renderer.clear();

		// game objects
		renderer.draw_object(&apple);
		renderer.draw_object(&snake);

		for x in barrier.iter(){
			renderer.draw_object(x);
		}

		// HUD
		renderer.draw_text("Score      Time ", Coordinate { x:0, y:0 }, Color::RGB(0x44, 0x44, 0x44));
		renderer.draw_text("     0         0", Coordinate { x:0, y:0 }, Color::RGB(0xCC, 0xCC, 0xCC));

		renderer.draw_text("Lives      FPS  ", Coordinate { x:0, y:SIZE_WORLD.h-1 }, Color::RGB(0x44, 0x44, 0x44));
		renderer.draw_text("     1        60", Coordinate { x:0, y:SIZE_WORLD.h-1 }, Color::RGB(0xCC, 0xCC, 0xCC));

		renderer.present();

		timestamp = Instant::now();
	}
}