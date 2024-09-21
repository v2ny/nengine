use super::gltf;

pub struct ModelLoader {
	pub path: String,
}

impl ModelLoader {
	pub fn new(path: &str) -> Self {
		ModelLoader {
			path: String::from(path)
		}
	}

	pub fn load(&mut self) {
		let extension = self.path.as_str().rsplit_once('.').unwrap().1;
		match extension {
			"gltf" | "glb" => gltf::load(self.path.as_str()),
			_ => {}
		}
	}
}