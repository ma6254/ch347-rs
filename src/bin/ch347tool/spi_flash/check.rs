use ch347_rs;
use clap::Parser;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;

use std::fs;

#[derive(Parser, Clone, Debug)]
#[clap(about = "Check spi flash chip memory")]
pub struct CmdSpiFlashCheck {
    /// output to file
    #[clap(value_parser)]
    file: String,
}

pub fn cli_spi_flash_check(flash_args: &super::CmdSpiFlash, args: &CmdSpiFlashCheck) {
    let file_buf = match fs::read(args.file.as_str()) {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(file_buf) => file_buf,
    };

    unsafe {
        if ch347_rs::CH347OpenDevice(flash_args.index) == ch347_rs::INVALID_HANDLE_VALUE {
            return;
        }
    }

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

        // let mut all_buf: Vec<u8> = Vec::new();
        let pb = ProgressBar::new(chip_info.capacity as u64);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({binary_bytes_per_sec}) ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

        println!("Checking...");
        const BLOCK_SIZE: usize = 4096;
        for i in (0..chip_info.capacity as usize).step_by(BLOCK_SIZE) {
            let mut rbuf: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
            device.read(i as u32, &mut rbuf);
            // all_buf.extend_from_slice(&rbuf);

            for x in 0..BLOCK_SIZE {
                if rbuf[x] != file_buf[i + x] {
                    pb.finish_and_clear();
                    println!(
                        "diff 0x{:04X}_{:04X} {:02X} => {:02X}",
                        (i + x) >> 16,
                        (i + x) & 0xFFFF,
                        file_buf[i + x],
                        rbuf[x]
                    );
                    return;
                }
            }
            pb.set_position(i as u64);
            // pb.inc(BLOCK_SIZE as u64);
        }
        pb.finish();
    }

    unsafe {
        ch347_rs::CH347CloseDevice(flash_args.index);
    }
}
