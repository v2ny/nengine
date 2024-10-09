use super::{gltf, obj::MeshData};
use crate::core::engine::objects::graphics::GraphicsObjects;

pub struct ModelGobs {
	default: GraphicsObjects,
	texture: GraphicsObjects
}

pub struct ModelLoader {
    pub path: String,

	pub has_texture: bool,
    pub mesh_data: Option<MeshData>, // To store the loaded mesh data
	pub gobs: ModelGobs
}

impl ModelLoader {
    pub fn new(path: &str, has_texture: bool) -> Self {
        ModelLoader {
            path: String::from(path),
			
            mesh_data: None,
			gobs: ModelGobs {
				default: GraphicsObjects::default(),
				texture: GraphicsObjects::default(),
			},

			has_texture,
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

		// Generate the VAO, VBO, and EBO only once before drawing
		self.gobs.default.generate_vao();
		self.gobs.default.generate_vbo();
		self.gobs.default.generate_ebo();

		// Generate the texture's VAO, VBO, and EBO only once before drawing if the model has texture
		if self.has_texture {
			self.gobs.texture.generate_vao();
			self.gobs.texture.generate_vbo();
			self.gobs.texture.generate_ebo();
		}
    }

	// Function to draw the model using GraphicsObjects
	pub fn draw(&mut self) {
		if let Some(mesh_data) = &self.mesh_data {
			// Upload vertex data to VBO
			unsafe {
				self.gobs.default.bind_vao();
				self.gobs.default.bind_vbo();
				gl::BufferData(
					gl::ARRAY_BUFFER,
					(mesh_data.vertices.len() * std::mem::size_of::<f32>()) as isize,
					mesh_data.vertices.as_ptr() as *const std::ffi::c_void,
					gl::STATIC_DRAW,
				);
				// Define vertex attribute pointers (assuming positions only, 3 floats per vertex)
				gl::VertexAttribPointer(
					0, // attribute index
					3, // number of components (x, y, z)
					gl::FLOAT, // type of components
					gl::FALSE, // normalized
					0, // stride
					std::ptr::null(), // pointer to the start
				);
				gl::EnableVertexAttribArray(0);

				if self.has_texture {
					self.gobs.texture.bind_vbo();
					gl::BufferData(
						gl::ARRAY_BUFFER,
						(mesh_data.texcoords.len() * std::mem::size_of::<[f32; 2]>()) as isize,
						mesh_data.texcoords.as_ptr() as *const std::ffi::c_void,
						gl::STATIC_DRAW,
					);
					gl::VertexAttribPointer(
						1,
						2,
						gl::FLOAT,
						gl::FALSE,
						0,
						std::ptr::null(),
					);
					gl::EnableVertexAttribArray(1);
				}

				self.gobs.default.bind_ebo();
				gl::BufferData(
					gl::ELEMENT_ARRAY_BUFFER,
					(mesh_data.indices.len() * std::mem::size_of::<u32>()) as isize,
					mesh_data.indices.as_ptr() as *const std::ffi::c_void,
					gl::STATIC_DRAW,
				);

				// Bind the VAO and draw the object
				gl::DrawElements(
					gl::TRIANGLES,
					mesh_data.indices.len() as i32,
					gl::UNSIGNED_INT,
					std::ptr::null(),
				);
			}

			// Unbind the VAO after drawing
			self.gobs.default.unbind_vao();
		}
	}
}