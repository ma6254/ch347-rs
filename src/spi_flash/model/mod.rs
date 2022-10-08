mod eon_silicon;
mod macronix;
mod winbond;

use std::fmt;

use super::{RegReadRet, Register, RegisterAccess, RegisterItem, SpiDrive, SpiFlash};

type JedecIdParser = fn(vendor: &'static Vendor, data: (u8, u8)) -> Option<Chip>;

// #[derive(Debug)]
pub struct Vendor<'a> {
    pub name: &'static str,
    pub id: u8,
    pub parser: JedecIdParser,
    pub reg_defines: Option<&'a [Register]>,
}

impl<'a> Vendor<'_> {
    pub fn read_uid(&self, spi_flash: &SpiFlash<dyn SpiDrive>) -> Result<Vec<u8>, &'static str> {
        if let None = self.reg_defines {
            return Err("Not define Registers");
        }

        let result = self
            .reg_defines
            .unwrap()
            .iter()
            .find(|&item| item.name.eq("unique_id"));

        if let None = result {
            return Err("Not support Unique ID");
        }
        let result = result.unwrap();

        let result = (result.reader)(spi_flash)?;

        if let RegReadRet::Muti(buf) = result {
            return Ok(buf);
        }

        panic!();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Capacity {
    C05,
    C10,
    C20,
    C40,
    C80,
    C16,
    C32,
    C64,
    C128,
    C256,
}

impl Into<usize> for Capacity {
    fn into(self) -> usize {
        match self {
            Capacity::C05 => 1024 * 64,
            Capacity::C10 => 1024 * 128,
            Capacity::C20 => 1024 * 256,
            Capacity::C40 => 1024 * 512,
            Capacity::C80 => 1024 * 1024 * 1,
            Capacity::C16 => 1024 * 1024 * 2,
            Capacity::C32 => 1024 * 1024 * 4,
            Capacity::C64 => 1024 * 1024 * 8,
            Capacity::C128 => 1024 * 1024 * 16,
            Capacity::C256 => 1024 * 1024 * 32,
        }
    }
}

impl fmt::Display for Capacity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Capacity::C05 => "64 KB",
                Capacity::C10 => "128 KB",
                Capacity::C20 => "256 KB",
                Capacity::C40 => "512 KB",
                Capacity::C80 => "1 MB",
                Capacity::C16 => "2 MB",
                Capacity::C32 => "4 MB",
                Capacity::C64 => "8 MB",
                Capacity::C128 => "16 MB",
                Capacity::C256 => "32 MB",
            }
        )
    }
}

// #[derive(Debug)]
pub struct Chip {
    pub name: String,
    pub vendor: &'static Vendor<'static>,
    pub capacity: Capacity,
}

const JEDEC_ID_LIST: [Vendor; 3] = [
    Vendor {
        name: "Eon Silicon",
        id: 0x1C,
        parser: eon_silicon::parse_jedec_id,
        reg_defines: Some(&eon_silicon::REGISTER_DEFINES),
    },
    Vendor {
        name: "Macronix (MX)",
        id: 0xC2,
        parser: macronix::parse_jedec_id,
        reg_defines: Some(&macronix::REGISTER_DEFINES),
    },
    Vendor {
        name: "Winbond (ex Nexcom) serial flashes",
        id: 0xEF,
        parser: winbond::parse_jedec_id,
        reg_defines: Some(&winbond::REGISTER_DEFINES),
    },
];

#[test]
pub fn test_parse_jedec_id() {
    assert!(parse_jedec_id(&[0xFF, 0xFF, 0xFF]).is_none());
    assert!(parse_jedec_id(&[0x00, 0x00, 0x00]).is_none());
    assert!(parse_jedec_id(&[]).is_none());
}

pub fn parse_jedec_id(buf: &[u8]) -> Option<Chip> {
    if buf.len() < 3 {
        return None;
    }

    let vendor = JEDEC_ID_LIST.iter().find(|&i| i.id == buf[0])?;

    (vendor.parser)(vendor, (buf[1], buf[2]))
}
