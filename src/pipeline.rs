use std::io::stdout;

use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};

use crate::{
    Framebuffer, Pos2, Pos4, camera::Camera, framebuffer::Buffer, map_brightness_to_char,
    object::Object, raster::fill_triangle, shape::Shape,
};

pub(crate) struct Pipeline<T, O>
where
    O: Shape,
{
    projection_matrix: Mat4,
    objects: Box<[Object<O>]>,
    framebuffer: Framebuffer<T>,
    camera: Camera,
}

impl<T, O> Pipeline<T, O>
where
    T: Copy,
    O: Shape,
{
    pub fn new(
        fov: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
        objects: Box<[Object<O>]>,
        framebuffer: Framebuffer<T>,
        camera: Camera,
    ) -> Self {
        Pipeline::<T, O> {
            projection_matrix: Mat4::perspective_rh(fov, aspect_ratio, near, far),
            objects,
            framebuffer,
            camera,
        }
    }
}

fn project_to_screen(point: Pos4, framebuffer_width: usize, framebuffer_height: usize) -> Pos2 {
    let ndc = point.truncate() / point.w; // Vec3(x/w, y/w, z/w)

    Vec2::new(
        (ndc.x + 1.0) * 0.5 * framebuffer_width as f32,
        (1.0 - ndc.y) * 0.5 * framebuffer_height as f32,
    )
}

impl<O> Pipeline<char, O>
where
    O: Shape,
{
    #[inline(always)]
    pub fn rotate_cam_x(&mut self, pitch: f32) {
        self.camera.rotate_x(pitch);
    }

    #[inline(always)]
    pub fn rotate_cam_y(&mut self, yaw: f32) {
        self.camera.rotate_y(yaw);
    }

    #[inline(always)]
    pub fn update_radius(&mut self, radius: f32) {
        self.camera.update_radius(radius);
    }

    pub fn render(&mut self) -> std::io::Result<()> {
        self.framebuffer.clear();

        for object in self.objects.iter_mut() {
            let transformed = object.get_vertices().to_vec();
            let triangles = object.get_triangles();

            for &(i1, i2, i3) in triangles {
                let a = transformed[i1];
                let b = transformed[i2];
                let c = transformed[i3];

                let e1 = (b - a).xyz();
                let e2 = (c - a).xyz();
                let normal = e1.cross(e2).normalize();

                let camera_position = self.camera.get_position();

                let cam_dir = (camera_position - Vec3::ZERO).normalize();
                if normal.dot(cam_dir) > 0.0 {
                    continue;
                }

                let light_dir = (Vec3::ZERO - camera_position).normalize();
                let brightness = normal.dot(light_dir).clamp(0.0, 1.0);

                let shade = map_brightness_to_char(brightness);

                let va = self.projection_matrix * *self.camera.get_view_matrix() * a;
                let vb = self.projection_matrix * *self.camera.get_view_matrix() * b;
                let vc = self.projection_matrix * *self.camera.get_view_matrix() * c;

                let pa = project_to_screen(va, self.framebuffer.width(), self.framebuffer.height());
                let pb = project_to_screen(vb, self.framebuffer.width(), self.framebuffer.height());
                let pc = project_to_screen(vc, self.framebuffer.width(), self.framebuffer.height());

                fill_triangle(&mut self.framebuffer, pa, pb, pc, shade);
            }
        }

        self.framebuffer.write_io(&mut stdout())
    }
}
