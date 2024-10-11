use model::{ModelTransformData, Threed};

pub struct ThreedSize {
	pub width: i32, pub height: i32
}

pub struct UseThreed {
	pub size: ThreedSize,
	pub shader_type: Threed,
	pub model_transform: ModelTransformData,
}

pub mod projection;
pub mod view;
pub mod model;