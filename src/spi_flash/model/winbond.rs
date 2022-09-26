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
    TopBottomProtect,
    StatusRegProtect,
    StatusRegProtect1,
    QuadEnable,
    SecurityRegisterLock,
    ComplementProtect,
    SuspendStatus,
    CurrentAddressMode,
    PowerUpAddressMode,
    WriteProtectSelection,
    OutputDriverStrength,
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
            SRegDef::Busy => Register::new_bit(0),
            SRegDef::WriteEnable => Register::new_bit(1),
            SRegDef::BlockProtect => Register::new(2..6),
            SRegDef::TopBottomProtect => Register::new_bit(6),
            SRegDef::StatusRegProtect => Register::new_bit(7),
            SRegDef::StatusRegProtect1 => Register::new_bit(8),
            SRegDef::QuadEnable => Register::new_bit(9),
            SRegDef::SecurityRegisterLock => Register::new(11..14),
            SRegDef::ComplementProtect => Register::new_bit(14),
            SRegDef::SuspendStatus => Register::new_bit(15),
            SRegDef::CurrentAddressMode => Register::new_bit(16),
            SRegDef::PowerUpAddressMode => Register::new_bit(17),
            SRegDef::WriteProtectSelection => Register::new_bit(18),
            SRegDef::OutputDriverStrength => Register::new(21..23),
        }
    }

    // pub fn parse(self, buf: &[u8]) -> u32 {
    //     self.reg_def().read(buf)
    // }
}

impl fmt::Display for SRegDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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

impl SREG {}

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

        write!(f, "{}", "")
    }
}
