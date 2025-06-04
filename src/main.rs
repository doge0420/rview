use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, poll, read},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io::{Write, stdout};
use std::time::Duration;
use terminal_size::{Height, Width, terminal_size};

fn flush_buffer(buffer: &Vec<Vec<char>>) -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0))?;

    let last_row = buffer.len() - 1;

    for (i, row) in buffer.iter().enumerate() {
        for &ch in row {
            write!(stdout, "{}", ch)?;
        }
        if i != last_row {
            write!(stdout, "\n")?;
        }
    }

    stdout.flush()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    execute!(std::io::stdout(), EnterAlternateScreen)?;

    let (Width(w), Height(h)) = terminal_size().unwrap();

    let width = w as usize;
    let height = h as usize;

    let mut buffer = vec![vec![' '; width]; height];

    enable_raw_mode()?;
    execute!(stdout(), Hide)?;

    for x in 0..width {
        // Clear buffer
        for row in &mut buffer {
            for cell in row.iter_mut() {
                *cell = ' ';
            }
        }

        // Draw a star moving horizontally
        buffer[height / 2][x] = '*';

        flush_buffer(&buffer)?;

        if poll(Duration::from_millis(5))? {
            let event = read()?;

            if event == Event::Key(KeyCode::Char('c').into()) {
                break;
            }
        }
    }

    execute!(stdout(), Show)?;
    disable_raw_mode()?;

    execute!(std::io::stdout(), LeaveAlternateScreen)
}
