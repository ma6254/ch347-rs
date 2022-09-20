mod gpio;
mod i2c;
mod list;

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
    I2cDelect(i2c::CmdI2cDelect),
    I2cDump(i2c::CmdI2cDump),
    Gpio(gpio::CmdGpio),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::List(args) => list::cli_list_device(args),
        Commands::Gpio(args) => gpio::cli_operator_gpio(args),
        Commands::I2cDelect(args) => i2c::cli_i2c_delect(args),
        Commands::I2cDump(args) => i2c::cli_i2c_dump(args),
        _ => {
            println!("undefined command");
        }
    }
}
