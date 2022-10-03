use std::io::Result;

use clap::Parser;
use cli_table::{
    format::{Align, Justify},
    Cell, Style, Table, TableStruct,
};

use super::utils;

#[derive(Parser, Clone, Debug)]
#[clap(about = "Operate chip registers")]
pub struct CmdReg {
    /// register name, eg: EQ, addr_mode, ADP,
    #[clap(value_parser)]
    register: Option<String>,

    /// write value, eg: true, 0, 0b11, 0x02
    #[clap(value_parser)]
    value: Option<String>,
}

pub fn cli_main(flash_args: &super::CmdSpiFlash, args: &CmdReg) {
    unsafe {
        if ch347_rs::CH347OpenDevice(flash_args.index) == ch347_rs::INVALID_HANDLE_VALUE {
            println!("CH347OpenDevice Fail");
            return;
        }
    }

    let clock_level = match ch347_rs::SpiClockLevel::from_byte(flash_args.freq) {
        None => {
            println!("Unknow SPI clock level: {}", flash_args.freq);
            return;
        }
        Some(level) => level,
    };
    println!("Select SPI Clock: {}", clock_level);

    unsafe {
        let mut spicfg = ch347_rs::SpiConfig::default();
        if ch347_rs::CH347SPI_GetCfg(flash_args.index, &mut spicfg) == 0 {
            println!("CH347SPI_GetCfg Fail");
            return;
        }

        spicfg.clock = flash_args.freq;
        // spicfg.chip_select = 0x80;
        spicfg.byte_order = 1;
        if ch347_rs::CH347SPI_Init(flash_args.index, &mut spicfg) == 0 {
            println!("CH347SPI_Init Fail");
            return;
        }
        // println!("{:#?}", spicfg);

        let device = ch347_rs::Ch347Device::new(flash_args.index).spi_flash();
        let chip_info = match device.detect() {
            Err(e) => {
                println!("{:X?}", e);
                return;
            }
            Ok(chip_info) => chip_info,
        };

        println!("ChipInfo:");
        println!("  Manufacturer: {}", chip_info.vendor.name);
        println!("          Name: {}", chip_info.name);
        println!("      Capacity: {}", chip_info.capacity);

        let reg_defines = match chip_info.vendor.reg_defines {
            None => {
                println!("Not define Registers");
                return;
            }
            Some(a) => a,
        };

        if (args.register == None) && (args.value == None) {
            // show all registers
            if let Err(e) = show_all_registers(device, chip_info, reg_defines) {
                println!("{}", e);
                return;
            }
        } else if (args.register != None) && (args.value == None) {
            // Read the specified register

            let reg_name = args.register.as_deref().unwrap();

            let find_result = utils::find_reg_by_name(reg_name, reg_defines);

            let find_result = match find_result {
                None => {
                    println!("Not Found Reg: {:?}", reg_name);
                    return;
                }
                Some(a) => a,
            };

            if let Err(e) = show_one_registers(device, find_result) {
                println!("{}", e);
                return;
            }
        } else {
            // write to the specified register
        }
    }

    unsafe {
        ch347_rs::CH347CloseDevice(flash_args.index);
    }
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
                        let v = a & (1 << ri.offset) != 0;
                        format!(
                            "{}",
                            if v {
                                console::style("True").green()
                            } else {
                                console::style("False").red()
                            }
                        )
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
    items_access_table.push("Desc".cell().bold(true));
    for ri in r.iter().rev() {
        items_access_table.push(match ri.access {
            ch347_rs::RegisterAccess::ReadOnly => "Read only".cell(),
            _ => "NO".cell(),
        });
    }
    items_table.push(items_access_table);

    items_table.table()
}

fn show_all_registers(
    spi_flash: ch347_rs::SpiFlash<ch347_rs::Ch347Device>,
    _chip_info: ch347_rs::Chip,
    reg_defines: &[ch347_rs::Register],
) -> Result<()> {
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
) -> Result<()> {
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
