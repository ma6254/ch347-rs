use ch347_rs;
use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[clap(about = "Detects spi flash chip model")]
pub struct CmdSpiFlashDetect {}

pub fn cli_spi_flash_detect(flash_args: &super::CmdSpiFlash, _args: &CmdSpiFlashDetect) {
    unsafe {
        if ch347_rs::CH347OpenDevice(flash_args.index) == ch347_rs::INVALID_HANDLE_VALUE {
            println!("CH347OpenDevice Fail");
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

        let chip_info = match device.detect() {
            Err(e) => {
                println!("{:X?}", e);
                return;
            }
            Ok(chip_info) => chip_info,
        };

        println!("ChipInfo:");
        println!("  Manufacturer: {}", chip_info.vendor.name);
        println!("          Name: {}", chip_info.name);
        println!("      Capacity: {}", chip_info.capacity);
        println!(
            "           UID: {}",
            match device.read_uuid(chip_info.vendor) {
                Err(e) => format!("{}", e),
                Ok(chip_uuid) => format!("{} Bit {:02X?}", chip_uuid.len() * 8, chip_uuid),
            }
        );

        // let sreg = match device.read_status_register(chip_info.vendor) {
        //     Err(e) => {
        //         println!("{:X?}", e);
        //         return;
        //     }
        //     Ok(a) => a,
        // };
        // println!("{}", sreg);
    }

    unsafe {
        ch347_rs::CH347CloseDevice(flash_args.index);
    }
}
