use super::{Capacity, Chip, RegReadRet, Register, RegisterAccess, RegisterItem, Vendor};

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

pub const REGISTER_DEFINES: [Register; 5] = [
    Register {
        name: "status_1",
        addr: 0x05,
        reader: |spi_flash| -> Result<RegReadRet, &'static str> {
            let mut buf: [u8; 2] = [0x05, 0x00];

            if let Err(_) = spi_flash.drive.transfer(&mut buf) {
                return Err("transfer fail");
            }

            Ok(RegReadRet::One(buf[1]))
        },
        items: Some(&[
            RegisterItem {
                name: "busy",
                alias: &["WIP"],
                describe: "Erase/Write In Progress",
                offset: 0,
                width: 1,
                access: RegisterAccess::ReadOnly,
            },
            RegisterItem {
                name: "write_enable",
                alias: &["WE", "WEL"],
                describe: "Write Enable Latch",
                offset: 1,
                width: 1,
                access: RegisterAccess::ReadOnly,
            },
            RegisterItem {
                name: "block_protect",
                alias: &["BP"],
                describe: "Block Protect Bits",
                offset: 2,
                width: 4,
                access: RegisterAccess::ReadOnly,
            },
            RegisterItem {
                name: "tb_protect",
                alias: &["TB"],
                describe: "Top/Bottom Protect Bit",
                offset: 6,
                width: 1,
                access: RegisterAccess::ReadOnly,
            },
            RegisterItem {
                name: "sreg_protect",
                alias: &["SRP"],
                describe: "Status Register Protect",
                offset: 7,
                width: 1,
                access: RegisterAccess::ReadOnly,
            },
        ]),
    },
    Register {
        name: "status_2",
        addr: 0x35,
        reader: |spi_flash| -> Result<RegReadRet, &'static str> {
            let mut buf: [u8; 2] = [0x35, 0x00];

            if let Err(_) = spi_flash.drive.transfer(&mut buf) {
                return Err("transfer fail");
            }

            Ok(RegReadRet::One(buf[1]))
        },
        items: Some(&[
            RegisterItem {
                name: "sreg_protect_1",
                alias: &["SRL"],
                offset: 0,
                width: 1,
                access: RegisterAccess::ReadOnly,
                describe: "Status Register Protect 1",
            },
            RegisterItem {
                name: "quad_enable",
                alias: &["QE"],
                offset: 1,
                width: 1,
                access: RegisterAccess::ReadOnly,
                describe: "Quad Enable",
            },
            RegisterItem {
                name: "resv",
                alias: &["R"],
                offset: 2,
                width: 1,
                access: RegisterAccess::ReadOnly,
                describe: "Reserved",
            },
            RegisterItem {
                name: "lock",
                alias: &["LB"],
                offset: 3,
                width: 3,
                access: RegisterAccess::ReadOnly,
                describe: "Security Register Lock Bits",
            },
            RegisterItem {
                name: "lock",
                alias: &["CMP"],
                offset: 6,
                width: 1,
                access: RegisterAccess::ReadOnly,
                describe: "Complement Protect",
            },
            RegisterItem {
                name: "suspend",
                alias: &["SUS"],
                offset: 7,
                width: 1,
                access: RegisterAccess::ReadOnly,
                describe: "Suspend Status",
            },
        ]),
    },
    Register {
        name: "status_3",
        addr: 0x15,
        reader: |spi_flash| -> Result<RegReadRet, &'static str> {
            let mut buf: [u8; 2] = [0x15, 0x00];

            if let Err(_) = spi_flash.drive.transfer(&mut buf) {
                return Err("transfer fail");
            }

            Ok(RegReadRet::One(buf[1]))
        },
        items: Some(&[
            RegisterItem {
                name: "cur_addr_mode",
                alias: &["ADS"],
                offset: 0,
                width: 1,
                access: RegisterAccess::ReadOnly,
                describe: "Current Address Mode",
            },
            RegisterItem {
                name: "powerup_addr_mode",
                alias: &["ADP"],
                offset: 1,
                width: 1,
                access: RegisterAccess::ReadOnly,
                describe: "Power Up Address Mode",
            },
            RegisterItem {
                name: "resv",
                alias: &["R"],
                offset: 2,
                width: 2,
                access: RegisterAccess::ReadOnly,
                describe: "Reserved",
            },
            RegisterItem {
                name: "DRV",
                alias: &["DRV"],
                offset: 5,
                width: 2,
                access: RegisterAccess::ReadOnly,
                describe: "Output Driver Strength",
            },
            RegisterItem {
                name: "resv",
                alias: &["R"],
                offset: 7,
                width: 1,
                access: RegisterAccess::ReadOnly,
                describe: "Reserved",
            },
        ]),
    },
    Register {
        name: "unique_id",
        addr: 0x4B,
        reader: |spi_flash| -> Result<RegReadRet, &'static str> {
            let mut wbuf: [u8; 13] = [0; 13];
            wbuf[0] = 0x4B;

            if let Err(e) = spi_flash.drive.transfer(&mut wbuf) {
                return Err(e);
            }

            Ok(RegReadRet::Muti(wbuf[5..wbuf.len()].to_vec()))
        },
        items: None,
    },
    Register {
        name: "ext_addr",
        addr: 0xC8,
        reader: |spi_flash| -> Result<RegReadRet, &'static str> {
            let mut buf: [u8; 2] = [0xC8, 0x00];

            spi_flash.drive.transfer(&mut buf)?;
            Ok(RegReadRet::One(buf[1]))
        },
        items: None,
    },
];
