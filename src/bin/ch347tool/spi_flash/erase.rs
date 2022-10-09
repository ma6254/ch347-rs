use std::{
    error::Error,
    time::{Duration, SystemTime},
};

use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[clap(about = "Erase spi flash chip")]
pub struct CmdSpiFlashErase {}

pub fn cli_spi_flash_erase(
    flash_args: &super::CmdSpiFlash,
    _args: &CmdSpiFlashErase,
) -> Result<(), Box<dyn Error>> {
    let (device, _) = flash_args.init()?;

    println!("Start Erase Full Chip ...");
    let start_time = SystemTime::now();

    device.erase_full()?;

    let take_time = start_time.elapsed().unwrap().as_millis();
    let take_time = Duration::from_millis(take_time as u64);
    println!("Done, Take time: {}", humantime::format_duration(take_time));

    Ok(())
}
