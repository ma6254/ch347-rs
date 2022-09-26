mod eon_silicon;
mod winbond;

use std::fmt;

use super::{Register, SpiDrive, SpiFlash, StatusRegister};

type JedecIdParser = fn(vendor: &'static Vendor, data: (u8, u8)) -> Option<Chip>;

type UidReader = fn(spi_flash: &SpiFlash<dyn SpiDrive>) -> Result<Vec<u8>, &'static str>;

type SregReader =
    fn(spi_flash: &SpiFlash<dyn SpiDrive>) -> Result<Box<dyn StatusRegister>, &'static str>;

// #[derive(Debug)]
pub struct Vendor {
    pub name: &'static str,
    pub id: u8,
    pub parser: JedecIdParser,
    pub uid_reader: Option<UidReader>,
    pub sreg_reader: Option<SregReader>,
}

#[derive(Debug)]
pub enum Capacity {
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
    pub name: &'static str,
    pub vendor: &'static Vendor,
    pub capacity: Capacity,
}

const JEDEC_ID_LIST: [Vendor; 3] = [
    Vendor {
        name: "Eon Silicon",
        id: 0x1C,
        parser: |vendor, data| {
            let memory_type = data.0;
            let capacity = data.1;
            // println!("{:02X} {:02X}", memory_type, capacity);

            match memory_type {
                0x30 => match capacity {
                    0x16 => Some(Chip {
                        name: "EN25Q32C",
                        vendor,
                        capacity: Capacity::C32,
                    }),
                    _ => None,
                },
                _ => None,
            }
        },
        uid_reader: Some(eon_silicon::uid_reader),
        sreg_reader: None,
    },
    Vendor {
        name: "Macronix (MX)",
        id: 0xC2,
        parser: |vendor, data| {
            let memory_type = data.0;
            let capacity = data.1;
            // println!("{:02X} {:02X}", memory_type, capacity);

            match memory_type {
                0x20 => match capacity {
                    0x19 => Some(Chip {
                        name: "MX25L256",
                        vendor,
                        capacity: Capacity::C256,
                    }),
                    _ => None,
                },
                _ => None,
            }
        },
        uid_reader: None,
        sreg_reader: None,
    },
    Vendor {
        name: "Winbond (ex Nexcom) serial flashes",
        id: 0xEF,
        parser: winbond::parse_jedec_id,
        uid_reader: Some(winbond::uid_reader),
        // uid_reader: None,
        sreg_reader: Some(winbond::sreg_reader),
        // sreg_reader: None,
    },
];

pub fn parse_jedec_id(buf: &[u8]) -> Option<Chip> {
    let vendor = JEDEC_ID_LIST.iter().find(|&i| i.id == buf[0])?;

    (vendor.parser)(vendor, (buf[1], buf[2]))
}
