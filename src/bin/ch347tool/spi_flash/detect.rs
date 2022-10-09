use std::error::Error;

use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[clap(about = "Detects spi flash chip model")]
pub struct CmdSpiFlashDetect {}

pub fn cli_spi_flash_detect(
    flash_args: &super::CmdSpiFlash,
    _args: &CmdSpiFlashDetect,
) -> Result<(), Box<dyn Error>> {
    let _ = flash_args.init()?;

    Ok(())
}
