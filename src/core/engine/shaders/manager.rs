use core::str;
use std::{ffi::CString, fs};
use gl::types::{GLchar, GLint};

pub struct ShaderSources {
	pub vertex: String,
	pub fragment: String
}

pub struct Shader {
	pub program_id: u32,
	pub content: ShaderSources,
}

impl Shader {
	fn read_file_to_string(path: String) -> String {
		fs::read_to_string(path).expect("[S:RFTS] Could not read vertex and/or fragment shader's file. Possibly doesn't exist at all.")
	}

	pub fn new(sources: ShaderSources) -> Self {
		Shader {
			program_id: 0,
			content: ShaderSources {
				vertex: Shader::read_file_to_string(sources.vertex),
				fragment: Shader::read_file_to_string(sources.fragment),
			}
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
				println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
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
				println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
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

			// Attach link the shader program
			gl::LinkProgram(shader_program);

			let mut success = gl::FALSE as GLint;
			let mut info_log = vec![0; 512];
			info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
			 // check for linking errors
			gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
			if success != gl::TRUE as GLint {
				gl::GetProgramInfoLog(shader_program, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
				println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
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
}