use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[clap(about = "Operate gpio")]
pub struct CmdGpio {
    /// device number
    #[clap(value_parser)]
    index: u32,

    /// gpio mask, eg. hex: 0xFF or FFH dec:64 bin:0b0000_0011
    #[clap(value_parser)]
    gpio_mask: String,

    #[clap(subcommand, value_enum)]
    command: Commands,
}

#[derive(ValueEnum, Subcommand, Clone, Debug)]
pub enum Commands {
    Status,
    Pwm,
    High,
    Low,
    Read,
}

fn parse_gpio_dir(a: u8, bit: u8) -> &'static str {
    if a & (1 << bit) != 0 {
        return "Out";
    }
    "In"
}

fn parse_gpio_data(a: u8, bit: u8) -> &'static str {
    if a & (1 << bit) != 0 {
        return "High";
    }
    "Low"
}

pub fn cli_operator_gpio(args: &CmdGpio) {
    println!("Select device index: {}", args.index);
    println!("Select gpio mask: {}", args.gpio_mask);
    match args.command {
        Commands::Status => {
            let mut gpio_dir: u8 = 0;
            let mut gpio_data: u8 = 0;
            if ch347_rs::open_device(args.index) == ch347_rs::INVALID_HANDLE_VALUE {
                eprintln!("open device {} fail", args.index);
                return;
            }
            ch347_rs::gpio_get(args.index, &mut gpio_dir, &mut gpio_data);
            ch347_rs::close_device(args.index);

            println!("Dir: 0x{:02X} Data: 0x{:02X}", gpio_dir, gpio_data);
            for i in 0..7 {
                println!(
                    "GPIO{} {} {}",
                    i,
                    parse_gpio_dir(gpio_dir, i),
                    parse_gpio_data(gpio_data, i),
                );
            }
        }
        Commands::Pwm => {
            let mut gpio_dir: u8 = 0;
            let mut gpio_data: u8 = 0;

            if ch347_rs::open_device(args.index) == ch347_rs::INVALID_HANDLE_VALUE {
                println!("open device {} fail", args.index);
                return;
            }
            ch347_rs::gpio_get(args.index, &mut gpio_dir, &mut gpio_data);

            loop {
                ch347_rs::gpio_set(args.index, 0x80, 0x80, 0x80);
                ch347_rs::gpio_set(args.index, 0x80, 0x80, 0x00);
            }
        }
        Commands::High => {}
        Commands::Low => {}
        Commands::Read => {}
    }
}
