use std::{cmp, error::Error, fmt::Write, fs};

use clap::Parser;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

#[derive(Parser, Clone, Debug)]
#[clap(about = "Check spi flash chip memory")]
pub struct CmdSpiFlashCheck {
    /// output to file
    #[clap(value_parser)]
    file: String,
}

pub fn cli_spi_flash_check(
    flash_args: &super::CmdSpiFlash,
    args: &CmdSpiFlashCheck,
) -> Result<(), Box<dyn Error>> {
    let file_buf = fs::read(args.file.as_str())?;
    let (device, chip_info) = flash_args.init()?;

    let wsize = cmp::min(file_buf.len(), chip_info.capacity.into());

    // let mut all_buf: Vec<u8> = Vec::new();
    let pb = ProgressBar::new(wsize as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({binary_bytes_per_sec}) ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    println!("Checking...");
    const BLOCK_SIZE: usize = 4096;
    for i in (0..wsize).step_by(BLOCK_SIZE) {
        if (wsize - i) >= BLOCK_SIZE {
            let mut rbuf: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
            device.read(i as u32, &mut rbuf);
            // all_buf.extend_from_slice(&rbuf);
            for x in 0..BLOCK_SIZE {
                if rbuf[x] != file_buf[i + x] {
                    pb.finish_and_clear();
                    return Err(format!(
                        "diff 0x{:04X}_{:04X} {:02X} => {:02X}",
                        (i + x) >> 16,
                        (i + x) & 0xFFFF,
                        file_buf[i + x],
                        rbuf[x]
                    )
                    .into());
                }
            }
        } else {
            let mut rbuf: Vec<u8> = Vec::new();
            for _ in 0..(wsize - i) {
                rbuf.push(0x00);
            }
            device.read(i as u32, &mut rbuf);
            for x in 0..rbuf.len() {
                if rbuf[x] != file_buf[i + x] {
                    pb.finish_and_clear();
                    return Err(format!(
                        "diff 0x{:04X}_{:04X} {:02X} => {:02X}",
                        (i + x) >> 16,
                        (i + x) & 0xFFFF,
                        file_buf[i + x],
                        rbuf[x]
                    )
                    .into());
                }
            }
        }

        pb.set_position(i as u64);
        // pb.inc(BLOCK_SIZE as u64);
    }
    pb.finish();

    Ok(())
}
