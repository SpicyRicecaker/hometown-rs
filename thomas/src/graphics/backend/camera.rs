/// [`Camera`]: Uses a perspective view matrix (from [`cgmath::perspective`]). 
/// If you're creating a pure 2d game *without zoom*, this class is not needed at all. 
/// Instead, just create a uniform view matrix with [`cgmath::ortho`].
/// TODO Update `tetris-wgpu-rs` to remove redundant model matrix
pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect_ratio: f32,
    pub fov: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    /// Takes in width and height for the aspect ratio, z for the view distance
    pub fn new(width: f32, height: f32, z_near: f32, z_far: f32) -> Self {
        Camera {
            // x is your horizontal location <- ->
            // y is your vertical location V ^
            // z is how far away or close you are to obj
            eye: cgmath::Point3::new(0.0, 0.0, 2.46),
            target: cgmath::Point3::new(0.0, 0.0, 0.0),
            up: cgmath::Vector3::unit_y(),
            aspect_ratio: (width / height) / 2.0,
            fov: 45.0,
            z_near,
            z_far,
        }
    }
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // Moves world to be at position & rotation of camera?
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);

        // Gives effect of depth
        let proj = cgmath::perspective(
            cgmath::Deg(self.fov),
            self.aspect_ratio,
            self.z_near,
            self.z_far,
        );

        // Order matters! Proj before view
        proj * view
    }
}
