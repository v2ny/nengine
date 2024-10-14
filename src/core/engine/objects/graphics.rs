use gl::types::{GLint, GLsizei, GLuint, GLvoid};
use image::{ImageBuffer, Rgba};

#[derive(Debug, Clone, Default)]
pub struct GraphicsObjects {
    pub vbo: u32,
    pub vao: u32,
    pub ebo: u32,
}

impl GraphicsObjects {
    pub fn generate_vbo(&mut self) {
        unsafe {
            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    pub fn generate_vao(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn generate_ebo(&mut self) {
        unsafe {
            gl::GenBuffers(1, &mut self.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        }
    }

    pub fn bind_vbo(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    pub fn unbind_vbo(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn bind_vao(&mut self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind_vao(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn bind_ebo(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        }
    }

    pub fn unbind_ebo(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

impl GraphicsObjects {
	pub fn create_texture(&mut self, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> GLuint {
        let (width, height) = img.dimensions();
        let raw_image = img.as_raw();

        // Generate a texture ID
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            // Upload texture data to GPU
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                raw_image.as_ptr() as *const GLvoid,
            );

            // Generate mipmaps
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        // Unbind the texture
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        texture_id
    }
}