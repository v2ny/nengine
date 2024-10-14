use core::engine::platform::implementations::{self, Size, WindowOptions, WindowProperties};

pub mod core;
pub mod utils;

fn main() {
    let mut platform = implementations::Window::new(WindowProperties {
		window_options: WindowOptions {
			title: String::from("nengine"),
			size: Size::from(1024, 600),
			// ..Default::default()
		},
		display_options: None
	}, vec!["examples/script/test.lua".to_string(), "examples/script/test.js".to_string()]);
	
	platform.initialize_opengl();
	platform.run();
}