use anyhow::{Context, Result};
use clap::{Parser, arg};
use crossterm::{
    cursor::{Hide, Show},
    event::{
        DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind, poll,
        read,
    },
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, SetTitle, disable_raw_mode, enable_raw_mode,
    },
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

const TARGET_FPS: u32 = 200;
const REFRESH_RATE: f32 = 1.0 / TARGET_FPS as f32;
const BACKGROUND: char = ' ';

const MAX_CAM_DISTANCE: f32 = 30.0;
const MIN_CAM_DISTANCE: f32 = 1.0;
const CAM_DISTANCE_STEP: f32 = 0.5;

const PITCH_SENSITIVITY: f32 = -0.1;
const YAW_SENSITIVITY: f32 = 0.1;

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
    let mut yaw: f32 = args.yaw.to_radians();
    let mut pitch: f32 = args.pitch.to_radians();

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

    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        SetTitle("rview")
    )
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
                        if distance < MAX_CAM_DISTANCE {
                            distance += CAM_DISTANCE_STEP;
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        if distance > MIN_CAM_DISTANCE {
                            distance -= CAM_DISTANCE_STEP;
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
