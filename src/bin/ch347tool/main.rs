mod gpio;
mod i2c;
mod list;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = false)]
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

// fn main() {
//     ch347lib::set_notify_callback(0, "VID_1A86&PID_55D\0", |status| {
//         println!("[notify_callback] {:?}", status);
//     });

//     let a = ch347lib::enum_device();
//     // println!("enum_device len:{:?} :", a.len());
//     for (k, v) in a.iter().enumerate() {
//         println!(
//             "#{} => {:?} {:?}\r\n\tdevice_id:{}\r\n\tdevice_path:{}\r\n\trpoduct_string:{}\r\n\tget_manufacturer_string:{}\r\n\tfunc_desc_str:{}",
//             k,
//             v.get_func_type(),
//             v.get_usb_class(),
//             v.get_device_id(),
//             v.get_device_path(),
//             v.get_rpoduct_string(),
//             v.get_manufacturer_string(),
//             v.get_func_desc_str(),
//         )
//         // println!("#{} => {} ", k, v)
//     }

//     let a = ch347lib::enum_uart_device();
//     // println!("enum_uart_device len:{:?} :", a.len());
//     for (k, v) in a.iter().enumerate() {
//         println!(
//             "#{} => {:?} {:?}\r\n\tdevice_id:{}\r\n\tdevice_path:{}\r\n\trpoduct_string:{}\r\n\tget_manufacturer_string:{}\r\n\tfunc_desc_str:{}",
//             k,
//             v.get_func_type(),
//             v.get_usb_class(),
//             v.get_device_id(),
//             v.get_device_path(),
//             v.get_rpoduct_string(),
//             v.get_manufacturer_string(),
//             v.get_func_desc_str(),
//         );
//         // println!("#{} => {} ", k, v)
//     }

//     // let mut stdout = stdout();
//     // stdout.write(b"Press Enter to continue...").unwrap();
//     // stdout.flush().unwrap();
//     // stdin().read(&mut [0]).unwrap();

//     // println!("Exit");
// }
