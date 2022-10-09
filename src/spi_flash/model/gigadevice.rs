use std::error::Error;

use super::{Capacity, Chip, RegReadRet, Register, RegisterAccess, RegisterItem, Vendor};

pub fn parse_jedec_id(vendor: &'static Vendor, data: (u8, u8)) -> Option<Chip> {
    let memory_type = data.0;
    let capacity = data.1;

    let mut chip_name = String::new();

    chip_name.push_str("GD25");

    match memory_type {
        0x40 => chip_name.push_str("Q"),
        0x60 => chip_name.push_str("LQ"),
        _ => return None,
    }

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
                width: 5,
                access: RegisterAccess::ReadWrite,
            },
            RegisterItem {
                name: "sreg_protect",
                alias: &["SRP0"],
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
                name: "cur_addr_mode",
                alias: &["ADS"],
                offset: 0,
                width: 1,
                access: RegisterAccess::ReadOnly,
                describe: "Current Address Mode",
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
                name: "suspend",
                alias: &["SUS2"],
                offset: 2,
                width: 1,
                access: RegisterAccess::ReadOnly,
                describe: "Program Suspend",
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
                name: "sreg_protect",
                alias: &["SRP1"],
                describe: "Status Register Protection",
                offset: 6,
                width: 1,
                access: RegisterAccess::ReadWrite,
            },
            RegisterItem {
                name: "suspend",
                alias: &["SUS1"],
                offset: 7,
                width: 1,
                access: RegisterAccess::ReadOnly,
                describe: "Erase Suspend",
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
                name: "dummy_cfg",
                alias: &["DC"],
                offset: 0,
                width: 2,
                access: RegisterAccess::ReadWrite,
                describe: "Dummy Configuration",
            },
            RegisterItem {
                name: "program_err",
                alias: &["PE"],
                offset: 2,
                width: 1,
                access: RegisterAccess::ReadOnly,
                describe: "Program Error",
            },
            RegisterItem {
                name: "erase_err",
                alias: &["EE"],
                offset: 3,
                width: 1,
                access: RegisterAccess::ReadOnly,
                describe: "Erase Error",
            },
            RegisterItem {
                name: "powerup_addr_mode",
                alias: &["ADP"],
                offset: 4,
                width: 1,
                access: RegisterAccess::ReadWrite,
                describe: "Power Up Address Mode",
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
                name: "hold_rst_func",
                alias: &["HOLD/RST"],
                offset: 7,
                width: 1,
                access: RegisterAccess::ReadWrite,
                describe: "HOLD# or RESET# Function",
            },
        ]),
    },
    Register {
        name: "unique_id",
        addr: 0x4B,
        reader: |spi_flash| -> Result<RegReadRet, &'static str> {
            let mut wbuf: [u8; 21] = [0; 21];
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
