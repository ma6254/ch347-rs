mod gpio;
mod i2c;
mod list;
mod spi_flash;

use clap::{Parser, Subcommand};
use shadow_rs::shadow;

shadow!(build);

#[derive(Parser)]
#[clap(author, version, about=build::ABOUT_MESSABE, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List(list::CmdListDevice),
    Info,
    Spi,
    SpiFlash(spi_flash::CmdSpiFlash),
    I2cDetect(i2c::CmdI2cDetect),
    I2cDump(i2c::CmdI2cDump),
    Gpio(gpio::CmdGpio),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::List(args) => list::cli_list_device(args),
        Commands::Gpio(args) => gpio::cli_operator_gpio(args),
        Commands::I2cDetect(args) => i2c::cli_i2c_detect(args),
        Commands::I2cDump(args) => i2c::cli_i2c_dump(args),
        Commands::SpiFlash(args) => spi_flash::cli_spi_flash(args),
        _ => {
            println!("undefined command");
        }
    }
}
