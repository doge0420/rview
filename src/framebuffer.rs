use std::{fmt, io};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

use crate::Point2D;

#[derive(Debug)]
pub(crate) struct Framebuffer<P> {
    pub pixels: Vec<P>,
    height: usize,
    width: usize,
}

impl<P> Framebuffer<P>
where
    P: Clone,
{
    pub fn new_with(element: P, width: usize, height: usize) -> Self {
        let pixels = vec![element; width * height];

        Framebuffer {
            pixels,
            width,
            height,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&P> {
        self.pixels.get(self.get_index(x, y))
    }

    pub fn set_pixel(&mut self, point: &Point2D, value: P) {
        let x = (point.x.round() + (self.width as f32) / 2.) as usize;
        let y = (point.y.round() + (self.height as f32) / 2.) as usize;

        let idx = self.get_index(x, y);

        if let Some(pixel) = self.pixels.get_mut(idx) {
            *pixel = value;
        }
    }

    pub fn clear(&mut self, value: P) {
        for pixel in &mut self.pixels {
            *pixel = value.clone();
        }
    }

    pub fn write_buffer_fmt<W>(&self, _writer: W)
    where
        W: fmt::Write,
    {
        unimplemented!("Not supported yet :(")
    }
}

impl Framebuffer<char> {
    pub fn write_buffer_io<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: io::Write,
    {
        execute!(writer, MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;

        let mut frame = String::with_capacity(self.width * self.height + (self.height - 1));

        for (row_idx, row) in self.pixels.chunks(self.width).enumerate() {
            for &ch in row {
                frame.push(ch);
            }

            if row_idx + 1 < self.height {
                frame.push('\n');
            }
        }

        write!(writer, "{}", frame)?;
        writer.flush()?;

        Ok(())
    }
}
