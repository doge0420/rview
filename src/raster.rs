use crate::framebuffer::Framebuffer;
use glam::Vec2;

pub fn fill_triangle(
    framebuffer: &mut Framebuffer<char>,
    p0: Vec2,
    p1: Vec2,
    p2: Vec2,
    shade: char,
) {
    let min_x = p0.x.min(p1.x).min(p2.x).floor() as i32;
    let max_x = p0.x.max(p1.x).max(p2.x).ceil() as i32;
    let min_y = p0.y.min(p1.y).min(p2.y).floor() as i32;
    let max_y = p0.y.max(p1.y).max(p2.y).ceil() as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // if let Some(z) = framebuffer.get_depth(x as usize, y as usize) {
            //     if z < 
            // }

            let px = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            if point_in_triangle(px, p0, p1, p2) {
                framebuffer.set_pixel(&Vec2::new(x as f32, y as f32), shade);
            }
        }
    }
}

fn point_in_triangle(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let v0 = b - a;
    let v1 = c - b;
    let v2 = a - c;
    let w0 = p - a;
    let w1 = p - b;
    let w2 = p - c;

    let c0 = v0.x * w0.y - v0.y * w0.x;
    let c1 = v1.x * w1.y - v1.y * w1.x;
    let c2 = v2.x * w2.y - v2.y * w2.x;

    (c0 >= 0.0 && c1 >= 0.0 && c2 >= 0.0) || (c0 <= 0.0 && c1 <= 0.0 && c2 <= 0.0)
}
