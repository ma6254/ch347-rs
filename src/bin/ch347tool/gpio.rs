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
            if ch347_rs::open_device(args.index) == ch347_rs::INVALID_HANDLE_VALUE {
                eprintln!("open device {} fail", args.index);
                return;
            }
            let res = ch347_rs::gpio_get(args.index);
            ch347_rs::close_device(args.index);

            match res {
                Ok((gpio_dir, gpio_data)) => {
                    println!("Dir: 0x{:02X} Data: 0x{:02X}", gpio_dir, gpio_data);

                    for i in 0..7 {
                        println!(
                            "GPIO{} {} {}",
                            i,
                            parse_gpio_dir(gpio_dir, i),
                            parse_gpio_data(gpio_data, i),
                        );
                    }
                },
                Err(err) => {
                    eprintln!("GPIO status error {}", err)
                }
            }
        }
        Commands::Pwm => {
            if ch347_rs::open_device(args.index) == ch347_rs::INVALID_HANDLE_VALUE {
                println!("open device {} fail", args.index);
                return;
            }
            let _ = ch347_rs::gpio_get(args.index);

            loop {
                let _ = ch347_rs::gpio_set(args.index, 0x80, 0x80, 0x80);
                let _ = ch347_rs::gpio_set(args.index, 0x80, 0x80, 0x00);
            }
        }
        Commands::High => {
            if ch347_rs::open_device(args.index) == ch347_rs::INVALID_HANDLE_VALUE {
                println!("open device {} fail", args.index);
                return;
            }
            /*
             * Enable flag: corresponding to bit 0-7, corresponding to GPIO0-7.
             *
             * Set the I/O direction. If a certain bit is cleared to 0, the
             * corresponding pin is input, and if a certain position is set to
             * 1, the corresponding pin is output. GPIO0-7 corresponds to bits
             * 0-7.
             *
             * Output data, if the I/O direction is output, then the
             * corresponding pin outputs low level when a certain bit is cleared
             * to 0, and the corresponding pin outputs high level when a certain
             * position is 1.
             */
            let res = ch347_rs::gpio_set(args.index, 0x80, 0x80, 0x80);
            println!("gpio set {:?}", res);
            ch347_rs::close_device(args.index);
        }
        Commands::Low => {
            if ch347_rs::open_device(args.index) == ch347_rs::INVALID_HANDLE_VALUE {
                println!("open device {} fail", args.index);
                return;
            }
            let res = ch347_rs::gpio_set(args.index, 0x80, 0x80, 0x00);
            println!("gpio set {:?}", res);
            ch347_rs::close_device(args.index);
        }
        Commands::Read => {}
    }
}
