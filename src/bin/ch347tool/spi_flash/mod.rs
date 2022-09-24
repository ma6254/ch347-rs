mod check;
mod detect;
mod erase;
mod read;
mod utils;
mod write;

use clap::{Parser, Subcommand, ValueEnum};

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
    #[clap(short, long, value_parser, default_value_t = 0)]
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
}

pub fn cli_spi_flash(args: &CmdSpiFlash) {
    match &args.command {
        Commands::Detect(sub_args) => detect::cli_spi_flash_detect(args, sub_args),
        Commands::Erase(sub_args) => erase::cli_spi_flash_erase(args, sub_args),
        Commands::Write(sub_args) => write::cli_spi_flash_write(args, sub_args),
        Commands::Read(sub_args) => read::cli_spi_flash_read(args, sub_args),
        Commands::Check(sub_args) => check::cli_spi_flash_check(args, sub_args),
    }
}
