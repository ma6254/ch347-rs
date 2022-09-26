use super::{SpiDrive, SpiFlash};

pub const UID_BITS: usize = 96;

pub fn uid_reader(spi_flash: &SpiFlash<dyn SpiDrive>) -> Result<Vec<u8>, &'static str> {
    let mut wbuf: [u8; 5 + UID_BITS / 8] = [0; 5 + UID_BITS / 8];
    wbuf[0] = 0x5A;
    wbuf[1] = 0x00;
    wbuf[2] = 0x00;
    wbuf[3] = 0x80;

    if let Err(e) = spi_flash.drive.transfer(&mut wbuf) {
        return Err(e);
    }

    return Ok(wbuf[5..wbuf.len()].to_vec());
}
