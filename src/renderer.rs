use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;
use sdl2::render::Texture;
use sdl2::rect::Rect;

use sdl2::ttf::Font;
use sdl2::pixels::Color;

use SIZE_TILE;
use types::Coordinate;
use traits::Drawable;

pub struct Renderer<'a> {
	canvas: WindowCanvas,
	creator: sdl2::render::TextureCreator<WindowContext>,
	spritesheet: Texture<'a>,
	font: Font<'a, 'static>,
	rect_source: Rect,
	rect_target: Rect,
	rect_font: Rect,
}

impl<'a> Renderer<'a> {
	pub fn new( canvas: WindowCanvas, texture: Texture<'a>, font: Font<'a, 'static> ) -> Renderer<'a> {
		let texture_creator = canvas.texture_creator();
		Renderer {
			canvas: canvas,
			creator: texture_creator,
			spritesheet: texture,
			font: font,
			rect_source: Rect::new(0, 0, SIZE_TILE.w as u32, SIZE_TILE.h as u32),
			rect_target: Rect::new(0, 0, SIZE_TILE.w as u32, SIZE_TILE.h as u32),
			rect_font: Rect::new(0, 0, 0, 0),
		}
	}

	pub fn clear( &mut self ){
		self.canvas.clear();
	}

	pub fn present( &mut self ){
		self.canvas.present();
	}

	pub fn draw_object( &mut self, drawable: &impl Drawable ){
		let sprite = drawable.get_sprite();
		let at = drawable.get_position();

		self.rect_source.set_x( (sprite.x * SIZE_TILE.w) as i32 );
		self.rect_source.set_y( (sprite.y * SIZE_TILE.h) as i32 );
		self.rect_target.set_x( (at.x * SIZE_TILE.w) as i32 );
		self.rect_target.set_y( (at.y * SIZE_TILE.h) as i32 );
		self.canvas.copy(&self.spritesheet, Some(self.rect_source), Some(self.rect_target)).unwrap();
	}

	pub fn draw_text( &mut self, text: &str, at: Coordinate<u8>, color: Color ){
		let font_surface = self.font.render(text).solid(color).unwrap();
		let font_texture = self.creator.create_texture_from_surface(&font_surface).unwrap();
		let font_query = font_texture.query();

		self.rect_font.set_x( (at.x * SIZE_TILE.w) as i32 );
		self.rect_font.set_y( (at.y * SIZE_TILE.h) as i32 );
		self.rect_font.set_width ( font_query.width  );
		self.rect_font.set_height( font_query.height );

		self.canvas.copy(&font_texture, None, Some(self.rect_font)).unwrap();
	}
}