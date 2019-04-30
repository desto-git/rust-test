use types::Coordinate;

pub trait Drawable {
	fn get_position( &self ) -> Coordinate<u8>;
	fn get_sprite( &self ) -> Coordinate<u8>;
}