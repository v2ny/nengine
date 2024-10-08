use glfw::Context;
use nalgebra::{Point3, Vector3};

use crate::core::{engine::{self, threed::{model::{ModelMatrix, ModelTransformData}, projection::{self, ProjectionData, ProjectionMatrix}, view::{ViewData, ViewMatrix}}}, utils::{model::manager::ModelLoader, texture::manager::Texture}};
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

	pub fn use_threed_world(&mut self, width: i32, height: i32) {
		let projection = ProjectionMatrix::new(ProjectionData {
			fov: 80.0_f32.to_radians(),
			aspect_ratio: (width as f32) / (height as f32),
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

		let time = self.glfw.get_time() as f32;
		let rotation = Vector3::new(time, time, 0.0); // Rotating in X and Y axes over time

		let model = ModelMatrix::new(ModelTransformData {
			translation: Vector3::default(),
			scale: Vector3::new(1.0, 1.0, 1.0),
			rotation,
		});

		self.shaders.default.set_uniform_matrix4fv("projection", &projection.matrix);
		self.shaders.default.set_uniform_matrix4fv("view", &view.matrix);
		self.shaders.default.set_uniform_matrix4fv("model", &model.matrix);
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

		while !self.should_close() {
			let (width, height) = self.window.get_framebuffer_size();
			gl::Viewport(0, 0, width, height); // Update viewport
	
			// * Handle glfw events
			self.glfw.poll_events();
			self.handle_events();

			self.shaders.default.use_program();
			
			// * Setup & use projection, model and view matrix
			self.use_threed_world(width, height);

			// * Clear window color
			lua_parser.load();
			js_parser.load();
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

			// * Render a model
			texture.apply(0, self.shaders.default.program_id);
			cube.draw();

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