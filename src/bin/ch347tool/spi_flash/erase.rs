use std::time::{Duration, SystemTime};

use ch347_rs;
use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[clap(about = "Erase spi flash chip")]
pub struct CmdSpiFlashErase {}

pub fn cli_spi_flash_erase(flash_args: &super::CmdSpiFlash, _args: &CmdSpiFlashErase) {
    unsafe {
        if ch347_rs::CH347OpenDevice(flash_args.index) == ch347_rs::INVALID_HANDLE_VALUE {
            println!("CH347OpenDevice Fail");
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

    let mut spicfg = ch347_rs::SpiConfig::default();

    unsafe {
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
    }

    let device = ch347_rs::Ch347Device::new(flash_args.index).spi_flash();
    let chip_info = match device.detect() {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(chip_info) => chip_info,
    };

    println!("ChipInfo:");
    println!("  Manufacturer: {}", chip_info.vendor.name);
    println!("          Name: {}", chip_info.name);
    println!("      Capacity: {}", chip_info.capacity);

    println!("Start Erase Full Chip ...");
    let start_time = SystemTime::now();

    if let Err(e) = device.erase_full() {
        println!("{:X?}", e);
        return;
    }

    let take_time = start_time.elapsed().unwrap().as_millis();
    let take_time = Duration::from_millis(take_time as u64);
    println!("Done, Take time: {}", humantime::format_duration(take_time));

    unsafe {
        ch347_rs::CH347CloseDevice(flash_args.index);
    }
}
