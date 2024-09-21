use std::fs;

pub fn load(path: &str) {
	let file_bytes = fs::read(path).expect("[GLTF:L] Failed to read the model's file. Possible a bad path.");
	let (gltf, _): (
		goth_gltf::Gltf<goth_gltf::default_extensions::Extensions>,
		_,
	) = goth_gltf::Gltf::from_bytes(&file_bytes).unwrap();
	println!("{:#?}", gltf);
}