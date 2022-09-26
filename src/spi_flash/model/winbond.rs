use super::{Capacity, Chip, Register, SpiDrive, SpiFlash, StatusRegister, Vendor};
use std::fmt;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub enum SpiFlashCmd {
    // status
    ReadStatus2,
    ReadStatus3,
    // read
    ReadUniqueID,
}

impl Into<u8> for SpiFlashCmd {
    fn into(self) -> u8 {
        match self {
            // status
            SpiFlashCmd::ReadStatus2 => 0x35,
            SpiFlashCmd::ReadStatus3 => 0x15,
            // read
            SpiFlashCmd::ReadUniqueID => 0x4B,
        }
    }
}

pub fn parse_jedec_id(vendor: &'static Vendor, data: (u8, u8)) -> Option<Chip> {
    let memory_type = data.0;
    let capacity = data.1;

    match memory_type {
        0x40 => match capacity {
            0x14 => Some(Chip {
                name: "W25Q80",
                vendor,
                capacity: Capacity::C80,
            }),
            0x15 => Some(Chip {
                name: "W25Q16",
                vendor,
                capacity: Capacity::C16,
            }),
            0x16 => Some(Chip {
                name: "W25Q32",
                vendor,
                capacity: Capacity::C32,
            }),
            _ => None,
        },
        0x60 => match capacity {
            _ => None,
        },
        _ => None,
    }
}

pub fn uid_reader(spi_flash: &SpiFlash<dyn SpiDrive>) -> Result<Vec<u8>, &'static str> {
    let mut wbuf: [u8; 13] = [0; 13];
    wbuf[0] = SpiFlashCmd::ReadUniqueID.into();

    if let Err(e) = spi_flash.drive.transfer(&mut wbuf) {
        return Err(e);
    }

    return Ok(wbuf[5..wbuf.len()].to_vec());
}

pub fn sreg_reader(
    spi_flash: &SpiFlash<dyn SpiDrive>,
) -> Result<Box<dyn StatusRegister>, &'static str> {
    let ret = SREG::from_drive(spi_flash)?;

    return Ok(Box::new(ret));
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum SRegDef {
    Busy,
    WriteEnable,
    BlockProtect,
    QuadEnable,
}

impl fmt::Display for SRegDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl SRegDef {
    pub fn max_name_len() -> usize {
        let mut max_len = 0;

        for i in SRegDef::iter() {
            let l = format!("{:?}", i).len();
            if l > max_len {
                max_len = l;
            };
        }

        return max_len;
    }

    pub fn reg_def(&self) -> Register {
        match self {
            SRegDef::Busy => Register::new_bit(1),
            SRegDef::WriteEnable => Register::new_bit(2),
            SRegDef::BlockProtect => Register::new(3..7),
            SRegDef::QuadEnable => Register::new_bit(9),
        }
    }

    pub fn parse(self, buf: &[u8]) -> u32 {
        self.reg_def().read(buf)
    }
}

#[derive(Debug)]
pub struct SREG {
    pub raw_data: [u8; 3],
}

impl StatusRegister for SREG {
    fn from_drive(spi_flash: &SpiFlash<dyn SpiDrive>) -> Result<Self, &'static str> {
        let mut ret = SREG { raw_data: [0; 3] };

        let mut buf: [u8; 2] = [0x05, 0x00];
        spi_flash.drive.transfer(&mut buf)?;
        ret.raw_data[0] = buf[1];

        let mut buf: [u8; 2] = [SpiFlashCmd::ReadStatus2.into(), 0x00];
        spi_flash.drive.transfer(&mut buf)?;
        ret.raw_data[1] = buf[1];

        let mut buf: [u8; 2] = [SpiFlashCmd::ReadStatus3.into(), 0x00];
        spi_flash.drive.transfer(&mut buf)?;
        ret.raw_data[2] = buf[1];

        Ok(ret)
    }
}

impl SREG {
    pub fn parse_reg(&self, s: SRegDef) -> u32 {
        s.parse(&self.raw_data)
    }

    pub fn busy(&self) -> bool {
        self.raw_data[0] & 0b0000_0001 != 0
    }

    pub fn write_enable(&self) -> bool {
        self.raw_data[0] & 0b0000_0010 != 0
    }

    pub fn block_protect(&self) -> (bool, bool, bool, bool) {
        (
            self.raw_data[0] & 0b0000_0100 != 0,
            self.raw_data[0] & 0b0000_1000 != 0,
            self.raw_data[0] & 0b0001_0000 != 0,
            self.raw_data[0] & 0b0010_0000 != 0,
        )
    }

    pub fn top_bottom_protect(&self) -> bool {
        self.raw_data[0] & 0b0100_0000 != 0
    }

    pub fn status_reg_protect(&self) -> bool {
        self.raw_data[0] & 0b1000_0000 != 0
    }

    pub fn status_reg_protect1(&self) -> bool {
        self.raw_data[1] & 0b0000_0001 != 0
    }

    pub fn quad_enable(&self) -> bool {
        self.raw_data[1] & 0b0000_0010 != 0
    }

    pub fn suspend_status(&self) -> bool {
        self.raw_data[1] & 0b1000_0000 != 0
    }

    pub fn current_address_mode(&self) -> bool {
        self.raw_data[2] & 0b0000_0001 != 0
    }

    pub fn power_up_address_mode(&self) -> bool {
        self.raw_data[2] & 0b0000_0010 != 0
    }

    pub fn write_protect_selection(&self) -> bool {
        self.raw_data[2] & 0b0000_0100 != 0
    }
}

impl fmt::Display for SREG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StatusRegister: {:02X?}\r\n", self.raw_data)?;

        let name_max_len = SRegDef::max_name_len();

        for i in SRegDef::iter() {
            let reg = i.reg_def();
            let bit_width = reg.bit_width();
            write!(
                f,
                "{:>name_max_len$}: 0b{:0bit_width$b}'{}\r\n",
                i.to_string(),
                reg.read(&self.raw_data),
                bit_width,
            )?;
        }

        write!(f, "[ 0'1] Busy: {}\r\n", self.parse_reg(SRegDef::Busy))?;
        write!(
            f,
            "[ 1'1] WriteEnable: {:01b}\r\n",
            self.parse_reg(SRegDef::WriteEnable)
        )?;

        write!(f, "  [ 0'1] Busy: {}\r\n", self.busy())?;
        write!(f, "  [ 1'1] WriteEnable: {}\r\n", self.write_enable())?;
        write!(
            f,
            "  [ 2'4] BlockProtect(0~3): {:?}\r\n",
            self.block_protect()
        )?;
        write!(
            f,
            "  [ 6'1] Top/Bottom Protect: {}\r\n",
            self.top_bottom_protect()
        )?;
        write!(
            f,
            "  [ 7'1] SatusReg Protect: {}\r\n",
            self.status_reg_protect()
        )?;
        write!(
            f,
            "  [ 8'1] SatusReg Protect 1: {}\r\n",
            self.status_reg_protect1()
        )?;
        write!(f, "  [ 9'1] Quad Enable: {}\r\n", self.quad_enable())?;
        write!(f, "  [15'1] Suspend Status: {}\r\n", self.suspend_status())?;
        write!(
            f,
            "  [16'1] Current Address Mode: {}\r\n",
            self.current_address_mode()
        )?;
        write!(
            f,
            "  [17'1] Power Up Address Mode: {}\r\n",
            self.power_up_address_mode()
        )?;
        write!(
            f,
            "  [18'1] Write Protect Selection: {}\r\n",
            self.write_protect_selection()
        )?;

        write!(f, "{}", "")
    }
}
