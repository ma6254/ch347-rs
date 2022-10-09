use std::error::Error;

use super::ch347dll::*;
use crate::spi_flash::{SpiDrive, SpiFlash};
use crate::windows::basetsd::*;

/// 枚举设备列表
///
/// # Arguments
///
/// list of device_info
///
/// # Examples
///
/// ```rust
/// println!("enum_device: {:?}", ch347_rs::enum_device());
/// ```
pub fn enum_device() -> Vec<Ch347Device> {
    let mut device_info_list = Vec::new();

    for i in 0..16 {
        if let Ok(dev) = Ch347Device::new(i) {
            device_info_list.push(dev);
        }
    }

    return device_info_list;
}

pub fn enum_uart_device() -> Vec<Ch347Device> {
    let mut device_info_list = Vec::new();

    for i in 0..16 {
        if let Some(dev) = Ch347Device::new_serial(i) {
            device_info_list.push(dev);
        }
    }

    return device_info_list;
}

/// Returns a person with the name given them
///
/// # Arguments
///
/// * `device_index` - A string slice that holds the name of the person
///
pub fn get_version(device_index: u32) -> (BOOL, u8, u8, u8, u8) {
    let mut i_driver_ver: u8 = 0;
    let mut i_dllver: u8 = 0;
    let mut ibcd_device: u8 = 0;
    let mut i_chip_type: u8 = 0;
    let ret: BOOL;

    unsafe {
        ret = CH347GetVersion(
            device_index as libc::c_ulong,
            &mut i_driver_ver,
            &mut i_dllver,
            &mut ibcd_device,
            &mut i_chip_type,
        );
    }

    return (ret, i_driver_ver, i_dllver, ibcd_device, i_chip_type);
}

pub fn get_device_info(device_index: u64) -> Option<DeviceInfo> {
    let device_info = DeviceInfo::default();
    let ret: BOOL;
    unsafe {
        ret = CH347GetDeviceInfor(device_index as libc::c_ulong, &device_info as *const _);
    }

    if ret == 0 {
        return None;
    }

    return Some(device_info);
}

pub fn get_uart_device_info(device_index: u64) -> Option<DeviceInfo> {
    let device_info = DeviceInfo::default();
    let ret: BOOL;
    unsafe {
        ret = CH347Uart_GetDeviceInfor(device_index as libc::c_ulong, &device_info as *const _);
    }

    if ret == 0 {
        return None;
    }

    return Some(device_info);
}

pub fn set_notify_callback(
    device_index: u32,
    device_id: &str,
    callback: fn(s: NotifyiEventStatus),
) {
    unsafe {
        let mut cbk_fn = |state: ULONG| {
            println!("Ch347NotifyRoutine");
            callback(match state {
                0 => NotifyiEventStatus::Inserted,
                3 => NotifyiEventStatus::Removed,
                _ => NotifyiEventStatus::Unknow(state),
            });
        };

        CH347SetDeviceNotify(
            device_index,
            // device_id.clone().as_mut_ptr(),
            device_id.as_ptr(),
            &mut cbk_fn as *mut _ as *mut libc::c_void,
        );
    }
}

pub fn i2c_device_detect(device_index: u32, i2c_dev_addr: u8) -> bool {
    unsafe {
        let mut wbuf: [u8; 1] = [i2c_dev_addr << 1];
        if CH347StreamI2C(
            device_index,
            1,
            wbuf.as_mut_ptr() as *mut libc::c_void,
            0,
            0 as *mut libc::c_void,
        ) == 0
        {
            return false;
        }
    }
    return true;
}

pub enum CH347TransType {
    Parallel,
    Serial,
}

pub struct Ch347Device {
    index: u32,
    ts_type: CH347TransType,
    spi_cfg: SpiConfig,
}

pub fn enum_ch347_device() -> Vec<Ch347Device> {
    let mut device_list: Vec<Ch347Device> = Vec::new();

    for i in enum_device() {
        device_list.push(i);
    }

    for i in enum_uart_device() {
        device_list.push(i);
    }

    return device_list;
}

impl Ch347Device {
    pub fn new(index: u32) -> Result<Ch347Device, &'static str> {
        unsafe {
            if CH347OpenDevice(index) == INVALID_HANDLE_VALUE {
                return Err("CH347OpenDevice Fail");
            }
        }

        Ok(Ch347Device {
            index,
            ts_type: CH347TransType::Parallel,
            spi_cfg: SpiConfig::default(),
        })
    }

    pub fn new_serial(index: u32) -> Option<Ch347Device> {
        unsafe {
            if CH347Uart_Open(index) == INVALID_HANDLE_VALUE {
                return None;
            }
        }

        Some(Ch347Device {
            index,
            ts_type: CH347TransType::Serial,
            spi_cfg: SpiConfig::default(),
        })
    }

    pub fn spi_flash(mut self) -> Result<SpiFlash<Ch347Device>, Box<dyn Error>> {
        self.spi_cfg = self.get_raw_spi_config()?;
        Ok(SpiFlash::new(self))
    }

    pub fn get_raw_info(&self) -> Option<DeviceInfo> {
        let device_info = DeviceInfo::default();

        match self.ts_type {
            CH347TransType::Parallel => {
                unsafe {
                    if CH347GetDeviceInfor(self.index as libc::c_ulong, &device_info as *const _)
                        == 0
                    {
                        return None;
                    }
                }
                Some(device_info)
            }
            CH347TransType::Serial => {
                unsafe {
                    if CH347Uart_GetDeviceInfor(
                        self.index as libc::c_ulong,
                        &device_info as *const _,
                    ) == 0
                    {
                        return None;
                    }
                }
                Some(device_info)
            }
        }
    }

    pub fn get_raw_spi_config(&self) -> Result<SpiConfig, &'static str> {
        let mut spicfg = SpiConfig::default();
        unsafe {
            if CH347SPI_GetCfg(self.index, &mut spicfg) == 0 {
                return Err("CH347SPI_GetCfg Fail");
            }
        }

        Ok(spicfg)
    }

    pub fn apply_spi_config(&mut self) -> Result<(), &'static str> {
        unsafe {
            if CH347SPI_Init(self.index, &mut self.spi_cfg) == 0 {
                return Err("CH347SPI_Init Fail");
            }
        }

        Ok(())
    }

    pub fn change_spi_raw_config<F>(&mut self, f: F) -> Result<(), &'static str>
    where
        F: Fn(&mut SpiConfig),
    {
        f(&mut self.spi_cfg);
        self.apply_spi_config()?;

        Ok(())
    }
}

impl SpiDrive for Ch347Device {
    fn transfer(&self, iobuf: &mut [u8]) -> Result<(), &'static str> {
        unsafe {
            if CH347StreamSPI4(
                self.index,
                0x00,
                iobuf.len() as ULONG,
                iobuf.as_mut_ptr() as *mut libc::c_void,
            ) == 0
            {
                return Err("ch347 transfer failed");
            }
        }

        return Ok(());
    }

    fn write_after_read(
        &self,
        write_len: u32,
        read_len: u32,
        iobuf: &mut [u8],
    ) -> Result<(), &'static str> {
        unsafe {
            let mut i_len = read_len as u32;
            if CH347SPI_Read(
                self.index,
                0x80,
                write_len as ULONG,
                &mut i_len,
                iobuf.as_mut_ptr() as *mut libc::c_void,
            ) == 0
            {
                return Err("ch347 transfer failed");
            }
        }

        return Ok(());
    }
}

impl Drop for Ch347Device {
    fn drop(&mut self) {
        unsafe {
            CH347CloseDevice(self.index);
        }
    }
}
