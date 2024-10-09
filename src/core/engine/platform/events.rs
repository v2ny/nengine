use std::ffi::c_void;

use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use glfw::Context;
use nalgebra::{Point3, Vector3};

use crate::core::{engine::{self, objects::graphics::GraphicsObjects, threed::{model::{ModelMatrix, ModelTransformData}, projection::{self, ProjectionData, ProjectionMatrix}, view::{ViewData, ViewMatrix}}}, utils::model::manager::ModelLoader};
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

	/// # Safety
	///
	/// This function should not be called before calling the `initialize_opengl()` and shouldn't
	/// be called at any time, If you called `initialize_opengl()` function then you don't need
	/// to call this function as the initialize function calls it after initializing opengl.
	pub unsafe fn game_loop(&mut self) {
		let mut lua_parser = engine::script::parser::LuaParser::setup();
		let mut js_parser = engine::script::parser::JSParser::setup();

		self.initialize_app(&mut lua_parser, &mut js_parser);

		let mut cube = ModelLoader::new("examples/models/Carrots.obj");
		let mut gob = GraphicsObjects::default();
		cube.load();
		while !self.should_close() {
			let (width, height) = self.window.get_framebuffer_size();
			gl::Viewport(0, 0, width, height); // Update viewport
	
			// * Handle glfw events
			self.glfw.poll_events();
			self.handle_events();

			self.shaders.default.use_program();

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

			// * Clear window color
			lua_parser.load();
			js_parser.load();
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
			
			self.shaders.default.use_program();

			self.shaders.default.set_uniform_matrix4fv("projection", &projection.matrix);
			self.shaders.default.set_uniform_matrix4fv("view", &view.matrix);
			self.shaders.default.set_uniform_matrix4fv("model", &model.matrix);

			cube.draw(&mut gob);
			gl::DrawArrays(gl::TRIANGLES, 0, 3);

			// * Swap window's buffers :)
			self.window.swap_buffers();
		}
	}

	/// # Safety
	///
	/// This function is temporary, It is used to debug until loading the models.
	pub unsafe fn draw_triangle(&mut self) -> u32 {
		let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0, // left
             0.5, -0.5, 0.0, // right
             0.0,  0.5, 0.0  // top
        ];
        let (mut vbo, mut vao) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as GLsizei, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered vbo as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the vao afterwards so other vao calls won't accidentally modify this vao, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

		vao
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