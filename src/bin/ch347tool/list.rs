use ch347_rs::{self, FuncType, UsbClass};
use clap::{Parser, ValueEnum};
use serde::Serialize;
use serde_json;

#[derive(Parser, Debug)]
#[clap(about = "List all plugged in devices")]
pub struct CmdListDevice {
    #[clap(short, long, action)]
    #[clap(default_value_t = ListFormat::Tree, value_enum)]
    pub format: ListFormat,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ListFormat {
    Tree,
    Json,
}

#[derive(Serialize)]
struct DeviceInfo {
    index: u8,
    name: String,
    func_type: String,
    usb_id: String,
    usb_class: String,
}

impl DeviceInfo {
    fn from_base_info(i: ch347_rs::DeviceInfo) -> DeviceInfo {
        DeviceInfo {
            index: i.index,
            name: i.get_func_desc_str(),
            func_type: match i.get_func_type() {
                FuncType::Uart => String::from("UART"),
                FuncType::SpiI2c => String::from("SPI & I2C & GPIO"),
                FuncType::JtagI2c => String::from("JTAG & I2C"),
            },
            usb_id: i.get_device_id(),
            usb_class: match i.get_usb_class() {
                UsbClass::Ch341 => String::from("ch341"),
                UsbClass::Hid => String::from("hid"),
                UsbClass::Vcp => String::from("vcp"),
            },
        }
    }
}

impl Into<DeviceInfo> for ch347_rs::DeviceInfo {
    fn into(self) -> DeviceInfo {
        DeviceInfo::from_base_info(self)
    }
}

pub fn cli_list_device(args: &CmdListDevice) {
    let mut l: Vec<DeviceInfo> = Vec::new();

    for i in ch347_rs::enum_ch347_device() {
        if let Some(info) = i.get_raw_info() {
            l.push(info.into());
        }
    }

    match args.format {
        ListFormat::Tree => {
            println!("'Ch347 device list:");
        }
        ListFormat::Json => {
            let j = serde_json::to_string_pretty(&l);

            match j {
                Ok(j) => {
                    println!("{}", j);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}
