mod ppmp6;

use crate::ppmp6::{Pixel, PPMP6};
use anyhow::Result;

fn main() -> Result<()> {
    // FT is 30 x 40 pixels in size
    let mut back_buffer = PPMP6::<15, 30>::new();
    let mut front_buffer = PPMP6::<15, 30>::new();

    back_buffer.set_color(&Pixel::new(10, 80, 250));

    front_buffer.set_color(&Pixel::new(250, 80, 10));

    println!("Back buffer: \n {}", back_buffer);

    println!("Front buffer: \n {}", front_buffer);

    // let udp_socket = UdpSocket::bind("0.0.0.0:0")?;

    // let mut buf = Vec::new();

    // back_buffer.write(&mut buf)?;

    // loop {
    // udp_socket.send_to(&buf, "ft.noise:1337")?;
    // sleep(Duration::from_millis(1));
    // }

    Ok(())
}
