use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, poll, read},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use point::{Point2D, Point3D};
use std::time::Duration;
use terminal_size::{Height, Width, terminal_size};

use crate::framebuffer::Framebuffer;

mod framebuffer;
mod point;

fn project(point: &Point3D, distance: f32) -> Point2D {
    Point2D::from((
        point.x / (point.z + distance),
        point.y / (point.z + distance),
    ))
}

fn main() -> std::io::Result<()> {
    const DISTANCE: f32 = 10.0;
    const SCALE: f32 = 5.0;

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

    // execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;
    execute!(stdout, Hide)?;

    let (Width(w), Height(h)) = terminal_size().unwrap();

    let width = w as usize;
    let height = h as usize;

    let mut framebuffer = Framebuffer::new_with(' ', width, height);

    loop {
        for point in &cube {
            let p2d = project(point, DISTANCE);
            let scaled_p2d = p2d * SCALE;

            framebuffer.set_pixel(&scaled_p2d, '*');
        }

        framebuffer.write_buffer_io(&mut stdout)?;

        if poll(Duration::from_millis(5))? {
            let event = read()?;

            if event == Event::Key(KeyCode::Char('c').into()) {
                break;
            }
        }
    }

    execute!(stdout, Show)?;
    disable_raw_mode()?;
    // execute!(std::io::stdout(), LeaveAlternateScreen)

    Ok(())
}
