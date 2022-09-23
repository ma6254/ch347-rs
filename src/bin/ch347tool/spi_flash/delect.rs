use ch347_rs;
use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[clap(about = "Detects spi flash chip model")]
pub struct CmdSpiFlashDelect {}

pub fn cli_spi_flash_detect(flash_args: &super::CmdSpiFlash, _args: &CmdSpiFlashDelect) {
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
        // println!("{:#?}", spicfg);

        spicfg.clock = flash_args.freq;
        spicfg.byte_order = 1;
        if ch347_rs::CH347SPI_Init(flash_args.index, &mut spicfg) == 0 {
            println!("CH347SPI_Init Fail");
            return;
        }
        // println!("{:#?}", spicfg);

        let device = ch347_rs::Ch347Device::new(flash_args.index).spi_flash();

        if let Err(e) = device.delect() {
            println!("{:X?}", e);
        }

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
    }

    unsafe {
        ch347_rs::CH347CloseDevice(flash_args.index);
    }
}