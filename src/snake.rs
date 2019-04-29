use types::*;
use SIZE_WORLD;

pub struct Snake {
	direction: Direction,
	head_position: Coordinate,
}

impl Snake {
	pub fn new( position: Coordinate, direction: Direction ) -> Snake {
		Snake {
			direction: direction,
			head_position: position,
		}
	}

	pub fn update( &mut self ){
		match self.direction {
			Direction::Up    => { self.head_position.y -= 1; },
			Direction::Down  => { self.head_position.y += 1; },
			Direction::Left  => { self.head_position.x -= 1; },
			Direction::Right => { self.head_position.x += 1; },
		}
		let pos = self.head_position;
		self.head_position = Coordinate {
			x: if pos.x < 0 { SIZE_WORLD.w as i8 - 1 } else { pos.x % SIZE_WORLD.w as i8 },
			y: if pos.y < 0 { SIZE_WORLD.h as i8 - 1 } else { pos.y % SIZE_WORLD.h as i8 },
		};
	}

	pub fn set_direction( &mut self, direction: Direction ){
		self.direction = direction;
	}

	pub fn get_position( &self ) -> Coordinate {
		self.head_position
	}
}