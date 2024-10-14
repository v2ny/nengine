use std::time::Instant;

use glfw::Context;
use nalgebra::{Point3, Vector3};

use crate::core::{engine::{self, threed::{model::{ModelMatrix, ModelTransformData, Threed}, projection::{self, ProjectionData, ProjectionMatrix}, view::{ViewData, ViewMatrix}, ThreedSize, UseThreed}, ui::text::ext::TextComponent}, utils::{model::manager::ModelLoader, texture::manager::Texture}};
use super::implementations::Window;

impl Window {
	pub fn initialize_app(
		&mut self, 
		lua_parser: &mut engine::script::parser::LuaParser,
		js_parser: &mut engine::script::parser::JSParser
	) {
		lua_parser.init_globals();
		js_parser.init_globals();

		for script in self.scripts.iter_mut() {
			if script.ends_with(".lua") {
				lua_parser.add(script.to_string());
			} else {
				js_parser.add(script.to_string());
			}
		}
	}

	pub fn use_threed_world(&mut self, data: UseThreed) {
		let projection = ProjectionMatrix::new(ProjectionData {
			fov: 80.0_f32.to_radians(),
			aspect_ratio: (data.size.width as f32) / (data.size.height as f32),
			distance: projection::Distance {
				far: 1000.0,
				near: 0.1,
			}
		});

		let view = ViewMatrix::new(ViewData {
			eye: Point3::new(0.0, 0.0, 5.0),
			target: Point3::default(),
			up: Vector3::y(),
		});

		let model = ModelMatrix::new(data.model_transform);

		if data.shader_type == Threed::DEFAULT {
			self.shaders.default.set_uniform_matrix4fv("projection", &projection.matrix);
			self.shaders.default.set_uniform_matrix4fv("view", &view.matrix);
			self.shaders.default.set_uniform_matrix4fv("model", &model.matrix);
		}
	}

	/// # Safety
	///
	/// This function should not be called before calling the `initialize_opengl()` and shouldn't
	/// be called at any time, If you called `initialize_opengl()` function then you don't need
	/// to call this function as the initialize function calls it after initializing opengl.
	pub unsafe fn game_loop(&mut self) {
		let mut lua_parser = engine::script::parser::LuaParser::setup();
		let mut js_parser = engine::script::parser::JSParser::setup();

		self.initialize_app(&mut lua_parser, &mut js_parser);

		let mut texture = Texture::new("examples/models/textures/CarrotTexture.png", true);

		let mut cube = ModelLoader::new("examples/models/cube.obj", true);
		texture.init();
		cube.load();

		let font_scale = 16.0;
		let mut text = TextComponent::new("resources/fonts/default.ttf", font_scale);
		text.set_text("excuse me what is a kilogram :eagle: 1234");
		text.initialize();

		let mut last_frame_time = Instant::now();
		let mut fps: f32 = 0.0;
		
		while !self.should_close() {
			let (width, height) = self.window.get_framebuffer_size();
			gl::Viewport(0, 0, width, height);
	
			// * Handle glfw events
			self.glfw.poll_events();
			self.handle_events();

			self.shaders.default.use_program();

			let time = self.glfw.get_time() as f32;
			
			// * Setup & use projection, model and view matrix
			self.use_threed_world(UseThreed { 
				size: ThreedSize { width, height }, 
				shader_type: Threed::DEFAULT, 
				model_transform: ModelTransformData { 
					translation: Vector3::default(), 
					rotation: Vector3::new(time, time, 0.0), 
					scale: Vector3::new(1.0, 1.0, 1.0)
				}
			});

			// * Clear window color
			lua_parser.load();
			js_parser.load();
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

			texture.apply(3, "texture1", self.shaders.default.program_id);
			cube.draw();

			self.shaders.ui.use_program();

			// Set up the projection matrix where (0, 0) is top-left and (width, height) is bottom-right
			let projection_matrix = nalgebra::Matrix4::new_orthographic(
				0.0, width as f32,    // Left, right
				height as f32, 0.0,   // Bottom, top (flipped to place (0, 0) at top-left)
				-1.0, 1.0             // Near, far
			);
			
			
			self.shaders.ui.set_uniform_matrix4fv("projection", &projection_matrix);

			let binding = text.text.clone();
			let chars = binding.chars();

			// 1. Calculate time since the last frame
			let now = Instant::now();
			let frame_duration = now.duration_since(last_frame_time);
			last_frame_time = now;
			
			// 2. Calculate FPS (frames per second)
			let frame_time_seconds = frame_duration.as_secs_f32();  // Convert to seconds
			if frame_time_seconds > 0.0 {
				fps = 1.0 / frame_time_seconds;
			}


			let mut last_img_width = font_scale / 2.0;
			let mut last_img_height = 0.0;
			let space_width = font_scale / 1.5;  // Adjust this value for the space character

			// Find the maximum height of all glyphs (to set a common baseline)
			let max_height = text.glyphs_image.values()
				.map(|img| img.dimensions.1 as f32)
				.max_by(|a, b| a.partial_cmp(b).unwrap())
				.unwrap_or(0.0);  // Default to 0.0 if no glyphs are available

			// Loop through each character in the string
			chars.for_each(|char| {
				if let Some(img) = text.glyphs_image.clone().get_mut(&char) {
					// Character found in glyphs_image
					last_img_height = img.dimensions.1 as f32;
					// Calculate the y offset to align all glyphs on the same baseline
					let vertical_offset = max_height - img.dimensions.1 as f32;

					// Adjust translation: apply only the vertical_offset without adding it to glyph height
					let model = ModelMatrix::new(ModelTransformData {
						translation: Vector3::new(last_img_width, last_img_height + vertical_offset + (font_scale / 2.0), 0.0),
						rotation: Vector3::default(),
						scale: Vector3::new(img.dimensions.0 as f32, img.dimensions.1 as f32, 1.0),
					});

					// Set the uniform matrix for the shader
					self.shaders.ui.set_uniform_matrix4fv("model", &model.matrix);

					// Apply the texture and draw the character
					img.apply(0, "texture1", self.shaders.ui.program_id);
					text.set_vertex();

					// Increment the width based on the character's width and a small adjustment for spacing
					last_img_width += img.dimensions.0 as f32 + font_scale * 0.1;  // Adjust horizontal spacing
				} else {
					// If character is not found (or is a space), increment by the space_width
					last_img_width += space_width;  // Use appropriate space width
				}
			});

			// * Swap window's buffers :)
			self.window.swap_buffers();
		}
	}

	/// # Safety
	///
	/// This function should not be called by you, the programmer/coder/user. This is automatically called!
	pub unsafe fn handle_events(&mut self) {
		for (_, event) in glfw::flush_messages(&self.events) {
			#[allow(clippy::single_match)]
			match event {
				glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Release, _) => {
					self.window.set_should_close(true);
				},
				_ => {}
				// glfw::WindowEvent::CursorPos(x, y) => {
				// 	println!("X: {:.2}, Y: {:.2}", x, y);
				// }
			}
		}
	}
}