use std::{
    fmt::Write,
    fs,
    time::{Duration, SystemTime},
};

use ch347_rs;
use clap::Parser;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

#[derive(Parser, Clone, Debug)]
#[clap(about = "Read spi flash chip")]
pub struct CmdSpiFlashRead {
    /// output to file
    #[clap(value_parser)]
    file: String,
}

pub fn cli_spi_flash_read(flash_args: &super::CmdSpiFlash, args: &CmdSpiFlashRead) {
    unsafe {
        if ch347_rs::CH347OpenDevice(flash_args.index) == ch347_rs::INVALID_HANDLE_VALUE {
            return;
        }
    }

    let clock_level = match ch347_rs::SpiClockLevel::from_byte(flash_args.freq) {
        None => {
            println!("Unknow SPI clock level: {}", flash_args.freq);
            return;
        }
        Some(level) => level,
    };
    println!("Select SPI Clock: {}", clock_level);

    unsafe {
        let mut spicfg = ch347_rs::SpiConfig::default();
        if ch347_rs::CH347SPI_GetCfg(flash_args.index, &mut spicfg) == 0 {
            println!("CH347SPI_GetCfg Fail");
            return;
        }

        spicfg.clock = flash_args.freq;
        // spicfg.chip_select = 0x80;
        spicfg.byte_order = 1;
        if ch347_rs::CH347SPI_Init(flash_args.index, &mut spicfg) == 0 {
            println!("CH347SPI_Init Fail");
            return;
        }
        // println!("{:#?}", spicfg);

        let device = ch347_rs::Ch347Device::new(flash_args.index).spi_flash();
        let chip_info = match device.delect() {
            Err(e) => {
                println!("{:X?}", e);
                return;
            }
            Ok(chip_info) => chip_info,
        };

        let adjusted_byte =
            byte_unit::Byte::from_bytes(chip_info.capacity as u128).get_appropriate_unit(true);

        println!("ChipInfo:");
        println!("  Manufacturer: {}", chip_info.vendor.name);
        println!("          Name: {}", chip_info.name);
        println!("      Capacity: {}", adjusted_byte);

        let mut all_buf: Vec<u8> = Vec::new();
        let pb = ProgressBar::new(chip_info.capacity as u64);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({binary_bytes_per_sec}) ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

        println!("Reading ...");
        let start_time = SystemTime::now();

        const BLOCK_SIZE: usize = 4096;
        for i in 0..(chip_info.capacity / (BLOCK_SIZE as u32)) {
            let mut rbuf: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
            device.read(i * BLOCK_SIZE as u32, &mut rbuf);
            all_buf.extend_from_slice(&rbuf);
            pb.set_position((i * BLOCK_SIZE as u32) as u64);
        }
        let take_time = start_time.elapsed().unwrap().as_millis();
        let take_time = Duration::from_millis(take_time as u64);
        pb.finish_and_clear();
        fs::write(args.file.as_str(), &all_buf).unwrap();

        println!("Done, Take time: {}", humantime::format_duration(take_time));
        let speed = (all_buf.len() as f64) / take_time.as_secs_f64();
        if speed < (1024.0) {
            println!(
                "{:.2} B/S ",
                (all_buf.len() as f64) / take_time.as_secs_f64()
            );
        } else if speed < (1024.0 * 1024.0) {
            println!(
                "{:.2} KB/S ",
                (all_buf.len() as f64) / take_time.as_secs_f64() / 1024.0
            );
        } else {
            println!(
                "{:.2} MB/S ",
                (all_buf.len() as f64) / take_time.as_secs_f64() / 1024.0 / 1024.0
            );
        }
    }

    unsafe {
        ch347_rs::CH347CloseDevice(flash_args.index);
    }
}
