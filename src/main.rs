use anyhow::Result;
use std::io::{stdout, Write};

const PPM_VERSION: &'static str = "P6";
const COLOR_SPACE: u8 = u8::MAX;

struct PPMP6<const R: usize, const C: usize> {
    pixels: [[Pixel; C]; R],
}

#[derive(Default, Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl<const R: usize, const C: usize> PPMP6<R, C> {
    fn new() -> PPMP6<R, C> {
        PPMP6 {
            pixels: [[Pixel::default(); C]; R],
        }
    }

    fn write_headers<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        writeln!(writer, "{}", PPM_VERSION)?;
        writeln!(writer, "{} {}", C, R)?;
        writeln!(writer, "#FT 0 0 10")?;
        writeln!(writer, "{}", COLOR_SPACE)?;

        Ok(())
    }

    fn _write<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.write_headers(writer)?;

        for row in self.pixels {
            for pixel in row {
                writer.write(&[pixel.r, pixel.g, pixel.b])?;
            }
        }
        Ok(())
    }

    fn set_color(&mut self, p: &Pixel) {
        for row in self.pixels.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = *p;
            }
        }
    }

    fn print_ascii(&self) {
        for row in self.pixels {
            for pixel in row {
                print!("\x1b[38;2;{};{};{}m█\x1b[0m", pixel.r, pixel.g, pixel.b);
            }
            println!();
        }
    }
}

fn main() -> Result<()> {
    // FT is 30 x 40 pixels in size
    let mut back_buffer = PPMP6::<15, 30>::new();
    let mut front_buffer = PPMP6::<15, 30>::new();

    back_buffer.set_color(&Pixel {
        r: 10,
        g: 80,
        b: 250,
    });

    front_buffer.set_color(&Pixel {
        r: 250,
        g: 80,
        b: 10,
    });

    println!("Back buffer: ");
    back_buffer.write_headers(&mut stdout())?;
    back_buffer.print_ascii();

    println!("Front buffer: ");
    front_buffer.write_headers(&mut stdout())?;
    front_buffer.print_ascii();

    // let udp_socket = UdpSocket::bind("0.0.0.0:0")?;

    // let mut buf = Vec::new();

    // back_buffer.write(&mut buf)?;

    // loop {
    // udp_socket.send_to(&buf, "ft.noise:1337")?;
    // sleep(Duration::from_millis(1));
    // }

    Ok(())
}
