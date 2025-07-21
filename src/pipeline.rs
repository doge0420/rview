use std::{f32, io::stdout};

use anyhow::Result;
use glam::{Mat4, Vec3A, Vec4Swizzles};

use crate::{
    Framebuffer, Pos3, Pos4, camera::Camera, framebuffer::Buffer, model::Model, obj_loader::Object,
    raster::fill_triangle,
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
    let ndc = point.truncate() / (point.w + f32::EPSILON); // Vec3(x/w, y/w, z/w)

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

    pub fn render(&mut self) -> Result<()> {
        self.framebuffer.clear();

        let camera_position = self.camera.get_position();
        let view_matrix = self.camera.get_view_matrix();
        let proj_matrix = self.projection_matrix;

        let width = self.framebuffer.width();
        let height = self.framebuffer.height();

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

                let pa = project_to_screen(va_clip, width, height);
                let pb = project_to_screen(vb_clip, width, height);
                let pc = project_to_screen(vc_clip, width, height);

                fill_triangle(&mut self.framebuffer, pa, pb, pc, shade);
            }
        }

        self.framebuffer.write_io(&mut stdout())
    }
}

fn map_brightness_to_char(b: f32) -> char {
    /* from https://stackoverflow.com/a/74186686 */

    const PALETTE: [char; 92] = [
        ' ', '`', '.', '-', '\'', ':', '_', ',', '^', '=', ';', '>', '<', '+', '!', 'r', 'c', '*',
        '/', 'z', '?', 's', 'L', 'T', 'v', ')', 'J', '7', '(', '|', 'F', 'i', '{', 'C', '}', 'f',
        'I', '3', '1', 't', 'l', 'u', '[', 'n', 'e', 'o', 'Z', '5', 'Y', 'x', 'j', 'y', 'a', ']',
        '2', 'E', 'S', 'w', 'q', 'k', 'P', '6', 'h', '9', 'd', '4', 'V', 'p', 'O', 'G', 'b', 'U',
        'A', 'K', 'X', 'H', 'm', '8', 'R', 'D', '#', '$', 'B', 'g', '0', 'M', 'N', 'W', 'Q', '%',
        '&', '@',
    ];

    const BRIGHTNESS_LEVELS: [f32; 92] = [
        0.0000, 0.0751, 0.0829, 0.0848, 0.1227, 0.1403, 0.1559, 0.1850, 0.2183, 0.2417, 0.2571,
        0.2852, 0.2902, 0.2919, 0.3099, 0.3192, 0.3232, 0.3294, 0.3384, 0.3609, 0.3619, 0.3667,
        0.3737, 0.3747, 0.3838, 0.3921, 0.3960, 0.3984, 0.3993, 0.4075, 0.4091, 0.4101, 0.4200,
        0.4230, 0.4247, 0.4274, 0.4293, 0.4328, 0.4382, 0.4385, 0.4420, 0.4473, 0.4477, 0.4503,
        0.4562, 0.4580, 0.4610, 0.4638, 0.4667, 0.4686, 0.4693, 0.4703, 0.4833, 0.4881, 0.4944,
        0.4953, 0.4992, 0.5509, 0.5567, 0.5569, 0.5591, 0.5602, 0.5602, 0.5650, 0.5776, 0.5777,
        0.5818, 0.5870, 0.5972, 0.5999, 0.6043, 0.6049, 0.6093, 0.6099, 0.6465, 0.6561, 0.6595,
        0.6631, 0.6714, 0.6759, 0.6809, 0.6816, 0.6925, 0.7039, 0.7086, 0.7235, 0.7302, 0.7332,
        0.7602, 0.7834, 0.8037, 0.9999,
    ];

    let mut index = BRIGHTNESS_LEVELS.len() - 1; // default to last
    for (i, &lvl) in BRIGHTNESS_LEVELS.iter().enumerate() {
        if b <= lvl {
            index = i;
            break;
        }
    }

    *PALETTE.get(index).unwrap_or(&'▓')
}
