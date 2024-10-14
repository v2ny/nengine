use image::{ImageBuffer, Rgba, RgbaImage};
use nalgebra::Vector3;
use rusttype::{point, Font, Point, Scale};

#[derive(Debug, Clone)]
pub struct GlyphExtractor {
	pub font_source: String,
	pub font_rt: Font<'static>,

	pub start_point: Point<f32>,
	pub glyph_scale: f32,
}

impl Default for GlyphExtractor {
	fn default() -> Self {
		let path = String::from("resources/fonts/default.ttf");
		let bytes = Box::leak(std::fs::read(path.clone())
			.expect("Failed to get default.ttf from resources directory.")
			.into_boxed_slice());
	
		let font = Font::try_from_bytes(bytes).expect("Failed to try from default.ttf bytes.");

		Self {
			font_source: path,

			start_point: point(0.0, 0.0),
			glyph_scale: 16.0,

			font_rt: font,
		}
	}
}

impl GlyphExtractor {
	pub fn new(mut ge: GlyphExtractor) -> Self {
		let bytes = Box::leak(std::fs::read(ge.font_source.clone())
			.expect("Failed to get default.ttf from resources directory.")
			.into_boxed_slice());
	
		let font = Font::try_from_bytes(bytes).expect("Failed to try from default.ttf bytes.");

		ge.font_rt = font;
		ge
	}
}

impl GlyphExtractor {
	pub fn extract_glyph(&mut self, character: char, color: Vector3<u8>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
		let glyph = self.font_rt.glyph(character).scaled(Scale::uniform(self.glyph_scale)).positioned(self.start_point);

		// Get the bounding box of the glyph
		let bb = glyph.pixel_bounding_box().unwrap();
	
		// Create an empty image buffer to hold the glyph (RGBA format)
		let width = bb.width() as u32;
		let height = bb.height() as u32;
		let mut image = RgbaImage::new(width, height);
	
		// Rasterize the glyph into the image buffer
		glyph.draw(|x, y, v| {
			// x and y are offsets into the image
			let px = x;
			let py = y;
	
			// The 'v' is the coverage value (0.0 to 1.0), we map it to 255 for alpha
			let alpha = (v * 255.0) as u8;
	
			// Write the pixel into the image with white color and 'alpha' transparency
			image.put_pixel(px, py, Rgba([color.x, color.y, color.z, alpha]));
		});

		image
	}
}