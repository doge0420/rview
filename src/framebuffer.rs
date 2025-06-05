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
        let mut pixels = Vec::with_capacity(width * height);
        pixels.fill(element);

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
        let x = (point.x.round() + 10.) as usize;
        let y = (point.y.round() + 10.) as usize;

        // println!("{} {}", x, y);

        let idx = self.get_index(x, y);

        if let Some(pixel) = self.pixels.get_mut(idx) {
            *pixel = value;
        }
    }

    pub fn clear(&mut self) {
        self.pixels.clear();
    }

    pub fn write_buffer_fmt<W>(&self, _writer: W)
    where
        W: fmt::Write,
    {
        unimplemented!("Unsupported buffer :(")
    }
}

impl Framebuffer<char> {
    pub fn write_buffer_io<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: io::Write,
    {
        execute!(writer, Clear(ClearType::All), MoveTo(0, 0))?;
        self.clear();

        for row in self.pixels.chunks(self.width) {
            let mut line = String::new();

            for &ch in row {
                line.push(ch);
            }
            write!(writer, "{}\n", line)?;
        }

        writer.flush()?;

        Ok(())
    }
}
