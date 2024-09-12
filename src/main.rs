use core::engine::platform::implementations::{self, WindowProperties, WindowOptions, Size};

pub mod core;

fn main() {
    let mut platform = implementations::Window::new(WindowProperties {
		window_options: WindowOptions {
			title: String::from("nengine"),
			size: Size::from(1024, 600),
			// ..Default::default()
		},
		display_options: None
	});

	platform.initialize_opengl();
}