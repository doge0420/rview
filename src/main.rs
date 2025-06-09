use crossterm::{
    cursor::{Hide, Show},
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind, poll, read},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use glam::{Mat3, Vec2, Vec3};
use std::time::{Duration, Instant};
use terminal_size::{Height, Width, terminal_size};

use crate::{
    framebuffer::{Buffer, Framebuffer},
    shape::{Shape, ShapeData},
};

mod framebuffer;
mod shape;

type Pos3 = Vec3;
type Pos2 = Vec2;

fn project(point: &Pos3, distance: f32) -> Pos2 {
    Pos2::from((
        point.x / (point.z + distance),
        point.y / (point.z + distance),
    ))
}

fn rotation_matrix(t: f32) -> Mat3 {
    let rot_x = Mat3::from_rotation_x(t);
    let rot_z = Mat3::from_rotation_z(t);

    let rotation = rot_x * rot_z;

    rotation
}

fn main() -> std::io::Result<()> {
    let mut distance = 5.0;
    const SCALE: f32 = 25.0;

    let cube = ShapeData::new(
        &[
            (-1.0, -1.0, -1.0),
            (1.0, -1.0, -1.0),
            (1.0, 1.0, -1.0),
            (-1.0, 1.0, -1.0),
            (-1.0, -1.0, 1.0),
            (1.0, -1.0, 1.0),
            (1.0, 1.0, 1.0),
            (-1.0, 1.0, 1.0),
        ],
        &[
            (0, 1, 2),
            (0, 2, 3), // front
            (4, 5, 6),
            (4, 6, 7), // back
            (0, 1, 5),
            (0, 5, 4), // bottom
            (3, 2, 6),
            (3, 6, 7), // top
            (1, 2, 6),
            (1, 6, 5), // right
            (0, 3, 7),
            (0, 7, 4), // left
        ],
    );

    let mut stdout = std::io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    execute!(stdout, Hide)?;

    let (Width(w), Height(h)) = terminal_size().unwrap();

    let width = w as usize;
    let height = h as usize;

    let mut framebuffer = Framebuffer::new_with(' ', width, height);

    let mut prev = Instant::now();
    let timer = Instant::now();

    loop {
        let now = Instant::now();
        let _dt = now.duration_since(prev).as_secs_f32();

        let rotation_matrix = rotation_matrix(timer.elapsed().as_secs_f32());

        framebuffer.clear(' ');

        for point in cube.get_points() {
            let rot_point = rotation_matrix * *point;
            let point_2d = project(&rot_point, distance);
            let scaled_point_2d = point_2d * SCALE;

            framebuffer.set_pixel(&scaled_point_2d, '+');
        }

        framebuffer.write_io(&mut stdout)?;

        if poll(Duration::from_millis(5))? {
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
