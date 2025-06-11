use std::{f32, io};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

use crate::Pos2;

pub trait Buffer {
    fn write_io<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: io::Write;
}

#[derive(Debug)]
pub(crate) struct Framebuffer<P> {
    pub pixels: Vec<P>,
    height: usize,
    width: usize,
    depth: Vec<f32>,
}

impl<P> Framebuffer<P>
where
    P: Clone,
{
    pub fn new_with(element: P, width: usize, height: usize) -> Self {
        let pix_count = width * height;
        let pixels = vec![element; pix_count];
        let depth = vec![f32::INFINITY; pix_count];

        Framebuffer {
            pixels,
            width,
            height,
            depth,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_depth(&self, x: usize, y: usize) -> Option<&f32> {
        self.depth.get(self.get_index(x, y))
    }

    pub fn set_pixel(&mut self, point: &Pos2, value: P) {
        let x = (point.x.round() + (self.width as f32) / 2.) as usize;
        let y = (point.y.round() + (self.height as f32) / 2.) as usize;

        let idx = self.get_index(x, y);

        if let Some(pixel) = self.pixels.get_mut(idx) {
            *pixel = value;
        }
    }

    pub fn clear(&mut self, value: P) {
        self.pixels.fill(value);
        self.depth.fill(f32::INFINITY);
    }
}

impl Buffer for Framebuffer<char> {
    fn write_io<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: io::Write,
    {
        execute!(writer, MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;

        let mut frame = String::with_capacity(self.width * self.height + (self.height - 1));

        for (row_idx, row) in self.pixels.chunks_exact(self.width).enumerate() {
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
