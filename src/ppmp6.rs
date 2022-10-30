use anyhow::Result;
use std::{fmt::Display, io::Write};

const PPM_VERSION: &'static str = "P6";
const COLOR_SPACE: u8 = u8::MAX;

pub struct PPMP6<const R: usize, const C: usize> {
    pixels: [[Pixel; C]; R],
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b }
    }
}

impl<const R: usize, const C: usize> PPMP6<R, C> {
    pub fn new(p: Pixel) -> PPMP6<R, C> {
        PPMP6 {
            pixels: [[p; C]; R],
        }
    }

    fn _write_headers<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        writeln!(writer, "{}", PPM_VERSION)?;
        writeln!(writer, "{} {}", C, R)?;
        writeln!(writer, "#FT 0 0 100")?;
        writeln!(writer, "{}", COLOR_SPACE)?;

        Ok(())
    }

    fn format_headers<W>(&self, writer: &mut W) -> Result<()>
    where
        W: std::fmt::Write,
    {
        writeln!(writer, "{}", PPM_VERSION)?;
        writeln!(writer, "{} {}", C, R)?;
        writeln!(writer, "#FT 0 0 100")?;
        writeln!(writer, "{}", COLOR_SPACE)?;

        Ok(())
    }

    pub fn _write<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        self._write_headers(writer)?;

        for row in self.pixels {
            for pixel in row {
                writer.write(&[pixel.r, pixel.g, pixel.b])?;
            }
        }
        Ok(())
    }

    pub fn set_col(&mut self, col_index: usize, height: f32, p: Pixel) {
        if col_index >= C {
            return; // Just skip extra columns
        }
        // dbg!(height);

        if height > 1. || height < 0. {
            return;
        }
        assert!(0. <= height && height <= 1.);

        let row = (height * R as f32).round() as usize;

        for i in 0..(R - row) {
            self.pixels[R - i - 1][col_index] = p;
        }
    }
}

impl<const R: usize, const C: usize> Display for PPMP6<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_headers(f).map_err(|_| std::fmt::Error)?;
        for row in self.pixels {
            for pixel in row {
                write!(f, "\x1b[38;2;{};{};{}m█\x1b[0m", pixel.r, pixel.g, pixel.b)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
