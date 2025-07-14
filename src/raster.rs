use crate::{Pos3, framebuffer::Framebuffer};
use glam::{Vec2, Vec3Swizzles};

pub fn fill_triangle(
    framebuffer: &mut Framebuffer<char>,
    p0: Pos3, // Pos3 includes .z
    p1: Pos3,
    p2: Pos3,
    shade: char,
) {
    let min_x = p0.x.min(p1.x).min(p2.x).floor() as i32;
    let max_x = p0.x.max(p1.x).max(p2.x).ceil() as i32;
    let min_y = p0.y.min(p1.y).min(p2.y).floor() as i32;
    let max_y = p0.y.max(p1.y).max(p2.y).ceil() as i32;

    // Precompute triangle area
    let area = edge_function(p0.xy(), p1.xy(), p2.xy());

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

            let w0 = edge_function(p1.xy(), p2.xy(), px);
            let w1 = edge_function(p2.xy(), p0.xy(), px);
            let w2 = edge_function(p0.xy(), p1.xy(), px);

            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                let w0 = w0 / area;
                let w1 = w1 / area;
                let w2 = w2 / area;

                // Interpolate z value
                let z = w0 * p0.z + w1 * p1.z + w2 * p2.z;

                framebuffer.set_pixel_with_depth(&Vec2::new(x as f32, y as f32), shade, z);
            }
        }
    }
}

fn edge_function(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    let ac = c - a;
    let ba = b - a;

    ac.perp_dot(ba)
}
