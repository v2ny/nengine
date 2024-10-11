use crate::core::engine::objects::graphics::GraphicsObjects;

#[derive(Debug, Default)]
pub struct TestComponent {
	gob: GraphicsObjects
}

const VERTICES: [f32; 12] = [
    -0.5, -0.5, 0.0, // Bottom-left
     0.5, -0.5, 0.0, // Bottom-right
    -0.5,  0.5, 0.0, // Top-left
     0.5,  0.5, 0.0, // Top-right
];

const INDICES: [u32; 6] = [
    0, 1, 2, // First triangle
    1, 3, 2, // Second triangle
];

impl TestComponent {
	pub fn init(&mut self) {
		self.gob.generate_vao();
		self.gob.generate_vbo();
		self.gob.generate_ebo();

		self.gob.bind_vao();
		self.gob.bind_vbo();

		unsafe {
			// Bind and fill VBO
			gl::BindBuffer(gl::ARRAY_BUFFER, self.gob.vbo);
			gl::BufferData(gl::ARRAY_BUFFER, (VERTICES.len() * std::mem::size_of::<f32>()) as isize, VERTICES.as_ptr() as *const _, gl::STATIC_DRAW);	

			// Bind and fill EBO
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.gob.ebo);
			gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (INDICES.len() * std::mem::size_of::<u32>()) as isize, INDICES.as_ptr() as *const _, gl::STATIC_DRAW);
	
			// Set vertex attribute pointers
			gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, std::ptr::null());
			gl::EnableVertexAttribArray(0);
		}

		self.gob.unbind_vbo();
		self.gob.unbind_vao();
	}

	pub fn draw(&mut self) {
		// Draw the square
		unsafe {
            gl::BindVertexArray(self.gob.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
	}
}