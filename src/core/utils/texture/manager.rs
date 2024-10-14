use gl::types::GLuint;
use image::{DynamicImage, GenericImageView};
use std::ffi::CString;

use crate::{log, utils::log::manager::{gl_error_to_message, LogLevel, Logger}};

#[derive(Debug, Clone)]
pub struct Texture {
    pub id: GLuint,
    pub src: String,
	pub flipv: bool,
    pub linear_sampler: GLuint,
	pub has_alpha: bool,
	logger: Logger
}

impl Texture {
    pub fn new(path: &str, flip_verticall: bool) -> Self {
        Texture {
            id: 0,
            src: String::from(path),
            linear_sampler: 0,
			has_alpha: false,
			flipv: flip_verticall,
			logger: Logger::new("debug/texture.log")
        }
    }

    pub fn init(&mut self) {
        unsafe {
            // Create texture and set up storage
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut self.id);
            if self.id == 0 {
				log!(self.logger, LogLevel::Error, "Failed to generate a texture id for \"{}\"", self.src);
            }

			// Create and configure sampler
			gl::CreateSamplers(1, &mut self.linear_sampler);
			if self.linear_sampler == 0 {
				log!(self.logger, LogLevel::Error, "Failed to generate a sampler id for \"{}\"", self.src);
			}

			gl::SamplerParameteri(self.linear_sampler, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
			gl::SamplerParameteri(self.linear_sampler, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
			gl::SamplerParameteri(self.linear_sampler, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
			gl::SamplerParameteri(self.linear_sampler, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
			
			// fn bad_image_log(src: &str) {
			// 	panic!("Failed to load an image");
			// }

			// fn get_debug_image(src: &str) -> DynamicImage {
			// 	let dbg_path = "assets/debug/texture.jpg";
			// 	bad_image_log(src);
			// 	if let Ok(dbg_img) = image::open(dbg_path) {
			// 		dbg_img
			// 	} else {
			// 		Log::send(
			// 			3850, "texture struct", "init", 
			// 			LogStatus::CRASH, 
			// 			"Could not find debug texture", dbg_path);
			// 		panic!("");
			// 	}
			// }

            // Load image data
            let mut img = image::open(self.src.clone()).unwrap();
			
			img = if self.flipv {
				img.flipv()
			} else {
				img
			};

            let (width, height) = img.dimensions();

			let (internal_format, format) = match img {
                DynamicImage::ImageRgba8(_) => (gl::RGBA8 as i32, gl::RGBA),
                DynamicImage::ImageRgb8(_) => (gl::RGB8 as i32, gl::RGB),
                _ => (gl::RGB8 as i32, gl::RGB),
			};

			if format == gl::RGBA {
				self.has_alpha = true;
			}

			let printable_format = if internal_format == gl::RGB8 as i32 {
				"gl::RGB8"
			} else { "gl::RGBA8" };

			log!(self.logger, LogLevel::Info, "Detected texture's image format is \"{}\"", printable_format);

            gl::TextureStorage2D(self.id, 1, internal_format as u32, width as i32, height as i32);

			match format {
				gl::RGBA => gl::TextureSubImage2D(
					self.id, 0, 
					0, 0, 
					width as i32, height as i32, 
					format, gl::UNSIGNED_BYTE,
					img.to_rgba8().as_ptr() as *const std::ffi::c_void
				),
				_ => gl::TextureSubImage2D(
					self.id, 0, 
					0, 0, 
					width as i32, height as i32, 
					format, gl::UNSIGNED_BYTE,
					img.to_rgb8().as_ptr() as *const std::ffi::c_void
				),
			};

            // Generate mipmaps
            gl::GenerateTextureMipmap(self.id);
			log!(self.logger, LogLevel::Info, "Generated texture mipmaps for texture's id number: \"{}\"", self.id);
        }
    }


    pub fn apply(&mut self, texture_unit: u32, sampler_name: &str, program_id: u32) {
        unsafe {
            if self.id == 0 {
				log!(self.logger, LogLevel::Error, "Couldn't apply texture \"{}\" because texture id was \"{}\"", self.src, self.id);
			}

            // Set the uniform to use the texture unit
            let texture_name = CString::new(sampler_name).unwrap();
            let texture_location = gl::GetUniformLocation(program_id, texture_name.as_ptr());
			if texture_location == -1 {
				log!(self.logger, LogLevel::Error, "Invalid sampler2d's \"{}\" uniform location for \"{}\" texture", sampler_name, self.src);
			}
            gl::ProgramUniform1i(program_id, texture_location, texture_unit as i32);

			let has_alpha_name = CString::new("has_alpha").unwrap();
			let has_alpha_location = gl::GetUniformLocation(program_id, has_alpha_name.as_ptr());
			if has_alpha_location == -1 {
				log!(self.logger, LogLevel::Error, "Invalid boolean's \"has_alpha\" uniform location for \"{}\" texture", self.src);
			}
			gl::ProgramUniform1i(program_id, has_alpha_location, self.has_alpha as i32);

            // Check for OpenGL errors
            let error = gl::GetError();
            if error != gl::NO_ERROR {
				log!(self.logger, LogLevel::Error, "Opengl error occured while applying texture \"{}\"\n- Opengl Code: {}\n- Opengl Formatted Reason: {}", self.src, error, gl_error_to_message(error));
            }

            // Activate texture unit
            gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::BindSampler(texture_unit, self.linear_sampler);

			// Check for OpenGL errors
            let error = gl::GetError();
            if error != gl::NO_ERROR {
				log!(self.logger, LogLevel::Error, "Opengl error occured after setting uniform location of the texture \"{}\"\n- Opengl Code: {}\n- Opengl Formatted Reason: {}", self.src, error, gl_error_to_message(error));
            }
        }
    }

	pub fn free(&self, texture_unit: u32) {
        unsafe {
			gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
        }
	}

    pub fn unapply(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0); // Unbind the texture
            gl::BindSampler(0, 0); // Unbind the sampler
        }
    }
}