use egui_glfw::glfw::{self, Context};
use crate::core::engine;

use super::implementations::Window;

impl Window {
	/// # Safety
	///
	/// This function should not be called before calling the `initialize_opengl()` and shouldn't
	/// be called at any time, If you called `initialize_opengl()` function then you don't need
	/// to call this function as the initialize function calls it after initializing opengl.
	pub unsafe fn game_loop(&mut self) {
		let mut parser = engine::script::parser::LuaParser::setup();
		parser.init_globals();
		for script in self.scripts.iter_mut() {
			parser.add(script.to_string());
		}

		while !self.should_close() {
			// * Handle glfw events
			self.glfw.poll_events();
			self.handle_events();

			// * Clear window color
			// gl::ClearColor(0.0, 1.0, 0.3, 1.0);
			parser.load();
			gl::Clear(gl::COLOR_BUFFER_BIT);

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
				// glfw::WindowEvent::CursorPos(x, y) => {
				// 	println!("X: {:.2}, Y: {:.2}", x, y);
				// }
				_ => {}
			}
		}
	}
}