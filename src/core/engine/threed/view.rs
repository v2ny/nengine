use nalgebra::{Matrix4, Point3, Vector3, Isometry3};

#[derive(Debug)]
pub struct ViewData {
	pub eye: Point3<f32>, 
	pub target: Point3<f32>, 
	pub up: Vector3<f32>
}

#[derive(Debug)]
pub struct ViewMatrix {
    pub matrix: Matrix4<f32>,
}

impl ViewMatrix {
    pub fn new(data: ViewData) -> Self {
        let view = Isometry3::look_at_rh(
			&data.eye, &data.target, &data.up
		);

        ViewMatrix {
            matrix: view.to_homogeneous(),
        }
    }
}