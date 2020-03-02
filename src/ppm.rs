
use std::{
    io::prelude::*,
    cmp::max,
};

use crate::color::Color;

#[derive(Debug, Clone)]
pub struct PPM {
    data: Vec<Vec<Color>>,
}

impl PPM {
    pub fn new() -> PPM {
        PPM {
            data: Vec::new(),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let (width, height) = self.dimensions();
        if x + 1 > width || y + 1 > height {
            self.reserve(x + 1, y + 1);
        }

        self.data[y][x] = color
    }

    pub fn dimensions(&self) -> (usize, usize) {
        let height = self.data.len();
        if height == 0 {
            (0, 0)
        } else {
            (self.data[0].len(), height)
        }
    }

    pub fn reserve(&mut self, width: usize, height: usize) {
        self.data.resize(max(height, self.data.len()), Vec::new());
        for row in &mut self.data {
            row.resize(max(width, row.len()), Default::default());
        }
    }

    pub fn write<W: Write>(&self, mut w: W) -> std::io::Result<()> {
        let (width, height) = self.dimensions();
        write!(w, "P3\n")?;
        write!(w, "{} {}\n", width, height)?;
        write!(w, "255\n")?;

        for row in &self.data {
            for color in row {
                match color {
                    Color::Bit8(r, g, b) => {
                        write!(w, "{} {} {}\n", r, g, b)?;
                    }
                }
            }
        }

        Ok(())
    }
}
