use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use clap::Parser;
use cli_table::{
    format::{Align, Justify},
    Cell, Style, Table, TableStruct,
};

use super::utils;

#[derive(Parser, Clone, Debug)]
#[clap(about = "Operate chip registers")]
pub struct CmdReg {
    /// register name, eg: QE, addr_mode, ADP,
    #[clap(value_parser)]
    register: Option<String>,

    /// write value, eg: true, false, 0b11, 0x02
    #[clap(value_parser)]
    value: Option<String>,
}

pub fn cli_main(flash_args: &super::CmdSpiFlash, args: &CmdReg) -> Result<(), Box<dyn Error>> {
    let (device, chip_info) = flash_args.init()?;

    let reg_defines = match chip_info.vendor.reg_defines {
        None => return Err("Not define Registers".into()),
        Some(a) => a,
    };

    if (args.register == None) && (args.value == None) {
        // show all registers
        show_all_registers(device, chip_info, reg_defines)?;
    } else if (args.register != None) && (args.value == None) {
        // Read the specified register

        let reg_name = args.register.as_deref().unwrap();

        let find_result = utils::find_reg_by_name(reg_name, reg_defines);

        let find_result = match find_result {
            None => return Err(format!("Not Found Reg: {:?}", reg_name).into()),
            Some(a) => a,
        };

        show_one_registers(device, find_result)?;
    } else {
        // write to the specified register

        let reg_name = args.register.as_deref().unwrap();
        let reg_write_value = args.value.as_deref().unwrap();

        let find_result = utils::find_reg_by_name(reg_name, reg_defines);

        let find_result = match find_result {
            None => return Err(format!("Not Found Reg: {:?}", reg_name).into()),
            Some(a) => a,
        };

        write_registers(device, find_result, reg_write_value)?;
    }

    Ok(())
}

fn show_register_item_table(r: &[ch347_rs::RegisterItem], v: ch347_rs::RegReadRet) -> TableStruct {
    let mut items_table = Vec::new();

    // name line
    let mut items_name_table = Vec::new();
    items_name_table.push("Name".cell().bold(true));
    for ri in r.iter().rev() {
        items_name_table.push(ri.name.cell());
    }
    items_table.push(items_name_table);

    // bit line
    let mut items_posion_table = Vec::new();
    items_posion_table.push("Bit".cell().bold(true));
    for ri in r.iter().rev() {
        items_posion_table.push(
            if ri.width == 1 {
                format!("{}", ri.offset)
            } else {
                format!("{}..{}", ri.offset + ri.width - 1, ri.offset)
            }
            .cell(),
        );
    }
    items_table.push(items_posion_table);

    let mut items_desc_table = Vec::new();
    items_desc_table.push("Desc".cell().bold(true));
    for ri in r.iter().rev() {
        items_desc_table.push(ri.describe.cell());
    }
    items_table.push(items_desc_table);

    // value line
    let mut items_val_table = Vec::new();
    items_val_table.push("Val".cell().bold(true));
    for ri in r.iter().rev() {
        items_val_table.push(
            match v {
                ch347_rs::RegReadRet::One(a) => {
                    if ri.width == 1 {
                        utils::display_bool_with_color(a & (1 << ri.offset) != 0)
                    } else {
                        let mut v: u8 = 0;

                        for i in (ri.offset..(ri.offset + ri.width)).rev() {
                            v <<= 1;
                            if a & (1 << i) != 0 {
                                v |= 1;
                            }
                        }

                        let width = ri.width as usize;
                        format!("{:0>width$b}'b{}", v, width)
                    }
                }
                ch347_rs::RegReadRet::Muti(_) => {
                    panic!();
                }
            }
            .cell(),
        );
    }
    items_table.push(items_val_table);

    let mut items_access_table = Vec::new();
    items_access_table.push("Access".cell().bold(true));
    for ri in r.iter().rev() {
        items_access_table.push(
            console::style(ri.access.to_string())
                .bg(match ri.access {
                    ch347_rs::RegisterAccess::ReadOnly => console::Color::Black,
                    ch347_rs::RegisterAccess::ReadWrite => console::Color::Blue,
                    ch347_rs::RegisterAccess::ReadWriteOTP => console::Color::Yellow,
                })
                .cell(),
        );
    }
    items_table.push(items_access_table);

    items_table.table()
}

fn show_all_registers(
    spi_flash: ch347_rs::SpiFlash<ch347_rs::Ch347Device>,
    _chip_info: ch347_rs::Chip,
    reg_defines: &[ch347_rs::Register],
) -> Result<(), Box<dyn Error>> {
    let mut table = Vec::new();

    for r in reg_defines {
        let v = match (r.reader)(&spi_flash) {
            Err(e) => panic!("{}", e),
            Ok(v) => v,
        };

        table.push(match &v {
            ch347_rs::RegReadRet::One(a) => {
                vec![
                    r.name.cell().align(Align::Center),
                    format!("0x{:02X?}", a).cell().align(Align::Center),
                    match r.items {
                        None => "".cell(),
                        Some(items) => show_register_item_table(items, v)
                            .display()?
                            .cell()
                            .justify(Justify::Left),
                    },
                ]
            }
            ch347_rs::RegReadRet::Muti(a) => {
                vec![
                    r.name.cell().align(Align::Center),
                    format!("{} Byte ->", a.len()).cell().align(Align::Center),
                    format!("{:02X?}", a).cell(),
                ]
            }
        });

        // table.push(vec![r.name.to_string(), "".to_string(), v_str]);
        // println!("{} {}", r.name, v_str);
    }

    let table = table.table().title(vec![
        "Name".cell().bold(true),
        "Value".cell().bold(true),
        "Item".cell().bold(true),
    ]);

    println!("{}", table.display()?);

    Ok(())
}

fn show_one_registers(
    spi_flash: ch347_rs::SpiFlash<ch347_rs::Ch347Device>,
    reg_result: utils::FindRegType,
) -> Result<(), Box<dyn Error>> {
    match reg_result {
        utils::FindRegType::Reg(r) => {
            let v = match (r.reader)(&spi_flash) {
                Err(e) => panic!("{}", e),
                Ok(v) => v,
            };

            println!("{} {:02X?}", r.name, v);

            let mut table = Vec::new();

            match v {
                ch347_rs::RegReadRet::One(a) => {
                    table.push(vec![
                        r.name.cell().align(Align::Center),
                        format!("0x{:02X?}", a).cell().align(Align::Center),
                        match r.items {
                            None => "".cell(),
                            Some(items) => show_register_item_table(items, v)
                                .display()?
                                .cell()
                                .justify(Justify::Left),
                        },
                    ]);
                }
                ch347_rs::RegReadRet::Muti(a) => {
                    table.push(vec![
                        r.name.cell().align(Align::Center),
                        format!("{} Byte ->", a.len()).cell().align(Align::Center),
                        format!("{:02X?}", a).cell(),
                    ]);
                }
            }

            let table = table.table().title(vec![
                "Name".cell().bold(true),
                "Value".cell().bold(true),
                "Item".cell().bold(true),
            ]);

            println!("{}", table.display()?);
        }
        utils::FindRegType::RegItem(r, i) => {
            let v = match (r.reader)(&spi_flash) {
                Err(e) => panic!("{}", e),
                Ok(v) => v,
            };

            let item = &r.items.unwrap()[i];
            println!(
                "{}({:?}) <= {}({}..{})",
                item.name,
                item.alias.join(","),
                r.name,
                item.offset + item.width,
                item.offset,
            );
            println!("Desc: {}", item.describe);
            println!("Access: {:?}({})", item.access, item.access);

            match v {
                ch347_rs::RegReadRet::One(a) => {
                    let mut v: u8 = 0;

                    if item.width == 1 {
                        let v = a & (1 << item.offset) != 0;

                        let v_str = if v {
                            console::style("True").green()
                        } else {
                            console::style("False").red()
                        };

                        println!("Value: {}", v_str);
                    } else {
                        for i in (item.offset..(item.offset + item.width)).rev() {
                            v <<= 1;
                            if a & (1 << i) != 0 {
                                v |= 1;
                            }
                        }

                        let width = item.width as usize;
                        println!("Value: {:0>width$b}'b{}", v, width)
                    }
                }
                ch347_rs::RegReadRet::Muti(a) => {
                    let l: Vec<String> = a.iter().map(|i| format!("0x{:02X}", i)).collect();
                    let l = l.join(", ");
                    println!("{}", l);
                }
            }
        }
    }

    Ok(())
}

fn write_registers(
    spi_flash: ch347_rs::SpiFlash<ch347_rs::Ch347Device>,
    reg_result: utils::FindRegType,
    input_str: &str,
) -> Result<(), Box<dyn Error>> {
    match reg_result {
        utils::FindRegType::Reg(r) => {
            let v = match (r.reader)(&spi_flash) {
                Err(e) => panic!("{}", e),
                Ok(v) => v,
            };

            println!("{} old: {}", r.name, v);

            let write_val: u8 = utils::parse_cli_arg_number(&input_str, false)?;

            let reg_writer = match r.writer {
                None => panic!("The Reg Not Support Write"),
                Some(a) => a,
            };

            println!("will to write: {}", utils::display_u8_hex(write_val));

            let v = match v {
                ch347_rs::RegReadRet::One(a) => a,
                ch347_rs::RegReadRet::Muti(a) => a[0],
            };

            if let Some(items) = r.items {
                for ri in items {
                    let old_ri_val: bool = v & (1 << ri.offset) != 0;
                    let new_ri_val: bool = write_val & (1 << ri.offset) != 0;

                    if old_ri_val == new_ri_val {
                        continue;
                    }

                    if ri.width == 1 {
                        println!(
                            "  Name: {} Val: {} => {} Access: {}",
                            ri.name,
                            utils::display_bool_with_color(old_ri_val),
                            utils::display_bool_with_color(new_ri_val),
                            ri.access,
                        );
                    } else {
                        let mut old_ri_val: u8 = 0;
                        let mut new_ri_val: u8 = 0;

                        for i in (ri.offset..(ri.offset + ri.width)).rev() {
                            old_ri_val <<= 1;
                            if v & (1 << i) != 0 {
                                old_ri_val |= 1;
                            }

                            new_ri_val <<= 1;
                            if write_val & (1 << i) != 0 {
                                new_ri_val |= 1;
                            }
                        }

                        if old_ri_val == new_ri_val {
                            continue;
                        }

                        let width = ri.width as usize;
                        println!(
                            "  Name: {} Val: 0b{:0>width$b} => 0b{:0>width$b} Access: {}",
                            ri.name, old_ri_val, new_ri_val, ri.access,
                        );
                    }
                }
            }

            reg_writer(&spi_flash, &vec![write_val])?;

            let v = (r.reader)(&spi_flash)?;
            println!("ReRead Chk: {}", v);
        }
        utils::FindRegType::RegItem(r, i) => {
            let ri = &r.items.unwrap()[i];
            let width = ri.width as usize;

            let reg_writer = match r.writer {
                None => return Err("The Reg Not Support Write".into()),
                Some(a) => a,
            };

            if let ch347_rs::RegisterAccess::ReadOnly = ri.access {
                return Err("This RegItem is Read-only, cannot be write".into());
            }

            let new_ri_val = utils::parse_cli_arg_number(input_str, width == 1)?;

            let v = match (r.reader)(&spi_flash)? {
                ch347_rs::RegReadRet::One(a) => a,
                ch347_rs::RegReadRet::Muti(a) => a[0],
            };

            let ri_bitmask: u8 = if width == 1 {
                1 << ri.offset
            } else {
                !(1 << ri.width) << ri.offset
            };

            let write_val = v & !(ri_bitmask);
            let write_val = write_val | ((new_ri_val << ri.offset) & ri_bitmask);

            let mut old_ri_val: u8 = 0;
            for i in (ri.offset..(ri.offset + ri.width)).rev() {
                old_ri_val <<= 1;
                if v & (1 << i) != 0 {
                    old_ri_val |= 1;
                }
            }

            if width == 1 {
                println!(
                    "{} Val: {} => {}",
                    ri.name,
                    utils::display_bool_with_color(old_ri_val != 0),
                    utils::display_bool_with_color(new_ri_val != 0),
                );
            } else {
                println!(
                    "{} Val: 0b{:0>width$b} => 0b{:0>width$b}",
                    ri.name, old_ri_val, new_ri_val
                );
            }

            println!("{} Val:", r.name);
            println!("  Old: {}", utils::display_u8_hex(v));
            println!("  New: {}", utils::display_u8_hex(write_val));

            if old_ri_val == new_ri_val {
                stdout().write_all(b"Need to rewrite ? (Y): ")?;
                stdout().flush()?;
                let mut s = String::new();
                stdin().read_line(&mut s)?;
                if !s.trim().to_lowercase().eq("y") {
                    return Err("Operation must be confirmed".into());
                }
            }

            if let ch347_rs::RegisterAccess::ReadWriteOTP = ri.access {
                // if let ch347_rs::RegisterAccess::ReadWrite = ri.access {
                stdout().write_all(b"OTP Reg must be confirmed(Y): ")?;
                stdout().flush()?;
                let mut s = String::new();
                stdin().read_line(&mut s)?;
                if !s.trim().to_lowercase().eq("y") {
                    return Err("Operation must be confirmed".into());
                }
            }

            reg_writer(&spi_flash, &vec![write_val])?;

            match (r.reader)(&spi_flash)? {
                ch347_rs::RegReadRet::One(a) => {
                    println!("  Chk: {}", utils::display_u8_hex(a));
                }
                ch347_rs::RegReadRet::Muti(a) => {
                    println!("  Chk: {:02X?}", a);
                }
            };
        }
    }

    Ok(())
}
