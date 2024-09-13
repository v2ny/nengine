use super::parser::LuaParser;

impl LuaParser {
	fn export_functions(&mut self) {
		let lua = self.lua.get_mut();
		lua.globals().set("clear_window_color", lua.create_function(|_, (red, green, blue, alpha)| {
            clear_gl_window_color(red, green, blue, alpha);
            Ok(())
        }).unwrap()).unwrap();
	}

	pub fn set_globals(&mut self) {
		self.export_functions();
		// todo
	}
}

// Define a Rust function that you want to expose to Lua
fn clear_gl_window_color(red: f32, green: f32, blue: f32, alpha: f32) {
	unsafe { gl::ClearColor(red, green, blue, alpha) };
}