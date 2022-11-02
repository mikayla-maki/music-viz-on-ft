mod ppmp6;

use std::{
    io::{stdin, stdout, Write},
    net::UdpSocket,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::ppmp6::{Pixel, PPMP6};

use anyhow::{anyhow, Result};
use cpal::{
    default_host,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, InputCallbackInfo, SampleRate,
};

use spectrum_analyzer::{
    samples_fft_to_spectrum,
    scaling::{divide_by_N, scale_20_times_log10, scale_to_zero_to_one},
    windows::hann_window,
};

const ROWS: usize = 35;
const COLS: usize = 45;

fn main() -> Result<()> {
    // FT is 30 x 40 pixels in size
    let back_buffer = PPMP6::<ROWS, COLS>::new(Pixel::black(), 15);
    let front_buffer = PPMP6::<ROWS, COLS>::new(Pixel::new(250, 80, 10), 14);

    println!("Back buffer: \n{}", back_buffer);

    println!("Front buffer: \n{}", front_buffer);

    let host = default_host();

    let devices = host.devices()?.collect::<Vec<Device>>();

    println!("Select an audio device to use as input: ");
    for (idx, device) in devices
        .iter()
        .filter(|device| device.default_input_config().is_ok())
        .enumerate()
    {
        println!(" {idx} - {:?}", device.name()?);
    }
    print!(" > ");
    stdout().flush()?;

    let mut answer = String::new();
    stdin().read_line(&mut answer)?;

    let idx = str::parse::<usize>(answer.trim())?;

    let device = devices
        .get(idx)
        .ok_or(anyhow!("No device matching that selection"))?;

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let supported_config = device.default_input_config()?;

    let sample_rate = supported_config.sample_rate();
    let config = supported_config.config();

    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let stream = device.build_input_stream(&config, print_stuff(sample_rate, socket), err_fn)?;

    stream.play()?;

    sleep(Duration::from_secs(20));

    Ok(())
}

fn print_stuff(
    samples_per_sec: SampleRate,
    socket: UdpSocket,
) -> impl FnMut(&[f32], &InputCallbackInfo) {
    let samples_rate = samples_per_sec.0;

    let start = Instant::now();

    let back_frame = PPMP6::<ROWS, COLS>::new(Pixel::black(), 10);
    send_to_big_ft(&socket, &back_frame);

    let mut frame = PPMP6::<ROWS, COLS>::new(Pixel::new(0, 0, 0), 26);

    move |data, _info| {
        let now = Instant::now();
        let color_idx = now.duration_since(start).as_secs_f32() * 20.; //Speed up the cycle a lil

        let hann_window = hann_window(data);

        let spectrum_hann_window = samples_fft_to_spectrum(
            &hann_window,
            samples_rate,
            spectrum_analyzer::FrequencyLimit::All,
            Some(&scale_20_times_log10),
        )
        .unwrap();

        let freq_data = spectrum_hann_window.data();
        let chunk_size = freq_data.len() / COLS;
        let color = color_gradient(color_idx);

        for (idx, chunk) in freq_data.chunks(chunk_size).enumerate() {
            // let first_freq = chunk.first().map(|f| f.0);
            // let last_freq = chunk.last().map(|f| f.0);
            let avg = chunk
                .iter()
                .map(|freq| freq.1.val())
                .reduce(|total, freq| total + freq)
                .map(|freq| freq / (chunk_size as f32));

            if let Some(avg) = avg {
                let avg = avg / -100.;
                frame.set_col(idx, avg, color);
            }
        }

        print!("\x1b[2J\x1b[H{}", frame);
        println!("sent frame with {:?}", color);
        // send_to_big_ft(&socket, &back_frame);
        send_to_big_ft(&socket, &frame);
        sleep(Duration::from_millis(10));
    }
}

fn send_to_big_ft<const R: usize, const C: usize>(socket: &UdpSocket, frame: &PPMP6<R, C>) {
    let mut buf = vec![];
    frame.write(&mut buf).ok();
    socket
        .send_to(&buf, "ft.noise:1337")
        .map_err(|err| dbg!(err))
        .ok();
}

fn color_gradient(idx: f32) -> Pixel {
    //Pick new values for these constants: https://krazydad.com/tutorials/makecolors.php
    const F1: f32 = 0.3;
    const F2: f32 = 0.3;
    const F3: f32 = 0.3;
    const PHASE_1: f32 = 0.;
    const PHASE_2: f32 = 2.;
    const PHASE_3: f32 = 4.;
    const CENTER: u8 = 128;
    const WIDTH: u8 = 127;

    let red = ((F1 * idx as f32 + PHASE_1).sin() * WIDTH as f32 + CENTER as f32) as u8;
    let grn = ((F2 * idx as f32 + PHASE_2).sin() * WIDTH as f32 + CENTER as f32) as u8;
    let blu = ((F3 * idx as f32 + PHASE_3).sin() * WIDTH as f32 + CENTER as f32) as u8;

    Pixel::new(red, grn, blu)
}
