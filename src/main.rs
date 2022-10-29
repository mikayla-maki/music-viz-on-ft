use anyhow::Result;
use std::io::{stdout, Write};

const PPM_VERSION: &'static str = "P6";
const COLOR_SPACE: u8 = u8::MAX;

struct PPMP6 {
    height: u8,
    width: u8,
}

impl PPMP6 {
    fn new(width: u8, height: u8) -> PPMP6 {
        PPMP6 { height, width }
    }

    fn write_headers<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        writeln!(writer, "{}", PPM_VERSION)?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "#FT 0 0 10")?;
        writeln!(writer, "{}", COLOR_SPACE)?;

        Ok(())
    }

    fn write<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.write_headers(writer)?;

        for _row in 0..self.height {
            for _col in 0..self.width {
                let color = u8::MAX / 2;
                writer.write(&[color, color, color])?;
            }
        }
        Ok(())
    }

    fn print_ascii(&self) {
        for _row in 0..self.height {
            for _col in 0..self.width {
                let color = u8::MAX / 2;
                print!("\x1b[38;2;{};{};{}m█\x1b[0m", color, color + 20, color - 20);
            }
            println!();
        }
    }
}

fn main() -> Result<()> {
    // FT is 30 x 40 pixels in size
    let back_buffer = PPMP6::new(30, 15);

    // let udp_socket = UdpSocket::bind("0.0.0.0:0")?;

    // let mut buf = Vec::new();

    back_buffer.write_headers(&mut stdout())?;
    // back_buffer.write(&mut buf)?;

    // loop {
    // udp_socket.send_to(&buf, "ft.noise:1337")?;
    // sleep(Duration::from_millis(1));
    // }

    back_buffer.print_ascii();

    Ok(())
}
