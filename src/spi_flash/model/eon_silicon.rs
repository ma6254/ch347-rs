use super::{Capacity, Chip, RegReadRet, Register, Vendor};

pub fn parse_jedec_id(vendor: &'static Vendor, data: (u8, u8)) -> Option<Chip> {
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
}

pub const REGISTER_DEFINES: [Register; 2] = [
    Register {
        name: "status",
        addr: 0x05,
        reader: |spi_flash| -> Result<RegReadRet, &'static str> {
            let mut buf: [u8; 2] = [0x05, 0x00];

            if let Err(_) = spi_flash.drive.transfer(&mut buf) {
                return Err("transfer fail");
            }

            Ok(RegReadRet::One(buf[1]))
        },
        items: None,
    },
    Register {
        name: "unique_id",
        addr: 0x5A,
        reader: |spi_flash| -> Result<RegReadRet, &'static str> {
            const UID_BITS: usize = 96;
            let mut wbuf: [u8; 5 + UID_BITS / 8] = [0; 5 + UID_BITS / 8];
            wbuf[0] = 0x5A;
            wbuf[1] = 0x00;
            wbuf[2] = 0x00;
            wbuf[3] = 0x80;

            if let Err(e) = spi_flash.drive.transfer(&mut wbuf) {
                return Err(e);
            }

            return Ok(RegReadRet::Muti(wbuf[5..wbuf.len()].to_vec()));
        },
        items: None,
    },
];
