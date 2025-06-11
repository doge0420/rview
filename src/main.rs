use crossterm::{
    cursor::{Hide, Show},
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind, poll, read},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use glam::{Mat3A, Vec2, Vec3, Vec3A};
use std::time::{Duration, Instant};
use terminal_size::{Height, Width, terminal_size};

use crate::{
    framebuffer::{Buffer, Framebuffer},
    raster::fill_triangle,
    shape::{Shape, ShapeData},
};

mod framebuffer;
mod raster;
mod shape;

type Pos3 = Vec3A;
type Pos2 = Vec2;

const TARGET_FPS: f32 = 165.0;
const REFRESH_RATE: f32 = 1.0 / TARGET_FPS;
const BACKGROUND: char = ' ';

fn project(point: &Pos3, distance: f32, scale: f32) -> Pos2 {
    Pos2::from((
        (point.x / (point.z + distance)) * scale,
        (point.y / (point.z + distance)) * scale,
    ))
}

fn rotation_matrix(t: f32) -> Mat3A {
    // let rot_x = Mat3A::from_rotation_x(t);
    let rot_z = Mat3A::from_rotation_z(t);

    // let rotation = rot_x * rot_z;

    // rotation
    rot_z
}

fn map_brightness_to_char(b: f32) -> char {
    /* from https://stackoverflow.com/a/74186686 */

    const PALETTE: &str = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";

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

    debug_assert_eq!(PALETTE.chars().count(), BRIGHTNESS_LEVELS.len());

    let mut index = BRIGHTNESS_LEVELS.len() - 1; // default to last
    for (i, &lvl) in BRIGHTNESS_LEVELS.iter().enumerate() {
        if b <= lvl {
            index = i;
            break;
        }
    }

    PALETTE.chars().nth(index).unwrap_or('▓')
}

fn main() -> std::io::Result<()> {
    let mut distance = 5.0;
    const SCALE: f32 = 25.0;

    let cube = ShapeData::new(
        &[
            (-1.0, -1.0, -1.0), // 0
            (1.0, -1.0, -1.0),  // 1
            (1.0, 1.0, -1.0),   // 2
            (-1.0, 1.0, -1.0),  // 3
            (-1.0, -1.0, 1.0),  // 4
            (1.0, -1.0, 1.0),   // 5
            (1.0, 1.0, 1.0),    // 6
            (-1.0, 1.0, 1.0),   // 7
        ],
        &[
            // Front face (Z = -1)
            (0, 1, 2),
            (0, 2, 3),
            // Back face (Z = +1)
            (5, 7, 6),
            (5, 4, 7),
            // Bottom face (Y = -1)
            (0, 4, 5),
            (0, 5, 1),
            // Top face (Y = +1)
            (3, 2, 6),
            (3, 6, 7),
            // Right face (X = +1)
            (1, 5, 6),
            (1, 6, 2),
            // Left face (X = -1)
            (4, 0, 3),
            (4, 3, 7),
        ],
    );

    let mut stdout = std::io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    execute!(stdout, Hide)?;

    let (Width(w), Height(h)) = terminal_size().unwrap();

    let width = w as usize;
    let height = h as usize;

    let mut framebuffer = Framebuffer::new_with(BACKGROUND, width, height);

    let mut prev = Instant::now();
    let timer = Instant::now();

    loop {
        let now = Instant::now();

        let _dt = now.duration_since(prev).as_secs_f32();
        let t = timer.elapsed().as_secs_f32();

        let rotation_matrix = rotation_matrix(t / 2.0);

        framebuffer.clear(BACKGROUND);

        let transformed: Vec<Pos3> = cube
            .get_points()
            .iter()
            .map(|&p| rotation_matrix * p) // rotate each 3D point
            .collect();

        for &(i1, i2, i3) in cube.get_triangles() {
            let a = transformed[i1];
            let b = transformed[i2];
            let c = transformed[i3];

            let e1 = b - a;
            let e2 = c - a;
            let normal = e1.cross(e2).normalize();

            let cam_dir = Vec3A::new(0.0, 0.0, 1.0);
            if normal.dot(cam_dir) < 0.0 {
                continue;
            }

            let light_dir = Vec3A::new(1.0, 0.0, 1.0).normalize();
            let brightness = normal.dot(light_dir).clamp(0.0, 1.0);

            let shade = map_brightness_to_char(brightness);

            let pa = project(&a, distance, SCALE);
            let pb = project(&b, distance, SCALE);
            let pc = project(&c, distance, SCALE);

            fill_triangle(&mut framebuffer, pa, pb, pc, shade);
        }

        framebuffer.write_io(&mut stdout)?;

        if poll(Duration::from_secs_f32(REFRESH_RATE))? {
            match read()? {
                Event::Key(key_event) => {
                    if key_event == KeyCode::Char('c').into() {
                        break;
                    }
                }
                Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::ScrollDown => {
                        if distance < 30.0 {
                            distance += 1.0
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        if distance > 2.0 {
                            distance -= 1.0
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        prev = now;
    }

    execute!(stdout, Show)?;
    disable_raw_mode()?;
    execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}
