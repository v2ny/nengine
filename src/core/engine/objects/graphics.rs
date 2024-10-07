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