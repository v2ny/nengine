use nalgebra::{Matrix4, Perspective3};

#[derive(Debug)]
pub struct Distance {
	pub near: f32, pub far: f32
}

#[derive(Debug)]
pub struct ProjectionData {
	pub aspect_ratio: f32, 
	pub fov: f32,
	pub distance: Distance
}

#[derive(Debug)]
pub struct ProjectionMatrix {
    pub matrix: Matrix4<f32>,
}

impl ProjectionMatrix {
    pub fn new(data: ProjectionData) -> Self {
        let projection = Perspective3::new(
			data.aspect_ratio, data.fov, 
			data.distance.near, data.distance.far
		);

        ProjectionMatrix {
            matrix: projection.to_homogeneous(),
        }
    }
}
