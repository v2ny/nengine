use std::collections::HashMap;

use nalgebra::Vector3;
use rusttype::point;

use crate::core::{engine::objects::graphics::GraphicsObjects, utils::texture::bitmap::Texture};

use super::glyph::GlyphExtractor;

#[derive(Debug, Clone)]
pub struct TextComponent {
	pub text: String,

	scale: f32,
	color: Vector3<u8>,

	extractor: GlyphExtractor,

	pub glyphs_image: HashMap<char, Texture>,

	pub gob: GraphicsObjects,
	pub tgob: GraphicsObjects
}

impl TextComponent {
	pub fn new(font: &str, scale: f32) -> Self {
		let mut gob = GraphicsObjects::default();
		let mut tgob = GraphicsObjects::default();

		gob.generate_vbo();
		gob.generate_vao();
		gob.generate_ebo();
		tgob.generate_vbo();

		Self {
			color: Vector3::new(255, 255, 255),

			scale,
			text: String::new(),

			extractor: GlyphExtractor::new(GlyphExtractor {
				glyph_scale: scale,
				font_source: font.to_owned(),
				start_point: point(0.0, 0.0),
				..Default::default()
			}),

			glyphs_image: HashMap::new(),

			gob,
			tgob
		}
	}

	pub fn set_text(&mut self, text: &str) {
		self.text = text.to_owned();
	}

	pub fn set_scale(&mut self, scale: f32) {
		self.scale = scale;
	}

	pub fn set_color(&mut self, color: Vector3<u8>) {
		self.color = color;
	}
}

impl TextComponent {
	fn extract_text_as_glyphs(&mut self) {
		let characters = self.text.split("").filter(|x| !x.trim().is_empty());
		characters.for_each(|char| {
			let char = char.chars().nth(0).unwrap();
			if let std::collections::hash_map::Entry::Vacant(e) = self.glyphs_image.entry(char) {
				let image = self.extractor.extract_glyph(char, self.color);
				let mut texture = Texture::new(image, false);
				texture.init();
	
				e.insert(texture);
			}
		});
	}

	pub fn initialize(&mut self) {
		self.upload_vertices();
		self.upload_indices();
		self.bind_textures();

		self.extract_text_as_glyphs();
	}
}