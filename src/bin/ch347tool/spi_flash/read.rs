use std::{
    error::Error,
    fmt::Write,
    fs,
    time::{Duration, SystemTime},
};

use clap::Parser;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use super::utils::format_byte_per_sec;

#[derive(Parser, Clone, Debug)]
#[clap(about = "Read spi flash chip")]
pub struct CmdSpiFlashRead {
    /// output to file
    #[clap(value_parser)]
    file: String,
}

pub fn cli_spi_flash_read(
    flash_args: &super::CmdSpiFlash,
    args: &CmdSpiFlashRead,
) -> Result<(), Box<dyn Error>> {
    let (device, chip_info) = flash_args.init()?;

    let chip_capacity: usize = chip_info.capacity.into();

    let mut all_buf: Vec<u8> = Vec::new();
    let pb = ProgressBar::new(chip_capacity as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({binary_bytes_per_sec}) ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    println!("Reading ...");
    let start_time = SystemTime::now();

    const BLOCK_SIZE: usize = 4096;

    for i in 0..(chip_capacity / BLOCK_SIZE) {
        let mut rbuf: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
        device.read((i * BLOCK_SIZE) as u32, &mut rbuf);
        all_buf.extend_from_slice(&rbuf);
        pb.set_position((i * BLOCK_SIZE) as u64);
    }
    let take_time = start_time.elapsed().unwrap().as_millis();
    let take_time = Duration::from_millis(take_time as u64);
    pb.finish_and_clear();
    fs::write(args.file.as_str(), &all_buf)?;

    println!("Done, Take time: {}", humantime::format_duration(take_time));
    let speed = (all_buf.len() as f64) / take_time.as_secs_f64();
    println!("{}", format_byte_per_sec(speed));

    Ok(())
}
