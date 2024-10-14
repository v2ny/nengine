pub struct Orthographic {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
}

impl Orthographic {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self {
            left,
            right,
            bottom,
            top,
            near,
            far,
        }
    }

    pub fn matrix(&self) -> nalgebra::Matrix4<f32> {
        *nalgebra::Orthographic3::new(self.left, self.right, self.bottom, self.top, self.near, self.far).as_matrix()
    }
}
