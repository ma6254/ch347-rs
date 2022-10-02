use super::*;
use std::ops::Range;

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

pub enum WriteEvent {
    Block(usize, usize),
    Finish(usize),
}

pub type WriteEventFn = fn(e: WriteEvent);

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

pub struct SpiFlash<T: SpiDrive + ?Sized> {
    pub drive: T,
}

#[derive(Debug)]
pub enum DetectErr {
    UnknowManufacturerID(u8),
    Other(&'static str),
}

impl<T: SpiDrive + 'static> SpiFlash<T> {
    pub fn new(drive: T) -> SpiFlash<T> {
        SpiFlash { drive: drive }
    }

    pub fn detect(&self) -> Result<Chip, DetectErr> {
        let mut wbuf: [u8; 4] = [SpiFlashCmd::JedecId.into(), 0x00, 0x00, 0x00];

        if let Err(e) = self.drive.transfer(&mut wbuf) {
            return Err(DetectErr::Other(e));
        }

        let jedec_id = &wbuf[1..4];
        // println!("JEDEC_ID: {:02X?} ", jedec_id);

        let manufacturer_id = jedec_id[0];

        let chip_info = match parse_jedec_id(jedec_id) {
            None => {
                return Err(DetectErr::UnknowManufacturerID(manufacturer_id));
            }
            Some(chip_info) => chip_info,
        };

        return Ok(chip_info);
    }

    pub fn read_uuid(&self, vendor: &Vendor) -> Result<Vec<u8>, &'static str> {
        return vendor.read_uid(self);
    }

    pub fn read_status_register(
        &self,
        _vendor: &Vendor,
    ) -> Result<Box<dyn StatusRegister>, &'static str> {
        return Err("Not supported");
    }

    pub fn detect_and_print(&self) -> Result<Chip, DetectErr> {
        let chip_info = self.detect()?;

        println!("ChipInfo:");
        println!("  Manufacturer: {}", chip_info.vendor.name);
        println!("          Name: {}", chip_info.name);
        println!("      Capacity: {}", chip_info.capacity);

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

    pub fn write(&self, addr: u32, buf: &[u8]) -> Result<(), &'static str> {
        self.write_with_callback(|_| true, addr, buf)
    }

    pub fn write_with_callback<F>(
        &self,
        mut cbk: F,
        addr: u32,
        buf: &[u8],
    ) -> Result<(), &'static str>
    where
        F: FnMut(WriteEvent) -> bool,
    {
        self.wait_not_busy()?;

        const BLOCK_SIZE: usize = 0x100;

        for i in (0..buf.len()).step_by(BLOCK_SIZE) {
            // write enable
            let mut wbuf: [u8; 1] = [SpiFlashCmd::WriteEnable.into()];
            self.drive.transfer(&mut wbuf)?;

            let addr_offset = addr + i as u32;

            if (buf.len() - i) >= BLOCK_SIZE {
                let mut wbuf: [u8; 4 + BLOCK_SIZE] = [0; 4 + BLOCK_SIZE];

                wbuf[0] = SpiFlashCmd::PageProgram.into();
                wbuf[1] = (addr_offset >> 16) as u8;
                wbuf[2] = (addr_offset >> 8) as u8;
                wbuf[3] = addr_offset as u8;
                wbuf[4..(4 + BLOCK_SIZE)].copy_from_slice(&buf[i..(i + BLOCK_SIZE)]);
                self.drive.transfer(&mut wbuf)?;
                self.wait_not_busy()?;

                if !cbk(WriteEvent::Block(i, BLOCK_SIZE)) {
                    return Ok(());
                }
            } else {
                let mut wbuf: Vec<u8> = Vec::new();
                wbuf.extend_from_slice(&[
                    SpiFlashCmd::PageProgram.into(),
                    (addr_offset >> 16) as u8,
                    (addr_offset >> 8) as u8,
                    addr_offset as u8,
                ]);
                wbuf.extend_from_slice(&buf[i..buf.len()]);
                self.drive.transfer(&mut wbuf)?;
                self.wait_not_busy()?;

                if cbk(WriteEvent::Block(i, buf.len() - i)) {
                    return Ok(());
                }
            }
        }

        cbk(WriteEvent::Finish(buf.len()));
        return Ok(());
    }
}

pub type RegReader = fn(spi_flash: &SpiFlash<dyn SpiDrive>) -> Result<RegReadRet, &'static str>;

pub enum RegLen {
    One,
    Muti(usize),
}

#[derive(Debug)]
pub enum RegReadRet {
    One(u8),
    Muti(Vec<u8>),
}

pub struct Register<'a> {
    // pub bits: Range<usize>,
    pub name: &'static str,
    pub addr: u8,
    pub items: Option<&'a [RegisterItem]>,
    pub reader: RegReader,
}

pub struct RegisterItem {
    pub name: &'static str,
    pub alias: &'static [&'static str],
    pub describe: &'static str,
    pub offset: u8,
    pub width: u8,
    pub access: RegisterAccess,
}

#[derive(Default)]
pub enum RegisterAccess {
    #[default]
    ReadOnly,
    ReadWriteVolatile,
    ReadWrite,
    ReadWriteOTP,
}

// impl Register {
//     pub fn new(bits: Range<usize>) -> Register {
//         Register { bits }
//     }

//     pub fn new_bit(bit: usize) -> Register {
//         Register { bits: bit..bit + 1 }
//     }

//     pub fn read(self, buf: &[u8]) -> u32 {
//         read_reg_bits(buf, self.bits)
//     }

//     pub fn bit_width(&self) -> usize {
//         self.bits.len()
//     }
// }

pub struct RegisterRead<'a> {
    buf: &'a [u8],
}

impl<'a> RegisterRead<'_> {
    pub fn new(buf: &'a [u8]) -> RegisterRead {
        RegisterRead { buf }
    }

    pub fn read_bit(&self, bit: usize) -> Result<bool, &'static str> {
        let buf_index = bit / 8;
        let bit_index = bit % 8;

        let ret = if self.buf[buf_index] & (1 << bit_index) != 0 {
            true
        } else {
            false
        };

        return Ok(ret);
    }

    pub fn read_bits(&self, bits: Range<usize>) -> Result<Vec<bool>, &'static str> {
        let mut ret = Vec::new();

        for i in bits {
            let buf_index = i / 8;
            let bit_index = i % 8;

            let b = if self.buf[buf_index] & (1 << bit_index) != 0 {
                true
            } else {
                false
            };

            ret.push(b);
        }

        Ok(ret)
    }

    pub fn read_bytes(&self, bits: Range<usize>) -> Result<Vec<u8>, &'static str> {
        let mut ret = Vec::new();
        let mut b: u8 = 0;

        let bit_width = if bits.start > bits.end {
            bits.start - bits.end
        } else {
            bits.end - bits.start
        };

        for (k, i) in bits.enumerate() {
            let buf_index = i / 8;
            let bit_index = i % 8;

            b <<= 1;
            if self.buf[buf_index] & (1 << bit_index) != 0 {
                b |= 1;
            }

            if (k != 0) && (k % 8 == 0) {
                ret.push(b);
                b = 0;
            }
        }

        if (bit_width % 8) != 0 {
            ret.push(b);
        }

        Ok(ret)
    }
}
