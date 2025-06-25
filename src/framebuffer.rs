use std::{f32, io};

use anyhow::{Context, Result};
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

use crate::Pos2;

pub trait Buffer {
    fn write_io<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: io::Write;
}

#[derive(Debug)]
pub(crate) struct Framebuffer<P> {
    pub pixels: Vec<P>,
    height: usize,
    width: usize,
    depth: Vec<f32>,
    default: P,
}

impl<P> Framebuffer<P>
where
    P: Copy,
{
    pub fn new_with(element: P, width: usize, height: usize, default: P) -> Self {
        let pix_count = width * height;
        let pixels = vec![element; pix_count];
        let depth = vec![f32::INFINITY; pix_count];

        Framebuffer {
            pixels,
            width,
            height,
            depth,
            default,
        }
    }

    #[inline(always)]
    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn _get_depth(&self, x: usize, y: usize) -> Option<&f32> {
        self.depth.get(self.get_index(x, y))
    }

    pub fn set_pixel_with_depth(&mut self, point: &Pos2, value: P, depth: f32) {
        let x = point.x.round() as usize;
        let y = point.y.round() as usize;

        if x >= self.width || y >= self.height {
            return;
        }

        let idx = self.get_index(x, y);

        if depth < self.depth[idx] {
            self.depth[idx] = depth;
            if let Some(pixel) = self.pixels.get_mut(idx) {
                *pixel = value;
            }
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(self.default);
        self.depth.fill(f32::INFINITY);
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width
    }
}

impl Buffer for Framebuffer<char> {
    fn write_io<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        execute!(writer, MoveTo(0, 0), Clear(ClearType::FromCursorDown))
            .context("Couldn't execute crossterm commands.")?;

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
        writer.flush().context("Couldn't flush the terminal.")?;

        Ok(())
    }
}
