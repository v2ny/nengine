use super::{gltf, obj::MeshData};
use crate::core::engine::objects::graphics::GraphicsObjects;

pub struct ModelLoader {
    pub path: String,
    pub mesh_data: Option<MeshData>, // To store the loaded mesh data
}

impl ModelLoader {
    pub fn new(path: &str) -> Self {
        ModelLoader {
            path: String::from(path),
            mesh_data: None,
        }
    }

    // Load the model depending on file extension
    pub fn load(&mut self) {
        let extension = self.path.as_str().rsplit_once('.').unwrap().1;
        match extension {
            "gltf" | "glb" => gltf::load(self.path.as_str()),
            "obj" => {
                if let Ok(mesh_data) = MeshData::load(self.path.as_str()) {
                    self.mesh_data = Some(mesh_data);
                }
            }
            _ => {
                println!("Unsupported model format");
            }
        }
    }

	// Function to draw the model using GraphicsObjects
	pub fn draw(&self, graphics_objects: &mut GraphicsObjects) {
		if let Some(mesh_data) = &self.mesh_data {
			// Generate the VAO, VBO, and EBO only once before drawing
			graphics_objects.generate_vao();
			graphics_objects.generate_vbo();
			graphics_objects.generate_ebo();

			// Bind the VAO, VBO, and EBO
			graphics_objects.bind_vao();
			graphics_objects.bind_vbo();
			graphics_objects.bind_ebo();

			// Upload vertex data to VBO
			unsafe {
				gl::BufferData(
					gl::ARRAY_BUFFER,
					(mesh_data.vertices.len() * std::mem::size_of::<f32>()) as isize,
					mesh_data.vertices.as_ptr() as *const std::ffi::c_void,
					gl::STATIC_DRAW,
				);

				// Upload index data to EBO
				gl::BufferData(
					gl::ELEMENT_ARRAY_BUFFER,
					(mesh_data.indices.len() * std::mem::size_of::<u32>()) as isize,
					mesh_data.indices.as_ptr() as *const std::ffi::c_void,
					gl::STATIC_DRAW,
				);

				// Define vertex attribute pointers (assuming positions only, 3 floats per vertex)
				gl::VertexAttribPointer(
					0, // attribute index
					3, // number of components (x, y, z)
					gl::FLOAT, // type of components
					gl::FALSE, // normalized
					3 * std::mem::size_of::<f32>() as i32, // stride
					std::ptr::null(), // pointer to the start
				);
				gl::EnableVertexAttribArray(0);

				// Bind the VAO and draw the object
				gl::DrawElements(
					gl::TRIANGLES,
					mesh_data.indices.len() as i32,
					gl::UNSIGNED_INT,
					std::ptr::null(),
				);
			}

			// Unbind the VAO after drawing
			graphics_objects.unbind_vao();
		}
	}
}