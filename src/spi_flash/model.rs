#[derive(Debug)]
pub struct Vendor {
    pub name: &'static str,
    pub id: u8,
    pub parser: JedecIdParser,
}

#[derive(Debug)]
pub struct Chip {
    pub name: &'static str,
    pub vendor: &'static Vendor,
    pub capacity: u32,
}

type JedecIdParser = fn(vendor: &'static Vendor, data: (u8, u8)) -> Option<Chip>;

fn parse_macronix_jedec_id(vendor: &'static Vendor, data: (u8, u8)) -> Option<Chip> {
    let memory_type = data.0;
    let capacity = data.1;
    // println!("{:02X} {:02X}", memory_type, capacity);

    match memory_type {
        0x20 => match capacity {
            0x19 => Some(Chip {
                name: "MX25L256",
                vendor,
                capacity: mb(32),
            }),
            _ => None,
        },
        _ => None,
    }
}

fn mb(a: u32) -> u32 {
    byte_unit::n_mib_bytes!(a as u128) as u32
}

fn parse_winbond_jedec_id(vendor: &'static Vendor, data: (u8, u8)) -> Option<Chip> {
    let memory_type = data.0;
    let capacity = data.1;

    match memory_type {
        0x40 => match capacity {
            0x14 => Some(Chip {
                name: "W25Q80",
                vendor,
                capacity: mb(1),
            }),
            0x15 => Some(Chip {
                name: "W25Q16",
                vendor,
                capacity: mb(2),
            }),
            0x16 => Some(Chip {
                name: "W25Q32",
                vendor,
                capacity: mb(4),
            }),
            _ => None,
        },
        0x60 => match capacity {
            _ => None,
        },
        _ => None,
    }
}

const JEDEC_ID_LIST: [Vendor; 2] = [
    Vendor {
        name: "Macronix (MX)",
        id: 0xC2,
        parser: parse_macronix_jedec_id,
    },
    Vendor {
        name: "Winbond (ex Nexcom) serial flashes",
        id: 0xEF,
        parser: parse_winbond_jedec_id,
    },
];

pub fn parse_jedec_id(buf: &[u8]) -> Option<Chip> {
    let vendor = JEDEC_ID_LIST.iter().find(|&i| i.id == buf[0])?;

    (vendor.parser)(vendor, (buf[1], buf[2]))
}
