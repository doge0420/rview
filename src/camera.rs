use glam::{Mat4, Vec3, Vec3A};

const UP: Vec3 = Vec3::Y;
const TARGET: Vec3 = Vec3::ZERO;
const RADIUS: f32 = 10.0;

const DEFAULT_POS: Vec3 = Vec3::new(0.0, 0.0, -RADIUS);

pub(crate) struct Camera {
    yaw: f32,
    pitch: f32,
    radius: f32,
    view_matrix: Mat4,
}

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Camera {
            yaw: std::f32::consts::PI,
            pitch: 0.0,
            radius: RADIUS,
            view_matrix: Mat4::look_at_rh(DEFAULT_POS, TARGET, UP),
        };

        camera.update_view_matrix();
        camera
    }
}

impl Camera {
    pub fn new() -> Self {
        Camera::default()
    }

    #[inline(always)]
    pub fn get_view_matrix(&self) -> &Mat4 {
        &self.view_matrix
    }

    pub fn update_view_matrix(&mut self) {
        let x = self.radius * self.pitch.cos() * self.yaw.cos();
        let y = self.radius * self.pitch.sin();
        let z = self.radius * self.pitch.cos() * self.yaw.sin();

        let eye = Vec3::new(x, y, z);
        let target = Vec3::ZERO;

        self.view_matrix = Mat4::look_at_rh(eye, target, UP);
    }

    pub fn get_position(&self) -> Vec3A {
        let x = self.radius * self.pitch.cos() * self.yaw.cos();
        let y = self.radius * self.pitch.sin();
        let z = self.radius * self.pitch.cos() * self.yaw.sin();
        Vec3A::new(x, y, z)
    }

    pub fn update_radius(&mut self, radius: f32) {
        self.radius = radius;
        self.update_view_matrix();
    }

    pub fn rotate_x(&mut self, pitch: f32) {
        self.pitch = pitch;
        self.update_view_matrix();
    }

    pub fn rotate_y(&mut self, yaw: f32) {
        self.yaw = yaw;
        self.update_view_matrix();
    }
}
