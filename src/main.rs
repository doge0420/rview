use anyhow::{Context, Result};
use clap::Parser;
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
use std::time::{Duration, Instant};
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

#[derive(Parser)]
#[command(about = "A fast terminal 3D rasterizer 🦀", long_about = None)]
struct Cli {
    file_path: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut distance = 5.0;

    let mut last_mouse_pos = (0, 0);
    let mut yaw: f32 = 180.0f32.to_radians();
    let mut pitch: f32 = 0.0;

    let mut stdout = std::io::stdout();

    let (Width(w), Height(h)) =
        terminal_size().context("Couldn't get terminal size with terminal_size.")?;

    let width = w as usize;
    let height = h as usize;

    let fov = 40f32.to_radians();
    let aspect_ratio = width as f32 / height as f32;

    let near = 0.01;
    let far = 10.0;

    let objects = Box::new([load(
        &args.file_path,
        Vec3::splat(1.0),
        Quat::IDENTITY,
        Vec3::ZERO,
    )
    .with_context(|| format!("Couldn't find {}", &args.file_path))?]);

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
