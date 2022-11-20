use ch347_rs::I2cSpeed;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(about = "Detects all device address on the I2C bus")]
pub struct CmdI2cDetect {
    /// device number
    #[clap(value_parser)]
    index: u32,

    /// 20kHz, 100kHz, 400kHz, 750kHz
    #[clap(value_parser)]
    #[clap(default_value_t = I2cSpeed::Std, value_enum)]
    speed_level: I2cSpeed,
}

pub fn cli_i2c_detect(args: &CmdI2cDetect) {
    println!("speed: {}", args.speed_level);

    let dev = match ch347_rs::Ch347Device::new(args.index) {
        Ok(a) => a,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    dev.i2c_set(args.speed_level);

    println!("     0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F");

    for y in 0..8 {
        let mut s = format!("{:02X}:", y * 0x10);
        for x in 0..16 {
            // skip reserved addresses
            if ((y == 0x00) && (x <= 0x2)) || ((y == 0x07) && (x >= 0x08)) {
                s.push_str("   ");
                continue;
            }

            if dev.i2c_device_detect(y * 0x10 + x) {
                s.push_str(&format!(" {:02X}", y * 0x10 + x));
                continue;
            }

            s.push_str(" --");
        }
        println!("{}", s);
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum DumpPage {
    Byte,
    Word,
    Full,
}

#[derive(Parser, Debug)]
pub struct CmdI2cDump {
    /// device number
    #[clap(value_parser)]
    index: u32,

    /// i2c device addr(shifted), eg. 0x3C
    #[clap(value_parser)]
    addr: String,

    /// 20kHz, 100kHz, 400kHz, 750kHz
    #[clap(value_parser)]
    #[clap(default_value_t = I2cSpeed::Std, value_enum)]
    speed_level: I2cSpeed,

    #[clap(value_parser)]
    #[clap(default_value_t = DumpPage::Byte, value_enum)]
    page: DumpPage,
}

pub fn cli_i2c_dump(args: &CmdI2cDump) {
    println!("speed: {}", args.speed_level);

    let device_addr: u8;
    if args.addr.starts_with("0x") {
        match u8::from_str_radix(args.addr.trim_start_matches("0x"), 16) {
            Ok(addr) => device_addr = addr,
            Err(err) => {
                println!("parse device_addr error: {}", err);
                return;
            }
        }
    } else if args.addr.ends_with('H') {
        match u8::from_str_radix(args.addr.trim_end_matches('H'), 16) {
            Ok(addr) => device_addr = addr,
            Err(err) => {
                println!("parse device_addr error: {}", err);
                return;
            }
        }
    } else {
        match str::parse(&args.addr) {
            Ok(addr) => device_addr = addr,
            Err(err) => {
                println!("parse device_addr error: {}", err);
                return;
            }
        }
    }

    println!(
        "device_addr: 0x{:02X}(w:0x{:02X}, r:0x{:02X})",
        device_addr,
        device_addr << 1,
        (device_addr << 1) + 1
    );

    let dev = match ch347_rs::Ch347Device::new(args.index) {
        Ok(a) => a,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    dev.i2c_set(args.speed_level);

    match args.page {
        DumpPage::Byte => {
            println!("     0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F");
            for y in 0..16 {
                let mut s = format!("{:02X}:", y * 0x10);
                for x in 0..16 {
                    let mut wbuf: [u8; 2] = [
                        device_addr << 1, // device addr
                        y * 0x10 + x,     // register addr
                    ];

                    if !ch347_rs::i2c_stream(
                        dev.get_dev_index(),
                        2,
                        wbuf.as_mut_ptr(),
                        0,
                        std::ptr::null_mut::<u8>(),
                    )
                    {
                        s.push_str(" XX");
                        continue;
                    }

                    let mut wbuf: [u8; 1] = [(device_addr << 1) + 1];
                    let mut rbuf: [u8; 1] = [0];

                    if !ch347_rs::i2c_stream(
                        dev.get_dev_index(),
                        1,
                        wbuf.as_mut_ptr(),
                        1,
                        rbuf.as_mut_ptr(),
                    )
                    {
                        s.push_str(" XX");
                    }

                    s.push_str(&format!(" {:02X}", rbuf[0]));
                }
                println!("{}", s);
            }
        }

        DumpPage::Word => {
            println!("TODO: sorry");
        }
        DumpPage::Full => {
            let mut wbuf: [u8; 2] = [device_addr << 1, 0x00];

            if !ch347_rs::i2c_stream(
                dev.get_dev_index(),
                2,
                wbuf.as_mut_ptr(),
                0,
                std::ptr::null_mut::<u8>(),
            )
            {
                println!("i2c device addr nack");
                return;
            }

            let mut wbuf: [u8; 1] = [(device_addr << 1) + 1];
            let mut rbuf: [u8; 256] = [0; 256];
            ch347_rs::i2c_stream(
                dev.get_dev_index(),
                1,
                wbuf.as_mut_ptr(),
                256,
                rbuf.as_mut_ptr(),
            );

            println!("     0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F");
            for y in 0..16 {
                let mut s = format!("{:02X}:", y * 0x10);
                for x in 0..16 {
                    s.push_str(&format!(" {:02X}", rbuf[y * 0x10 + x]));
                }
                println!("{}", s);
            }
        }
    }
}
