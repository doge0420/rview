use std::{f32, io::stdout};

use glam::{Mat4, Vec3A, Vec4Swizzles};

use crate::{
    Framebuffer, Pos3, Pos4, camera::Camera, framebuffer::Buffer, map_brightness_to_char,
    model::Model, obj_loader::Object, raster::fill_triangle,
};

pub(crate) struct Pipeline<T> {
    projection_matrix: Mat4,
    objects: Box<[Object]>,
    framebuffer: Framebuffer<T>,
    camera: Camera,
}

impl<T> Pipeline<T>
where
    T: Copy,
{
    pub fn new(
        fov: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
        objects: Box<[Object]>,
        framebuffer: Framebuffer<T>,
        camera: Camera,
    ) -> Self {
        Pipeline::<T> {
            projection_matrix: Mat4::perspective_rh(fov, aspect_ratio, near, far),
            objects,
            framebuffer,
            camera,
        }
    }
}

fn project_to_screen(point: Pos4, framebuffer_width: usize, framebuffer_height: usize) -> Pos3 {
    let ndc = point.truncate() / point.w + f32::EPSILON; // Vec3(x/w, y/w, z/w)

    Vec3A::new(
        (ndc.x + 1.0) * 0.5 * framebuffer_width as f32,
        (1.0 - ndc.y) * 0.5 * framebuffer_height as f32,
        ndc.z,
    )
}

impl Pipeline<char> {
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

        let camera_position = self.camera.get_position();
        let view_matrix = self.camera.get_view_matrix();
        let proj_matrix = self.projection_matrix;

        for object in self.objects.iter_mut() {
            let vertex = object.get_points().to_vec();
            let triangles = object.get_triangles();

            for &(i1, i2, i3) in triangles {
                let a_world = vertex[i1];
                let b_world = vertex[i2];
                let c_world = vertex[i3];

                let a_view = *view_matrix * a_world;
                let b_view = *view_matrix * b_world;
                let c_view = *view_matrix * c_world;

                let normal = Vec3A::from(b_view.xyz() - a_view.xyz())
                    .cross(Vec3A::from(c_view.xyz() - a_view.xyz()))
                    .normalize();

                if normal.z < 0.0 {
                    continue;
                }

                let light_dir_world = camera_position.normalize();

                let light_dir_view = view_matrix
                    .transform_vector3(light_dir_world.into())
                    .normalize();
                let light_dir = Vec3A::from(light_dir_view);
                let brightness = normal.dot(light_dir).clamp(0.0, 1.0);
                let shade = map_brightness_to_char(brightness);

                let va_clip = proj_matrix * a_view;
                let vb_clip = proj_matrix * b_view;
                let vc_clip = proj_matrix * c_view;

                let pa =
                    project_to_screen(va_clip, self.framebuffer.width(), self.framebuffer.height());
                let pb =
                    project_to_screen(vb_clip, self.framebuffer.width(), self.framebuffer.height());
                let pc =
                    project_to_screen(vc_clip, self.framebuffer.width(), self.framebuffer.height());

                fill_triangle(&mut self.framebuffer, pa, pb, pc, shade);
            }
        }

        self.framebuffer.write_io(&mut stdout())
    }
}
