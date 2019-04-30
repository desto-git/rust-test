use types::*;
use traits::*;
use SIZE_WORLD;

pub struct Snake {
	direction: Direction,
	head_position: Coordinate<u8>,
}

impl Snake {
	pub fn new( position: Coordinate<u8>, direction: Direction ) -> Snake {
		Snake {
			direction: direction,
			head_position: position,
		}
	}

	pub fn update( &mut self ){
		let mut pos = Coordinate {
			x: self.head_position.x as i8,
			y: self.head_position.y as i8,
		};

		match self.direction {
			Direction::Up    => { pos.y -= 1; },
			Direction::Down  => { pos.y += 1; },
			Direction::Left  => { pos.x -= 1; },
			Direction::Right => { pos.x += 1; },
		}

		self.head_position = Coordinate {
			x: if pos.x < 0 { SIZE_WORLD.w - 1 } else { pos.x as u8 % SIZE_WORLD.w },
			y: if pos.y < 0 { SIZE_WORLD.h - 1 } else { pos.y as u8 % SIZE_WORLD.h },
		};
	}

	pub fn set_direction( &mut self, direction: Direction ){
		self.direction = direction;
	}
}

impl Drawable for Snake {
	fn get_position( &self ) -> Coordinate<u8> {
		self.head_position
	}

	fn get_sprite( &self ) -> Coordinate<u8> {
		match self.direction {
			Direction::Up    => { Coordinate { x: 0, y: 0 } },
			Direction::Down  => { Coordinate { x: 0, y: 1 } },
			Direction::Left  => { Coordinate { x: 1, y: 0 } },
			Direction::Right => { Coordinate { x: 1, y: 1 } },
		}
	}
}