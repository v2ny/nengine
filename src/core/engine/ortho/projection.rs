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
        let lr = 1.0 / (self.right - self.left);
        let bt = 1.0 / (self.top - self.bottom);
        let nf = 1.0 / (self.far - self.near);

        nalgebra::Matrix4::new(
            2.0 * lr, 0.0, 0.0, 0.0,
            0.0, 2.0 * bt, 0.0, 0.0,
            0.0, 0.0, -2.0 * nf, 0.0,
            -(self.right + self.left) * lr,
            -(self.top + self.bottom) * bt,
            -(self.far + self.near) * nf,
            1.0,
        )
    }
}
