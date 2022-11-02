mod ppmp6;

use std::{
    io::{stdin, stdout, Write},
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
    samples_fft_to_spectrum, scaling::scale_20_times_log10, windows::hann_window,
};

const ROWS: usize = 35;
const COLS: usize = 45;

fn main() -> Result<()> {
    // FT is 30 x 40 pixels in size
    let back_buffer = PPMP6::<ROWS, COLS>::new(Pixel::new(10, 80, 250));
    let front_buffer = PPMP6::<ROWS, COLS>::new(Pixel::new(250, 80, 10));

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

    let stream = device.build_input_stream(&config, print_stuff(sample_rate), err_fn)?;

    stream.play()?;

    sleep(Duration::from_secs(20));

    Ok(())
}

fn print_stuff(samples_per_sec: SampleRate) -> impl FnMut(&[f32], &InputCallbackInfo) {
    let samples_rate = samples_per_sec.0;

    let start = Instant::now();

    move |data, _info| {
        let now = Instant::now();
        let color_idx = now.duration_since(start).as_secs_f32() * 10.; //Speed up the cycle a lil

        let hann_window = hann_window(data);

        let spectrum_hann_window = samples_fft_to_spectrum(
            &hann_window,
            samples_rate,
            spectrum_analyzer::FrequencyLimit::All,
            Some(&scale_20_times_log10),
        )
        .unwrap();

        let mut data = PPMP6::<ROWS, COLS>::new(Pixel::new(0, 0, 0));

        let freq_data = spectrum_hann_window.data();
        let chunk_size = freq_data.len() / COLS;

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
                data.set_col(idx, avg, color_gradient(color_idx));
            }
        }
        print!("\x1b[2J\x1b[H{}", data);
        sleep(Duration::from_millis(50));

        println!();
        println!();
    }
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
