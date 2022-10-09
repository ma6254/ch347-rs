use std::error::Error;

use clap::{Parser, Subcommand, ValueEnum};

mod utils;

mod check;
mod detect;
mod erase;
mod read;
mod reg;
mod write;

#[derive(Parser, Debug)]
#[clap(about = "Operate spi flash chip")]
pub struct CmdSpiFlash {
    /// device number
    #[clap(value_parser, default_value_t = 0)]
    index: u32,

    /// chip select
    #[clap(value_enum, value_parser,default_value_t=CS::CS0)]
    cs: CS,

    /// clock freq, 0=60MHz 1=30MHz 2=15MHz 3=7.5MHz 4=3.75MHz 5=1.875MHz 6=937.5KHz 7=468.75KHz
    #[clap(short, long, value_parser, default_value_t = 2)]
    freq: u8,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Subcommand, Clone, Debug)]
enum CS {
    CS0,
    CS1,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Detect(detect::CmdSpiFlashDetect),
    Erase(erase::CmdSpiFlashErase),
    Write(write::CmdSpiFlashWrite),
    Read(read::CmdSpiFlashRead),
    Check(check::CmdSpiFlashCheck),
    Reg(reg::CmdReg),
}

impl CmdSpiFlash {
    pub fn init(
        &self,
    ) -> Result<(ch347_rs::SpiFlash<ch347_rs::Ch347Device>, ch347_rs::Chip), Box<dyn Error>> {
        let clock_level = match ch347_rs::SpiClockLevel::from_byte(self.freq) {
            None => {
                return Err(format!("Unknow SPI clock level: {}", self.freq).into());
            }
            Some(level) => level,
        };
        println!("Select SPI Clock: {}", clock_level);

        let mut device = ch347_rs::Ch347Device::new(self.index)?;
        device.change_spi_raw_config(|spi_cfg| {
            spi_cfg.byte_order = 1;
            spi_cfg.clock = self.freq;
        })?;
        let device = device.spi_flash()?;

        let chip_info = match device.detect() {
            Err(e) => return Err(e.into()),
            Ok(chip_info) => chip_info,
        };

        let unique_id = match device.read_uuid(chip_info.vendor) {
            Err(e) => format!("{}: {}", console::style("error").red(), e),
            Ok(chip_uuid) => format!("{} Bit {:02X?}", chip_uuid.len() * 8, chip_uuid),
        };

        println!("ChipInfo:");
        println!("  Manufacturer: {}", chip_info.vendor.name);
        println!("          Name: {}", chip_info.name);
        println!("      Capacity: {}", chip_info.capacity);
        println!("           UID: {}", unique_id);

        Ok((device, chip_info))
    }
}

pub fn cli_spi_flash(args: &CmdSpiFlash) -> Result<(), Box<dyn Error>> {
    match &args.command {
        Commands::Detect(sub_args) => detect::cli_spi_flash_detect(args, sub_args)?,
        Commands::Erase(sub_args) => erase::cli_spi_flash_erase(args, sub_args)?,
        Commands::Write(sub_args) => write::cli_spi_flash_write(args, sub_args)?,
        Commands::Read(sub_args) => read::cli_spi_flash_read(args, sub_args)?,
        Commands::Check(sub_args) => check::cli_spi_flash_check(args, sub_args)?,
        Commands::Reg(sub_args) => reg::cli_main(args, sub_args)?,
    };

    Ok(())
}
