use anyhow::{Context, Result};
use clap::{Parser, arg};
use crossterm::{
    cursor::{Hide, Show},
    event::{
        DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind, poll,
        read,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use glam::{Quat, Vec2, Vec3, Vec3A, Vec4};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};
use terminal_size::{Height, Width, terminal_size};

use crate::{camera::Camera, framebuffer::Framebuffer, obj_loader::load, pipeline::Pipeline};

mod camera;
mod framebuffer;
mod model;
mod obj_loader;
mod pipeline;
mod raster;

type Pos4 = Vec4;
type Pos3 = Vec3A;
type Pos2 = Vec2;
type Face = (usize, usize, usize);

const TARGET_FPS: f32 = 165.0;
const REFRESH_RATE: f32 = 1.0 / TARGET_FPS;
const BACKGROUND: char = ' ';

const PITCH_SENSITIVITY: f32 = -0.1;
const YAW_SENSITIVITY: f32 = 0.1;

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

#[derive(Parser)]
#[command(about = "A fast cli Wavefront (.obj) file rasterizer 🦀", long_about = None)]
struct Cli {
    /// File path
    file_path: PathBuf,

    /// Yaw of the camera
    #[arg(long, default_value_t = 180.0f32.to_radians())]
    yaw: f32,

    /// Pitch of the camera
    #[arg(long, default_value_t = 0.0)]
    pitch: f32,

    /// Fov
    #[arg(long, default_value_t = 40f32.to_radians())]
    fov: f32,

    /// Near clipping plane
    #[arg(long, default_value_t = 0.01)]
    near: f32,

    /// Far clipping plane
    #[arg(long, default_value_t = 10.0)]
    far: f32,

    /// Distance of the camera from the origin
    #[arg(short, long, default_value_t = 5.0)]
    distance: f32,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut distance = args.distance;

    let mut last_mouse_pos = (0, 0);
    let mut yaw: f32 = args.yaw;
    let mut pitch: f32 = args.pitch;

    let mut stdout = std::io::stdout();

    let (Width(w), Height(h)) =
        terminal_size().context("Couldn't get terminal size with terminal_size.")?;

    let width = w as usize;
    let height = h as usize;

    let fov = args.fov;
    let aspect_ratio = width as f32 / height as f32;

    let near = args.near;
    let far = args.far;

    let path = args
        .file_path
        .to_str()
        .expect("Cannot convert path to string.");

    let objects = Box::new([load(path, Vec3::splat(1.0), Quat::IDENTITY, Vec3::ZERO)
        .with_context(|| format!("Couldn't load {}", path))?]);

    let camera = Camera::new();
    let framebuffer = Framebuffer::new_with(BACKGROUND, width, height, BACKGROUND);
    let mut pipeline = Pipeline::new(fov, aspect_ratio, near, far, objects, framebuffer, camera);

    let mut prev = Instant::now();
    let timer = Instant::now();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Couldn't execute crossterm commands.")?;
    enable_raw_mode().context("Couldn't enter crossterm raw mode.")?;
    execute!(stdout, Hide).context("Couldn't hide cursor with crossterm.")?;

    loop {
        let now = Instant::now();

        let _dt = now.duration_since(prev).as_secs_f32();
        let _t = timer.elapsed().as_secs_f32();

        pipeline.update_radius(distance);
        pipeline.rotate_cam_x(pitch);
        pipeline.rotate_cam_y(yaw);

        pipeline.render().context("Failed to render frame.")?;

        if poll(Duration::from_secs_f32(REFRESH_RATE))? {
            match read().context("Failed to read event with crossterm.")? {
                Event::Key(key_event) => {
                    if key_event == KeyCode::Char('c').into() {
                        break;
                    }
                }
                Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::ScrollDown => {
                        if distance < 30.0 {
                            distance += 0.5
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        if distance > 1.0 {
                            distance -= 0.5
                        }
                    }
                    MouseEventKind::Drag(MouseButton::Left) => {
                        let (new_x, new_y) = (mouse_event.column as i32, mouse_event.row as i32);
                        let (old_x, old_y) = last_mouse_pos;
                        let dx = new_x - old_x;
                        let dy = new_y - old_y;

                        yaw += dx as f32 * YAW_SENSITIVITY;
                        pitch += dy as f32 * PITCH_SENSITIVITY;

                        last_mouse_pos = (new_x, new_y);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        prev = now;
    }

    execute!(stdout, Show).context("Couldn't show cursor with crossterm.")?;
    disable_raw_mode().context("Couldn't exit crossterm raw mode.")?;
    execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)
        .context("Couldn't execute crossterm commands.")?;

    Ok(())
}
