use super::{Capacity, Chip, RegReadRet, Register, RegisterAccess, RegisterItem, Vendor};

pub fn parse_jedec_id(vendor: &'static Vendor, data: (u8, u8)) -> Option<Chip> {
    let memory_type = data.0;
    let capacity = data.1;
    // println!("{:02X} {:02X}", memory_type, capacity);

    match memory_type {
        0x20 => match capacity {
            0x17 => Some(Chip {
                name: "MX25L64".to_string(),
                vendor,
                capacity: Capacity::C64,
            }),
            0x19 => Some(Chip {
                name: "MX25L256".to_string(),
                vendor,
                capacity: Capacity::C256,
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

            spi_flash.drive.transfer(&mut buf)?;

            Ok(RegReadRet::One(buf[1]))
        },
        writer: None,
        items: Some(&[
            RegisterItem {
                name: "busy",
                alias: &["WIP"],
                describe: "write in progress bit",
                offset: 0,
                width: 1,
                access: RegisterAccess::ReadOnly,
            },
            RegisterItem {
                name: "write_enable",
                alias: &["WE", "WEL"],
                describe: "write enable latch",
                offset: 1,
                width: 1,
                access: RegisterAccess::ReadOnly,
            },
            RegisterItem {
                name: "block_protect",
                alias: &["BP"],
                describe: "level of protected block",
                offset: 2,
                width: 4,
                access: RegisterAccess::ReadOnly,
            },
            RegisterItem {
                name: "quad_enable",
                alias: &["QE"],
                describe: "Quad Enable",
                offset: 6,
                width: 1,
                access: RegisterAccess::ReadOnly,
            },
            RegisterItem {
                name: "sreg_w_prot",
                alias: &["SRWD"],
                describe: "status register write protect",
                offset: 7,
                width: 1,
                access: RegisterAccess::ReadOnly,
            },
        ]),
    },
    Register {
        name: "config",
        addr: 0x15,
        reader: |spi_flash| -> Result<RegReadRet, &'static str> {
            let mut buf: [u8; 2] = [0x15, 0x00];

            spi_flash.drive.transfer(&mut buf)?;

            Ok(RegReadRet::One(buf[1]))
        },
        writer: None,
        items: Some(&[
            RegisterItem {
                name: "ODS",
                alias: &["ODS"],
                describe: "output driver strength",
                offset: 0,
                width: 2,
                access: RegisterAccess::ReadOnly,
            },
            RegisterItem {
                name: "tb",
                alias: &["TB"],
                describe: "top/bottom selected",
                offset: 3,
                width: 1,
                access: RegisterAccess::ReadOnly,
            },
            RegisterItem {
                name: "tb_enable",
                alias: &["TBE"],
                describe: "Preamble bit Enable",
                offset: 4,
                width: 1,
                access: RegisterAccess::ReadOnly,
            },
            RegisterItem {
                name: "addr_mode",
                alias: &["EN4B"],
                describe: "",
                offset: 5,
                width: 1,
                access: RegisterAccess::ReadOnly,
            },
        ]),
    },
];
