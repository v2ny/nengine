use core::str;
use std::{ffi::CString, fs, path::Path};
use gl::types::{GLchar, GLint};
use nalgebra::Matrix4;

use crate::{log, utils::log::manager::{LogLevel, Logger}};

pub struct ShaderSources {
    pub vertex: String,
    pub fragment: String,
}

pub struct Shader {
    pub program_id: u32,
    pub content: ShaderSources,
    logger: Logger
}

impl Shader {
    fn read_file_to_string(path: String) -> String {
        fs::read_to_string(&path)
            .unwrap_or_else(|_| {
                log!(Logger::new("debug/shader.log"), LogLevel::Error, "Could not read shader file at path: {}", &path);
                panic!("[S:RFTS] Could not read vertex and/or fragment shader's file. Possibly doesn't exist at all.")
            })
    }

    pub fn new(sources: ShaderSources) -> Self {
        Shader {
            program_id: 0,
            content: ShaderSources {
                vertex: Shader::read_file_to_string(sources.vertex),
                fragment: Shader::read_file_to_string(sources.fragment),
            },
            logger: Logger::new("debug/shader.log")
        }
    }

    pub fn setup(&mut self) {
        let vs = self.generate_and_link_vertex_program();
        let fs = self.generate_and_link_fragment_program();
        self.link_vs_and_fs_to_shader_program(vs, fs);
    }

    fn generate_and_link_vertex_program(&mut self) -> u32 {
        unsafe {
            let vertex_shader: u32 = gl::CreateShader(gl::VERTEX_SHADER);
            let vertex_src = CString::new(self.content.vertex.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &vertex_src.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex_shader);

            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vertex_shader, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                info_log.set_len(512);

                let parent_dir = Path::new(&self.content.vertex).parent().map_or_else(
                    || "Unknown directory".to_string(),
                    |p| p.display().to_string()
                );

                log!(self.logger, LogLevel::Error, "ERROR::SHADER::VERTEX::COMPILATION_FAILED");
                log!(self.logger, LogLevel::Error, "Shader Directory: {}", parent_dir);
                log!(self.logger, LogLevel::Error, "Error: {}", String::from_utf8_lossy(&info_log));
            } else {
                log!(self.logger, LogLevel::Info, "Vertex shader compiled successfully.");
            }

            vertex_shader
        }
    }

    fn generate_and_link_fragment_program(&mut self) -> u32 {
        unsafe {
            let fragment_shader: u32 = gl::CreateShader(gl::FRAGMENT_SHADER);
            let fragment_src = CString::new(self.content.fragment.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &fragment_src.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment_shader);

            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(fragment_shader, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                info_log.set_len(512);

                let parent_dir = Path::new(&self.content.fragment).parent().map_or_else(
                    || "Unknown directory".to_string(),
                    |p| p.display().to_string()
                );

                log!(self.logger, LogLevel::Error, "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED");
                log!(self.logger, LogLevel::Error, "Shader Directory: {}", parent_dir);
                log!(self.logger, LogLevel::Error, "Error: {}", String::from_utf8_lossy(&info_log));
            } else {
                log!(self.logger, LogLevel::Info, "Fragment shader compiled successfully.");
            }

            fragment_shader
        }
    }

    fn link_vs_and_fs_to_shader_program(&mut self, vertex_shader: u32, fragment_shader: u32) {
        unsafe {
            let shader_program = gl::CreateProgram();

            // Attach vertex shader to the shader program
            gl::AttachShader(shader_program, vertex_shader);

            // Attach fragment shader to the shader program
            gl::AttachShader(shader_program, fragment_shader);

            // Link the shader program
            gl::LinkProgram(shader_program);

            let mut success = gl::FALSE as GLint;
            let mut info_log = vec![0; 512];
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character

            // check for linking errors
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader_program, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);

                log!(self.logger, LogLevel::Error, "ERROR::SHADER::PROGRAM::LINKING_FAILED");
                log!(self.logger, LogLevel::Error, "Error: {}", str::from_utf8(&info_log).unwrap());
            } else {
                log!(self.logger, LogLevel::Info, "Shader program compiled and linked successfully.");
            }

            // Set the shader implementation's struct program_id to the shader_program (The program id)
            self.program_id = shader_program;

            // Once we are done and we got the program id, We can now delete both of the vertex and fragment shader.
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }
    }

    pub fn use_program(&mut self) {
        // Use the shader program.
        unsafe { gl::UseProgram(self.program_id) }
    }

    pub fn set_uniform_matrix4fv(&mut self, name: &str, matrix: &Matrix4<f32>) {
        // Get the location of the uniform in the shader program
        let cstr = std::ffi::CString::new(name).unwrap();
        let location = unsafe { gl::GetUniformLocation(self.program_id, cstr.as_ptr()) };

        if location != -1 {
            // Send the matrix data to the uniform
            unsafe {
                gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
            }
        }
    }

    pub fn set_uniform3f(&mut self, name: &str, x: f32, y: f32, z: f32) {
        let cstr = CString::new(name).unwrap();
        let location = unsafe { gl::GetUniformLocation(self.program_id, cstr.as_ptr()) };
        unsafe {
            gl::Uniform3f(location, x, y, z);
        }
    }

	pub fn set_uniform4f(&mut self, name: &str, x: f32, y: f32, z: f32, a: f32) {
        let cstr = CString::new(name).unwrap();
        let location = unsafe { gl::GetUniformLocation(self.program_id, cstr.as_ptr()) };
        unsafe {
            gl::Uniform4f(location, x, y, z, a);
        }
    }
}