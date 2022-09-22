use super::*;

pub enum SpiFlashCmd {
    JedecId,
    WriteEnable,
    WriteDisable,
    // status
    ReadStatus,
    // erase
    ChipErase,
    Erase4K,
    Erase32K,
    Erase64K,
    // write
    PageProgram,
    // read
    ReadData,
}

impl Into<u8> for SpiFlashCmd {
    fn into(self) -> u8 {
        match self {
            SpiFlashCmd::JedecId => 0x9F,
            SpiFlashCmd::WriteEnable => 0x06,
            SpiFlashCmd::WriteDisable => 0x04,
            // status
            SpiFlashCmd::ReadStatus => 0x05,
            // erase
            SpiFlashCmd::ChipErase => 0xC7,
            SpiFlashCmd::Erase4K => 0x20,
            SpiFlashCmd::Erase32K => 0x52,
            SpiFlashCmd::Erase64K => 0xD8,
            // write
            SpiFlashCmd::PageProgram => 0x02,
            // read
            SpiFlashCmd::ReadData => 0x03,
        }
    }
}

#[derive(Debug)]
pub struct StatusRes {
    pub busy: bool,
    pub wtite_enable: bool,
}

impl From<u8> for StatusRes {
    fn from(data: u8) -> StatusRes {
        let s = StatusRes {
            busy: (data & 0x01) != 0,
            wtite_enable: (data & 0x02) != 0,
        };
        return s;
    }
}

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
        let mut wbuf: [u8; 4] = [SpiFlashCmd::JedecId.into(), 0x00, 0x00, 0x00];

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
        buf[0] = SpiFlashCmd::ReadData.into();
        buf[1] = (addr >> 16) as u8;
        buf[2] = (addr >> 8) as u8;
        buf[3] = (addr) as u8;

        if let Err(_) = self.drive.write_after_read(4, buf.len() as u32, buf) {
            return;
        }
    }

    pub fn read_status(&self) -> Result<StatusRes, &'static str> {
        let mut buf: [u8; 2] = [SpiFlashCmd::ReadStatus.into(), 0x00];

        if let Err(_) = self.drive.transfer(&mut buf) {
            return Err("transfer fail");
        }

        Ok(StatusRes::from(buf[1]))
    }

    pub fn wait_not_busy(&self) -> Result<StatusRes, &'static str> {
        loop {
            let status = match self.read_status() {
                Err(e) => {
                    println!("{:X?}", e);
                    return Err("read_status fail");
                }
                Ok(s) => s,
            };

            if status.busy {
                continue;
            }

            return Ok(status);
        }
    }

    pub fn erase_full(&self) -> Result<(), &'static str> {
        self.wait_not_busy()?;

        let mut buf: [u8; 1] = [SpiFlashCmd::WriteEnable.into()];
        self.drive.transfer(&mut buf)?;

        let mut buf: [u8; 1] = [SpiFlashCmd::ChipErase.into()];
        self.drive.transfer(&mut buf)?;

        self.wait_not_busy()?;

        return Ok(());
    }
}
