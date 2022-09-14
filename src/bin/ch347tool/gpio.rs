use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[clap(about = "Operate gpio")]
pub struct CmdGpio {
    /// device number
    #[clap(value_parser)]
    index: u8,

    /// gpio mask, eg. hex: 0xFF or FFH dec:64 bin:0b0000_0011
    #[clap(value_parser)]
    gpio_mask: String,

    #[clap(subcommand, value_enum)]
    command: Commands,
}

#[derive(ValueEnum, Subcommand, Clone, Debug)]
pub enum Commands {
    High,
    Low,
    Read,
}

pub fn cli_operator_gpio(args: &CmdGpio) {
    println!("Select device index: {}", args.index);
    println!("Select gpio mask: {}", args.gpio_mask);
    match args.command {
        Commands::High => {}
        Commands::Low => {}
        Commands::Read => {}
    }
}
