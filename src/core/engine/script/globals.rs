use rquickjs::prelude::Func;

use super::parser::{JSParser, LuaParser};

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

impl JSParser {
	fn export_functions(&mut self) {
        let context = self.context.borrow();
        context.with(|ctx| {
            // Example: Exporting a Rust function to JavaScript as "clear_window_color"
            ctx.globals().set(
                "clear_window_color",
                Func::new(|red: f32, green: f32, blue: f32, alpha: f32| {
                    clear_gl_window_color(red, green, blue, alpha);
                })
            ).unwrap();
        });
    }

	pub fn set_globals(&mut self) {
		self.context.borrow().with(|ctx| {
			ctx.globals().set("myGlobal", 42).unwrap();
		});
		self.export_functions();
		// todo
	}
}

// Define a Rust function that you want to expose to Lua
fn clear_gl_window_color(red: f32, green: f32, blue: f32, alpha: f32) {
	unsafe { gl::ClearColor(red, green, blue, alpha) };
}