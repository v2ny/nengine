use egui_glfw::{egui::{self, vec2, Pos2, Rect}, EguiInputState};
use super::implementations::Window;

// TODO
// FIX THE RESIZING ERROR FOR THE UI
// FIX THE RESIZING ERROR FOR THE UI
// FIX THE RESIZING ERROR FOR THE UI
// FIX THE RESIZING ERROR FOR THE UI
// FIX THE RESIZING ERROR FOR THE UI
// FIX THE RESIZING ERROR FOR THE UI
// FIX THE RESIZING ERROR FOR THE UI
// FIX THE RESIZING ERROR FOR THE UI
// FIX THE RESIZING ERROR FOR THE UI
// FIX THE RESIZING ERROR FOR THE UI
// FIX THE RESIZING ERROR FOR THE UI
// FIX THE RESIZING ERROR FOR THE UI
// TODO

impl Window {
	pub fn egui_input_state(&mut self, width: f32, height: f32) -> EguiInputState {
		let native_pixels_per_point = self.window.get_content_scale().0;

		let mut eis = EguiInputState::new(egui::RawInput {
			screen_rect: Some(Rect::from_min_size(
				Pos2::new(0f32, 0f32),
				vec2(width, height) / native_pixels_per_point,
			)),
			..Default::default()
		});
		eis.input.time = Some(self.glfw.get_time());
	
		eis
	}

	pub fn handle_egui(&mut self) {
		let native_pixels_per_point = self.window.get_content_scale().0;

		let egui::FullOutput {
			platform_output,
			textures_delta,
			shapes, .. } = self.ui.context.clone().unwrap().end_frame();

		if !platform_output.copied_text.is_empty() {
			egui_glfw::copy_to_clipboard(self.ui.estate.as_mut().unwrap(), platform_output.copied_text);
		}

		let clipped_shapes = self.ui.context.clone().unwrap().tessellate(shapes, native_pixels_per_point);
		self.ui.painter.as_mut().expect("[S:UC] Painter isn't available.").paint_and_update_textures(native_pixels_per_point, &clipped_shapes, &textures_delta);

	}
}