use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, poll, read},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use point::{Point2D, Point3D};
use std::time::{Duration, Instant};
use terminal_size::{Height, Width, terminal_size};

use crate::{framebuffer::Framebuffer, mat3::Mat3};

mod framebuffer;
mod mat3;
mod point;

fn project(point: &Point3D, distance: f32) -> Point2D {
    Point2D::from((
        point.x / (point.z + distance),
        point.y / (point.z + distance),
    ))
}

fn main() -> std::io::Result<()> {
    const DISTANCE: f32 = 5.0;
    const SCALE: f32 = 25.0;

    let cube: [Point3D; 8] = [
        Point3D::from((-1.0, -1.0, -1.0)),
        Point3D::from((1.0, -1.0, -1.0)),
        Point3D::from((1.0, 1.0, -1.0)),
        Point3D::from((-1.0, 1.0, -1.0)),
        Point3D::from((-1.0, -1.0, 1.0)),
        Point3D::from((1.0, -1.0, 1.0)),
        Point3D::from((1.0, 1.0, 1.0)),
        Point3D::from((-1.0, 1.0, 1.0)),
    ];

    let mut stdout = std::io::stdout();

    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;
    execute!(stdout, Hide)?;

    let (Width(w), Height(h)) = terminal_size().unwrap();

    let width = w as usize;
    let height = h as usize;

    let mut framebuffer = Framebuffer::new_with(' ', width, height);
    let rot_mat = Mat3::new([
        |theta: f32| theta.cos(),                // [0,0]
        |theta: f32| -theta.sin() * theta.cos(), // [0,1]
        |theta: f32| theta.sin() * theta.sin(),  // [0,2]
        |theta: f32| theta.sin(),                // [1,0]
        |theta: f32| theta.cos() * theta.cos(),  // [1,1]
        |theta: f32| -theta.cos() * theta.sin(), // [1,2]
        |_: f32| 0.0,                            // [2,0]
        |theta: f32| theta.sin(),                // [2,1]
        |theta: f32| theta.cos(),                // [2,2]
    ]);

    let mut prev = Instant::now();
    let timer = Instant::now();

    loop {
        let now = Instant::now();
        let _dt = now.duration_since(prev).as_secs_f32();

        framebuffer.clear(' ');

        for point in &cube {
            let rot_point = point.mul_mat(&rot_mat, timer.elapsed().as_secs_f32());
            let point_2d = project(&rot_point, DISTANCE);
            let scaled_point_2d = point_2d * SCALE;

            framebuffer.set_pixel(&scaled_point_2d, '+');
        }

        framebuffer.write_buffer_io(&mut stdout)?;

        if poll(Duration::from_millis(5))? {
            let event = read()?;
            if event == Event::Key(KeyCode::Char('c').into()) {
                break;
            }
        }

        prev = now;
    }

    execute!(stdout, Show)?;
    disable_raw_mode()?;
    execute!(std::io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
