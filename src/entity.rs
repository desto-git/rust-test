use types::Coordinate;
use traits::Drawable;

pub struct Entity {
	position: Coordinate<u8>,
	sprite: Coordinate<u8>,
}

impl Entity {
	pub fn new( position: Coordinate<u8>, sprite: Coordinate<u8> ) -> Entity {
		Entity {
			position: position,
			sprite: sprite,
		}
	}

	pub fn set_position( &mut self, position: Coordinate<u8> ){
		self.position = position;
	}
}

impl Drawable for Entity {
	fn get_position( &self ) -> Coordinate<u8> {
		self.position
	}

	fn get_sprite( &self ) -> Coordinate<u8> {
		self.sprite
	}
}