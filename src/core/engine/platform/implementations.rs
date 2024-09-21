use std::collections;

use egui_glfw::{egui, glfw::{self, GlfwReceiver, PWindow, WindowEvent}, self as egui_backend};
use crate::core::engine::shaders::manager::Shader;

#[derive(Clone)]
pub struct Size {
	pub width: u32,
	pub height: u32,
}

impl Size {
	pub fn from(width: u32, height: u32) -> Self {
		Size { width, height }
	}
}

#[derive(Clone, Copy)]
pub struct DisplayOptions {
	pub fit_screen: bool,
	pub fullscreen: bool,
}

#[derive(Clone)]
pub struct WindowOptions {
	pub title: String,
	pub size: Size,
}

impl Default for WindowOptions {
	fn default() -> Self {
		WindowOptions {
			title: String::new(),
			size: Size::from(800, 400)
		}
	}
}

pub struct WindowProperties {
	pub window_options: WindowOptions,
	pub display_options: Option<DisplayOptions>
}

pub struct WindowShaders {
	pub default: Shader
}

pub struct UIStates {
	pub float: collections::HashMap<String, f32>,
	pub string: collections::HashMap<String, String>,
}

pub struct UIHolder {
	pub painter: Option<egui_backend::Painter>,
	pub context: Option<egui::Context>,
	pub estate: Option<egui_backend::EguiInputState>,
	pub states: UIStates,
	pub native_pixel_point: f32,
}

impl Default for UIHolder {
	fn default() -> Self {
		UIHolder {
			painter: None,
			context: None,
			estate: None,
			states: UIStates { 
				float: collections::HashMap::new(), 
				string: collections::HashMap::new() 
			},
			native_pixel_point: 0.1,
		}	
	}
}

impl UIHolder {
	pub fn set_np(&mut self, window: &mut PWindow) {
		let native_pixels_per_point = window.get_content_scale().0;
		self.native_pixel_point = native_pixels_per_point;
	}

	pub fn setup(&mut self, window: &mut PWindow) {
		self.painter = Some(egui_backend::Painter::new(window));
		self.context = Some(egui::Context::default());
		self.set_np(window);
		self.states.float.insert(String::from("slider"), 0.0);
	}

	pub fn draw(&mut self) {
		let slider = self.states.float.get_mut(&String::from("slider")).unwrap();
		egui::Window::new("Egui with GLFW").show(&self.context.clone().unwrap(), |ui| {
			ui.label("A simple sine wave plotted onto a GL texture then blitted to an egui managed Image.");
			let btn_m = &mut ui.button("-");
			let btn_p = &mut ui.button("+");

			ui.add(egui::Slider::new(slider, 0.0..=100.0).text("My value"));

			if btn_m.clicked() && *slider > 0.0 {
				*slider -= 1.0;
			}

			if btn_p.clicked() && *slider < 100.0 {
				*slider += 1.0;
			}
		});
	}
}

pub struct Window {
	pub glfw: glfw::Glfw,
	pub window: glfw::PWindow,
	pub events: GlfwReceiver<(f64, WindowEvent)>,

	pub ui: UIHolder,
	pub scripts: Vec<String>,
	pub shaders: WindowShaders
}