use super::*;

pub struct SpiFlash<T: SpiDrive> {
    drive: T,
}

#[derive(Debug)]
pub enum DelectErr {
    UnknowManufacturerID(u8),
    Other(&'static str),
}

impl<T: SpiDrive> SpiFlash<T> {
    pub fn new(drive: T) -> SpiFlash<T> {
        SpiFlash { drive: drive }
    }

    pub fn delect(&self) -> Result<Chip, DelectErr> {
        let mut wbuf: [u8; 4] = [0x9F, 0x00, 0x00, 0x00];

        if let Err(e) = self.drive.transfer(&mut wbuf) {
            return Err(DelectErr::Other(e));
        }

        let jedec_id = &wbuf[1..4];
        // println!("JEDEC_ID: {:02X?} ", jedec_id);

        let manufacturer_id = jedec_id[0];

        let chip_info = match parse_jedec_id(jedec_id) {
            None => {
                return Err(DelectErr::UnknowManufacturerID(manufacturer_id));
            }
            Some(chip_info) => chip_info,
        };

        return Ok(chip_info);
    }

    pub fn read(&self, addr: u32, buf: &mut [u8]) {
        buf[0] = 0x03;
        buf[1] = (addr >> 16) as u8;
        buf[2] = (addr >> 8) as u8;
        buf[3] = (addr) as u8;

        if let Err(_) = self.drive.write_after_read(4, buf.len() as u32, buf) {
            return;
        }
    }
}
