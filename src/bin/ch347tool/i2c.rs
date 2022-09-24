// use ch347_rs;
use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(ValueEnum, Clone, Debug)]
enum I2cSpeed {
    Low,  //  20kHz
    Std,  // 100kHz
    Fast, // 400kHz
    High, // 750kHz
}

impl fmt::Display for I2cSpeed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}-{}",
            self,
            match self {
                I2cSpeed::Low => "20kHz",
                I2cSpeed::Std => "100kHz",
                I2cSpeed::Fast => "400kHz",
                I2cSpeed::High => "750kHz",
            }
        )
    }
}

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

    unsafe {
        if ch347_rs::CH347OpenDevice(args.index) == ch347_rs::INVALID_HANDLE_VALUE {
            println!("open device {} fail", args.index);
            return;
        }
    }

    unsafe {
        ch347_rs::CH347I2C_Set(
            args.index,
            match args.speed_level {
                I2cSpeed::Low => 0x00,
                I2cSpeed::Std => 0x01,
                I2cSpeed::Fast => 0x02,
                I2cSpeed::High => 0x03,
            },
        );
    }

    println!("     0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F");

    for y in 0..8 {
        let mut s = String::from(format!("{:02X}:", y * 0x10));
        for x in 0..16 {
            // skip reserved addresses
            if ((y == 0x00) && (x <= 0x2)) || ((y == 0x07) && (x >= 0x08)) {
                s.push_str(&format!("   "));
                continue;
            }

            if ch347_rs::i2c_device_detect(args.index, y * 0x10 + x) {
                s.push_str(&format!(" {:02X}", y * 0x10 + x));
                continue;
            }

            s.push_str(&format!(" --"));
        }
        println!("{}", s);
    }

    unsafe {
        ch347_rs::CH347CloseDevice(args.index);
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
    } else if args.addr.ends_with("H") {
        match u8::from_str_radix(args.addr.trim_end_matches("H"), 16) {
            Ok(addr) => device_addr = addr,
            Err(err) => {
                println!("parse device_addr error: {}", err);
                return;
            }
        }
    } else {
        match u8::from_str_radix(&args.addr, 10) {
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

    unsafe {
        if ch347_rs::CH347OpenDevice(args.index) == ch347_rs::INVALID_HANDLE_VALUE {
            println!("open device {} fail", args.index);
            return;
        }
    }

    unsafe {
        ch347_rs::CH347I2C_Set(
            args.index,
            match args.speed_level {
                I2cSpeed::Low => 0x00,
                I2cSpeed::Std => 0x01,
                I2cSpeed::Fast => 0x02,
                I2cSpeed::High => 0x03,
            },
        );
    }

    match args.page {
        DumpPage::Byte => {
            println!("     0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F");
            for y in 0..16 {
                let mut s = String::from(format!("{:02X}:", y * 0x10));
                for x in 0..16 {
                    let mut wbuf: [u8; 2] = [device_addr << 1, y * 0x10 + x];

                    unsafe {
                        if ch347_rs::CH347StreamI2C(
                            args.index,
                            2,
                            wbuf.as_mut_ptr() as *mut libc::c_void,
                            0,
                            0 as *mut libc::c_void,
                        ) == 0
                        {
                            s.push_str(&format!(" XX"));
                            continue;
                        }

                        let mut wbuf: [u8; 1] = [(device_addr << 1) + 1];
                        let mut rbuf: [u8; 1] = [0];

                        if ch347_rs::CH347StreamI2C(
                            args.index,
                            1,
                            wbuf.as_mut_ptr() as *mut libc::c_void,
                            1,
                            rbuf.as_mut_ptr() as *mut libc::c_void,
                        ) == 0
                        {
                            s.push_str(&format!(" XX"));
                        }

                        s.push_str(&format!(" {:02X}", rbuf[0]));
                    }
                }
                println!("{}", s);
            }
        }

        DumpPage::Word => {
            println!("TODO: sorry");
        }
        DumpPage::Full => {
            let mut wbuf: [u8; 2] = [device_addr << 1, 0x00];

            unsafe {
                if ch347_rs::CH347StreamI2C(
                    args.index,
                    2,
                    wbuf.as_mut_ptr() as *mut libc::c_void,
                    0,
                    0 as *mut libc::c_void,
                ) == 0
                {
                    println!("i2c device addr nack");
                    return;
                }
            }

            let mut wbuf: [u8; 1] = [(device_addr << 1) + 1];
            let mut rbuf: [u8; 256] = [0; 256];
            unsafe {
                ch347_rs::CH347StreamI2C(
                    args.index,
                    1,
                    wbuf.as_mut_ptr() as *mut libc::c_void,
                    256,
                    rbuf.as_mut_ptr() as *mut libc::c_void,
                );
            }

            println!("     0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F");
            for y in 0..16 {
                let mut s = String::from(format!("{:02X}:", y * 0x10));
                for x in 0..16 {
                    s.push_str(&format!(" {:02X}", rbuf[y * 0x10 + x]));
                }
                println!("{}", s);
            }
        }
    }

    unsafe {
        ch347_rs::CH347CloseDevice(args.index);
    }
}
