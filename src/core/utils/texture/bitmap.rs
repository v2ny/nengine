use gl::types::GLuint;
use image::ImageBuffer;
use std::ffi::CString;

use crate::{log, utils::log::manager::{gl_error_to_message, LogLevel, Logger}};

#[derive(Debug, Clone)]
pub struct Texture {
    pub id: GLuint,
    pub flipv: bool,
    pub linear_sampler: GLuint,
    pub has_alpha: bool,
    logger: Logger,
    pub image: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>,
	pub dimensions: (u32, u32)
}

impl Texture {
    pub fn new(image: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>, flip_vertical: bool) -> Self {
        Texture {
            id: 0,
            image,
            linear_sampler: 0,
            has_alpha: true,
            flipv: flip_vertical,
            logger: Logger::new("debug/texture.log"),
			dimensions: (0, 0)
        }
    }

    pub fn init(&mut self) {
        unsafe {
            // Create texture and set up storage
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut self.id);
            if self.id == 0 {
                log!(self.logger, LogLevel::Error, "Failed to generate a texture id for \"{}\"", "bitmap");
            }

            // Create and configure sampler
            gl::CreateSamplers(1, &mut self.linear_sampler);
            if self.linear_sampler == 0 {
                log!(self.logger, LogLevel::Error, "Failed to generate a sampler id for \"{}\"", "bitmap");
            }

            gl::SamplerParameteri(self.linear_sampler, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
			gl::SamplerParameteri(self.linear_sampler, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
			gl::SamplerParameteri(self.linear_sampler, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
			gl::SamplerParameteri(self.linear_sampler, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // Load image data
            let mut img = self.image.clone();
            if self.flipv {
                img = image::imageops::flip_vertical(&img);
            }

            let (width, height) = img.dimensions();
			self.dimensions = img.dimensions();

            let (internal_format, format) = (gl::RGBA8 as i32, gl::RGBA); // Assuming RGBA by default

            self.has_alpha = true;

            gl::TextureStorage2D(self.id, 1, internal_format as u32, width as i32, height as i32);

            gl::TextureSubImage2D(
                self.id,
                0,
                0,
                0,
                width as i32,
                height as i32,
                format,
                gl::UNSIGNED_BYTE,
                img.as_raw().as_ptr() as *const std::ffi::c_void,
            );

            // Generate mipmaps
            gl::GenerateTextureMipmap(self.id);
            log!(self.logger, LogLevel::Info, "Generated texture mipmaps for texture's id number: \"{}\"", self.id);
        }
    }

    pub fn apply(&mut self, texture_unit: u32, sampler_name: &str, program_id: u32) {
        unsafe {
            if self.id == 0 {
                log!(self.logger, LogLevel::Error, "Couldn't apply texture \"{}\" because texture id was \"{}\"", "bitmap", self.id);
            }

            // Set the uniform to use the texture unit
            let texture_name = CString::new(sampler_name).unwrap();
            let texture_location = gl::GetUniformLocation(program_id, texture_name.as_ptr());
            if texture_location == -1 {
                log!(self.logger, LogLevel::Error, "Invalid sampler2d's \"{}\" uniform location for \"{}\" texture", sampler_name, "bitmap");
            }
            gl::ProgramUniform1i(program_id, texture_location, texture_unit as i32);

            // Activate texture unit
            gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::BindSampler(texture_unit, self.linear_sampler);

            // Check for OpenGL errors
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                log!(self.logger, LogLevel::Error, "Opengl error occured while applying texture \"{}\"\n- Opengl Code: {}\n- Opengl Formatted Reason: {}", "bitmap", error, gl_error_to_message(error));
            }
        }
    }

    pub fn unapply(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0); // Unbind the texture
            gl::BindSampler(0, 0); // Unbind the sampler
        }
    }

    pub fn free(&self, texture_unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
