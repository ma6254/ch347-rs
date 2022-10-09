use std::error::Error;

use super::{Capacity, Chip, RegReadRet, Register, RegisterAccess, RegisterItem, Vendor};

#[test]
pub fn test_parse_jedec_id() {
    for (id, name, cap) in vec![
        (&[0xEF, 0x40, 0x10], "W25Q05", Capacity::C05),
        (&[0xEF, 0x40, 0x11], "W25Q10", Capacity::C10),
        (&[0xEF, 0x40, 0x12], "W25Q20", Capacity::C20),
        (&[0xEF, 0x40, 0x13], "W25Q40", Capacity::C40),
        (&[0xEF, 0x40, 0x14], "W25Q80", Capacity::C80),
        (&[0xEF, 0x40, 0x15], "W25Q16", Capacity::C16),
        (&[0xEF, 0x40, 0x16], "W25Q32", Capacity::C32),
    ] {
        assert!(super::parse_jedec_id(id).is_some());

        let chip = super::parse_jedec_id(id).unwrap();

        assert_eq!(name, &chip.name);
        assert_eq!(cap, chip.capacity);
    }
}

pub fn parse_jedec_id(vendor: &'static Vendor, data: (u8, u8)) -> Option<Chip> {
    let memory_type = data.0;
    let capacity = data.1;

    let mut chip_name = String::new();

    chip_name.push_str("W25Q");

    let (cap_str, chip_capacity) = match capacity {
        0x10 => ("05", Capacity::C05),
        0x11 => ("10", Capacity::C10),
        0x12 => ("20", Capacity::C20),
        0x13 => ("40", Capacity::C40),
        0x14 => ("80", Capacity::C80),
        0x15 => ("16", Capacity::C16),
        0x16 => ("32", Capacity::C32),
        0x17 => ("64", Capacity::C64),
        0x18 => ("128", Capacity::C128),
        0x19 => ("256", Capacity::C256),
        _ => return None,
    };
    chip_name.push_str(cap_str);

    match memory_type {
        0x40 => {}
        0x60 => {}
        _ => return None,
    }

    Some(Chip {
        name: chip_name,
        vendor,
        capacity: chip_capacity,
    })
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
        writer: Some(|spi_flash, wbuf| -> Result<(), Box<dyn Error>> {
            let mut buf: [u8; 1] = [0x06];
            spi_flash.drive.transfer(&mut buf)?;

            let mut buf: [u8; 2] = [0x01, wbuf[0]];
            spi_flash.drive.transfer(&mut buf)?;

            loop {
                let mut buf: [u8; 2] = [0x05, 0x00];
                spi_flash.drive.transfer(&mut buf)?;

                if buf[1] & 0x01 != 0x01 {
                    break;
                }
            }

            let mut buf: [u8; 1] = [0x04];
            spi_flash.drive.transfer(&mut buf)?;

            Ok(())
        }),
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
                access: RegisterAccess::ReadWrite,
            },
            RegisterItem {
                name: "tb_protect",
                alias: &["TB"],
                describe: "Top/Bottom Protect Bit",
                offset: 6,
                width: 1,
                access: RegisterAccess::ReadWrite,
            },
            RegisterItem {
                name: "sreg_protect",
                alias: &["SRP"],
                describe: "Status Register Protect",
                offset: 7,
                width: 1,
                access: RegisterAccess::ReadWrite,
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
        writer: Some(|spi_flash, wbuf| -> Result<(), Box<dyn Error>> {
            let mut buf: [u8; 1] = [0x06];
            spi_flash.drive.transfer(&mut buf)?;

            let mut buf: [u8; 2] = [0x31, wbuf[0]];
            spi_flash.drive.transfer(&mut buf)?;

            loop {
                let mut buf: [u8; 2] = [0x05, 0x00];
                spi_flash.drive.transfer(&mut buf)?;

                if buf[1] & 0x01 != 0x01 {
                    break;
                }
            }

            let mut buf: [u8; 1] = [0x04];
            spi_flash.drive.transfer(&mut buf)?;

            Ok(())
        }),
        items: Some(&[
            RegisterItem {
                name: "sreg_protect_1",
                alias: &["SRL"],
                offset: 0,
                width: 1,
                access: RegisterAccess::ReadWrite,
                describe: "Status Register Protect 1",
            },
            RegisterItem {
                name: "quad_enable",
                alias: &["QE"],
                offset: 1,
                width: 1,
                access: RegisterAccess::ReadWrite,
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
                access: RegisterAccess::ReadWriteOTP,
                describe: "Security Register Lock Bits",
            },
            RegisterItem {
                name: "lock",
                alias: &["CMP"],
                offset: 6,
                width: 1,
                access: RegisterAccess::ReadWrite,
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
        writer: Some(|spi_flash, wbuf| -> Result<(), Box<dyn Error>> {
            let mut buf: [u8; 1] = [0x06];
            spi_flash.drive.transfer(&mut buf)?;

            let mut buf: [u8; 2] = [0x11, wbuf[0]];
            spi_flash.drive.transfer(&mut buf)?;

            loop {
                let mut buf: [u8; 2] = [0x05, 0x00];
                spi_flash.drive.transfer(&mut buf)?;

                if buf[1] & 0x01 != 0x01 {
                    break;
                }
            }

            let mut buf: [u8; 1] = [0x04];
            spi_flash.drive.transfer(&mut buf)?;

            Ok(())
        }),
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
                access: RegisterAccess::ReadWrite,
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
                access: RegisterAccess::ReadWrite,
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
        writer: None,
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
        writer: Some(|spi_flash, wbuf| -> Result<(), Box<dyn Error>> {
            let mut buf: [u8; 1] = [0x06];
            spi_flash.drive.transfer(&mut buf)?;

            let mut buf: [u8; 2] = [0xC5, wbuf[0]];
            spi_flash.drive.transfer(&mut buf)?;

            let mut buf: [u8; 1] = [0x04];
            spi_flash.drive.transfer(&mut buf)?;

            Ok(())
        }),
        items: None,
    },
];
