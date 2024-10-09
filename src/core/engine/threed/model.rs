use nalgebra::{Matrix4, Vector3, Rotation3};

#[derive(Debug)]
pub struct ModelTransformData {
	pub translation: Vector3<f32>, 
	pub rotation: Vector3<f32>, 
	pub scale: Vector3<f32>
}

#[derive(Debug)]
pub struct ModelMatrix {
    pub matrix: Matrix4<f32>,
}

impl ModelMatrix {
    pub fn new(transform: ModelTransformData) -> Self {
        let translation_matrix = Matrix4::new_translation(&transform.translation);

        let rotation_matrix = Rotation3::from_euler_angles(
			transform.rotation.x, transform.rotation.y, transform.rotation.z
		).to_homogeneous();

        let scale_matrix = Matrix4::new_nonuniform_scaling(&transform.scale);

        let model = translation_matrix * rotation_matrix * scale_matrix;

        ModelMatrix {
            matrix: model,
        }
    }
}