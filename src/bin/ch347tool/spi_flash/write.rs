use std::{
    fmt::Write,
    fs,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime},
};

use clap::Parser;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use super::utils::{format_byte_per_sec, format_byte_unit};

#[derive(Parser, Clone, Debug)]
#[clap(about = "Write spi flash chip")]
pub struct CmdSpiFlashWrite {
    /// Before earse chip
    #[clap(short, long, value_parser, action)]
    erase: bool,

    /// Check after writing each page
    #[clap(short, long, value_parser, action)]
    check: bool,

    /// After check
    #[clap(long, value_parser, action)]
    after_check: bool,

    /// output to file
    #[clap(value_parser)]
    file: String,
}

pub fn cli_spi_flash_write(flash_args: &super::CmdSpiFlash, args: &CmdSpiFlashWrite) {
    let mut setp_count = 1;
    let mut setp_cnt = 0;
    if args.erase {
        setp_count += 1;
    }
    if args.after_check {
        setp_count += 1;
    }

    let mut file_buf = match fs::read(args.file.as_str()) {
        Err(e) => {
            println!("{:X?}", e);
            return;
        }
        Ok(f) => f,
    };

    if args.file.to_lowercase().ends_with(".cap") && (file_buf.len() > 0x800) {
        println!(
            "{} Detect {} file, will be offset {} address write",
            console::style("Note:").green(),
            console::style("ASUS-CAP").green(),
            console::style("0x800").green(),
        );

        file_buf = file_buf[0x800..file_buf.len()].to_vec();
    }

    unsafe {
        if ch347_rs::CH347OpenDevice(flash_args.index) == ch347_rs::INVALID_HANDLE_VALUE {
            return;
        }
    }

    let clock_level = match ch347_rs::SpiClockLevel::from_byte(flash_args.freq) {
        None => {
            println!("Unknow SPI clock level: {}", flash_args.freq);
            return;
        }
        Some(level) => level,
    };
    println!("Select SPI Clock: {}", clock_level);

    unsafe {
        let mut spicfg = ch347_rs::SpiConfig::default();
        if ch347_rs::CH347SPI_GetCfg(flash_args.index, &mut spicfg) == 0 {
            println!("CH347SPI_GetCfg Fail");
            return;
        }

        spicfg.clock = flash_args.freq;
        spicfg.write_read_interval = 0x100;
        // spicfg.chip_select = 0x80;
        spicfg.byte_order = 1;
        if ch347_rs::CH347SPI_Init(flash_args.index, &mut spicfg) == 0 {
            println!("CH347SPI_Init Fail");
            return;
        }
        // println!("{:#?}", spicfg);
    }

    let device = ch347_rs::Ch347Device::new(flash_args.index).spi_flash();
    let chip_info = match device.detect() {
        Err(e) => {
            println!("{:X?}", e);
            return;
        }
        Ok(chip_info) => chip_info,
    };

    println!("ChipInfo:");
    println!("  Manufacturer: {}", chip_info.vendor.name);
    println!("          Name: {}", chip_info.name);
    println!("      Capacity: {}", chip_info.capacity);

    let chip_capacity: usize = chip_info.capacity.into();

    let wsize: usize;
    if file_buf.len() <= chip_capacity {
        wsize = file_buf.len();
    } else {
        wsize = chip_capacity;
    }

    if file_buf.len() > chip_capacity {
        println!(
            "{} File size is too large, the last {} will be lost",
            console::style("Warn:").yellow(),
            console::style(format_byte_unit(file_buf.len() - chip_capacity)).yellow(),
        );
    }

    if args.erase {
        setp_cnt += 1;

        let spinner_style = ProgressStyle::with_template(
            "{prefix} {spinner:.green} [{elapsed_precise}] {wide_msg}",
        )
        .unwrap();
        // .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈x");

        let pb = ProgressBar::new(0);
        pb.set_style(spinner_style.clone());
        pb.set_prefix(format!(
            "{} Erasing",
            console::style(format!("[{}/{}]", setp_cnt, setp_count))
                .bold()
                .dim(),
        ));
        // pb.set_message(format!("Erasing ..."));

        let pb_finished = Arc::new(Mutex::new(pb));
        let mux_pb_finished = Arc::clone(&pb_finished);

        thread::spawn(move || loop {
            let pb_finished = mux_pb_finished.lock().unwrap();

            if (*pb_finished).is_finished() {
                break;
            }

            (*pb_finished).tick();
            thread::sleep(Duration::from_millis(40));
        });

        if let Err(e) = device.erase_full() {
            println!("{:X?}", e);
            return;
        }

        let pb_finished = pb_finished.lock().unwrap();
        (*pb_finished).finish_and_clear();

        let take_time = (*pb_finished).elapsed().as_millis();
        let take_time = Duration::from_millis(take_time as u64);

        println!(
            "{} Erase done, Take_time: {}",
            console::style(format!("[{}/{}]", setp_cnt, setp_count))
                .bold()
                .dim(),
            humantime::format_duration(take_time)
        );
    }

    let pb = ProgressBar::new(wsize as u64);
    pb.set_style(ProgressStyle::with_template(
        "{prefix} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({binary_bytes_per_sec}) ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    setp_cnt += 1;

    pb.set_prefix(format!(
        "{} {}",
        console::style(format!("[{}/{}]", setp_cnt, setp_count))
            .bold()
            .dim(),
        if args.check {
            "Writing with Verifing"
        } else {
            "Writing only"
        },
    ));

    let start_time = SystemTime::now();
    pb.tick();

    let a = |e| -> bool {
        match e {
            ch347_rs::WriteEvent::Block(addr, count) => {
                pb.inc(count as u64);

                if !args.check {
                    return true;
                }

                if (addr + 0x100) % 4096 != 0 {
                    return true;
                }

                const BLOCK_SIZE: usize = 4096;
                let block_addr = addr + 0x100 - BLOCK_SIZE;

                // println!(
                //     "check block {:02X}_{:04X}",
                //     block_addr >> 16,
                //     block_addr & 0xFFFF
                // );

                let mut rbuf: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
                device.read(block_addr as u32, &mut rbuf);

                let mut is_verify_pass = true;

                for x in 0..BLOCK_SIZE {
                    if rbuf[x] != file_buf[block_addr + x] {
                        pb.finish_and_clear();
                        println!(
                            "diff 0x{:04X}_{:04X} {:02X} => {:02X}",
                            (block_addr + x) >> 16,
                            (block_addr + x) & 0xFFFF,
                            file_buf[block_addr + x],
                            rbuf[x]
                        );
                        is_verify_pass = false;
                    }
                }

                return is_verify_pass;
            }
            ch347_rs::WriteEvent::Finish(_) => true,
        }
    };

    if let Err(e) = device.write_with_callback(a, 0, &file_buf[0..wsize]) {
        println!("{:X?}", e);
        return;
    };
    pb.finish_and_clear();
    let take_time = start_time.elapsed().unwrap().as_millis();
    let take_time = Duration::from_millis(take_time as u64);

    let speed = (wsize as f64) / take_time.as_secs_f64();
    let speed_str = format_byte_per_sec(speed);

    println!(
        "{} Write done, Take time: {} Speed: {}",
        console::style(format!("[{}/{}]", setp_cnt, setp_count))
            .bold()
            .dim(),
        humantime::format_duration(take_time),
        speed_str,
    );

    if args.after_check {
        setp_cnt += 1;

        println!(
            "{} Verify done, Take time: {}, Speed: {}",
            console::style(format!("[{}/{}]", setp_cnt, setp_count))
                .bold()
                .dim(),
            0,
            0,
        );
    }

    unsafe {
        ch347_rs::CH347CloseDevice(flash_args.index);
    }
}
