use std::thread::sleep;
use std::time::Duration;
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
            let dev = ch347_rs::Ch347Device::new(args.index).expect("error opening device");

            let (gpio_dir, gpio_data) = ch347_rs::gpio_get(dev.get_dev_index()).expect("GPIO status error");
            println!("Dir: 0x{:02X} Data: 0x{:02X}", gpio_dir, gpio_data);

            for i in 0..=7 {
                println!(
                    "GPIO{} {} {}",
                    i,
                    parse_gpio_dir(gpio_dir, i),
                    parse_gpio_data(gpio_data, i),
                );
            }
        }
        Commands::Pwm => {
            let mask = args.gpio_mask.parse::<u8>().unwrap();
            let dev = ch347_rs::Ch347Device::new(args.index).expect("error opening device");
            let (curr_dir, curr_data) = ch347_rs::gpio_get(dev.get_dev_index()).unwrap();
            let dir = curr_dir | mask;

            // TODO: This is currently a fixed 3Hz blinky with 60% duty cycle.
            // Frequency and duty cycle would need to be configurable.
            let freq = 3.0;
            let duty = 0.6;
            let on_period = Duration::from_micros((duty * 1_000_000.0 / freq) as u64);
            let off_period = Duration::from_micros(((1.0 - duty) * 1_000_000.0 / freq) as u64);
            loop {
                let data = curr_data | mask;
                let _ = ch347_rs::gpio_set(dev.get_dev_index(), mask, dir, data);
                sleep(on_period);
                let data = curr_data & !mask;
                let _ = ch347_rs::gpio_set(dev.get_dev_index(), mask, dir, data);
                sleep(off_period);
            }
        }
        Commands::High => {
            let mask = args.gpio_mask.parse::<u8>().unwrap();
            let dev = ch347_rs::Ch347Device::new(args.index).expect("error opening device");
            let (curr_dir, curr_data) = ch347_rs::gpio_get(dev.get_dev_index()).unwrap();
            let dir = curr_dir | mask;
            let data = curr_data | mask;
            let res = ch347_rs::gpio_set(dev.get_dev_index(), mask, dir, data);
            println!("gpio set result {:?}", res);
            ch347_rs::close_device(args.index);
        }
        Commands::Low => {
            let mask = args.gpio_mask.parse::<u8>().unwrap();
            let dev = ch347_rs::Ch347Device::new(args.index).expect("error opening device");
            let (curr_dir, curr_data) = ch347_rs::gpio_get(dev.get_dev_index()).unwrap();
            let dir = curr_dir | mask;
            let data = curr_data & !mask;
            let res = ch347_rs::gpio_set(dev.get_dev_index(), mask, dir, data);
            println!("gpio set result {:?}", res);
            ch347_rs::close_device(args.index);
        }
        Commands::Read => {
            let dev = ch347_rs::Ch347Device::new(args.index).expect("error opening device");
            let res = ch347_rs::gpio_get(dev.get_dev_index());
            println!("gpio get {:?}", res);
        }
    }
}
