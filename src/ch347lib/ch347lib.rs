use std::error::Error;
use std::{fmt, string};

use super::ch347dll::*;
use crate::spi_flash::{SpiDrive, SpiFlash};
use crate::windows::basetsd::*;
use clap::ValueEnum;

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

    device_info_list
}

#[cfg(target_os = "windows")]
pub fn enum_uart_device() -> Vec<Ch347Device> {
    let mut device_info_list = Vec::new();

    for i in 0..16 {
        if let Some(dev) = Ch347Device::new_serial(i) {
            device_info_list.push(dev);
        }
    }

    device_info_list
}

#[cfg(target_os = "windows")]
pub fn open_device(index: u32) -> HANDLE {
    unsafe { CH347OpenDevice(index as ULONG) }
}
#[cfg(target_os = "linux")]
pub fn open_device(index: u32) -> i32 {
    unsafe { CH347OpenDevice(index as ULONG) }
}

pub fn close_device(index: u32) {
    unsafe {
        CH347CloseDevice(index as ULONG);
    }
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

    (ret, i_driver_ver, i_dllver, ibcd_device, i_chip_type)
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

    Some(device_info)
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

    Some(device_info)
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
            device_index as ULONG,
            // device_id.clone().as_mut_ptr(),
            device_id.as_ptr(),
            &mut cbk_fn as *mut _ as *mut libc::c_void,
        );
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum I2cSpeed {
    Low = 0x00,  //  20kHz
    Std = 0x01,  // 100kHz
    Fast = 0x02, // 400kHz
    High = 0x03, // 750kHz
}

impl fmt::Display for I2cSpeed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}-{}",
            self,
            match self {
                I2cSpeed::Low => "20kHz",
                I2cSpeed::Std => "100kHz",
                I2cSpeed::Fast => "400kHz",
                I2cSpeed::High => "750kHz",
            }
        )
    }
}

pub fn i2c_stream(
    index: ULONG,
    wsize: u32,
    wbuf: *const u8,
    rsize: u32,
    rbuf: *mut u8,
) -> Result<(), ()> {
    unsafe {
        if CH347StreamI2C(
            index as ULONG,
            wsize as ULONG,
            wbuf as *mut libc::c_void,
            rsize as ULONG,
            rbuf as *mut libc::c_void,
        ) == 0
        {
            return Err(());
        }
        Ok(())
    }
}

pub fn gpio_get(index: ULONG) -> Result<(u8, u8), string::String> {
    let mut dir: u8 = 0;
    let gpio_dir: *mut u8 = &mut dir;
    let mut data: u8 = 0;
    let gpio_data: *mut u8 = &mut data;

    unsafe {
        match CH347GPIO_Get(index as ULONG, gpio_dir, gpio_data) {
            0 => Err("fail".to_string()),
            _ => Ok((dir, data)),
        }
    }
}

/**
 * Enable flag: corresponding to bit 0-7, corresponding to GPIO0-7.
 *
 * Set the I/O direction. If a certain bit is cleared to 0, the
 * corresponding pin is input, and if a certain position is set to
 * 1, the corresponding pin is output. GPIO0-7 corresponds to bits
 * 0-7.
 *
 * Output data, if the I/O direction is output, then the
 * corresponding pin outputs low level when a certain bit is cleared
 * to 0, and the corresponding pin outputs high level when a certain
 * position is 1.
 */
pub fn gpio_set(
    index: ULONG,
    gpio_enable: u8,
    gpio_dir: u8,
    gpio_data: u8,
) -> Result<(), string::String> {
    unsafe {
        match CH347GPIO_Set(index as ULONG, gpio_enable, gpio_dir, gpio_data) {
            0 => Err("".to_string()),
            _ => Ok(()),
        }
    }
}

pub enum CH347TransType {
    Parallel,
    Serial,
}

pub struct Ch347Device {
    #[cfg(target_os = "windows")]
    index: ULONG,

    #[cfg(target_os = "linux")]
    fd: i32,

    ts_type: CH347TransType,
    spi_cfg: SpiConfig,
}

pub fn enum_ch347_device() -> Vec<Ch347Device> {
    let mut device_list: Vec<Ch347Device> = Vec::new();

    for i in enum_device() {
        device_list.push(i);
    }

    #[cfg(target_os = "windows")]
    {
        for i in enum_uart_device() {
            device_list.push(i);
        }
    }

    device_list
}

impl Ch347Device {
    #[cfg(target_os = "windows")]
    pub fn new(index: u32) -> Result<Ch347Device, &'static str> {
        unsafe {
            if CH347OpenDevice(index as ULONG) == INVALID_HANDLE_VALUE {
                return Err("CH347OpenDevice Fail");
            }
        }

        Ok(Ch347Device {
            index: index as ULONG,
            ts_type: CH347TransType::Parallel,
            spi_cfg: SpiConfig::default(),
        })
    }

    #[cfg(target_os = "linux")]
    pub fn new(index: u32) -> Result<Ch347Device, &'static str> {
        let fd = unsafe { CH347OpenDevice(index as ULONG) };
        if fd <= 0 {
            return Err("CH347OpenDevice Fail");
        }

        Ok(Ch347Device {
            fd,
            ts_type: CH347TransType::Parallel,
            spi_cfg: SpiConfig::default(),
        })
    }

    #[cfg(target_os = "windows")]
    pub fn new_serial(index: u32) -> Option<Ch347Device> {
        unsafe {
            if CH347Uart_Open(index as ULONG) == INVALID_HANDLE_VALUE {
                return None;
            }
        }

        Some(Ch347Device {
            index: index as ULONG,
            ts_type: CH347TransType::Serial,
            spi_cfg: SpiConfig::default(),
        })
    }

    #[cfg(target_os = "windows")]
    pub fn get_dev_index(&self) -> ULONG {
        self.index
    }

    #[cfg(target_os = "linux")]
    pub fn get_dev_index(&self) -> ULONG {
        self.fd as ULONG
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
                    if CH347GetDeviceInfor(
                        self.get_dev_index() as libc::c_ulong,
                        &device_info as *const _,
                    ) == 0
                    {
                        return None;
                    }
                }
                Some(device_info)
            }
            CH347TransType::Serial => {
                unsafe {
                    if CH347Uart_GetDeviceInfor(self.get_dev_index(), &device_info as *const _) == 0
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
            if CH347SPI_GetCfg(self.get_dev_index(), &mut spicfg) == 0 {
                return Err("CH347SPI_GetCfg Fail");
            }
        }

        Ok(spicfg)
    }

    pub fn apply_spi_config(&mut self) -> Result<(), &'static str> {
        unsafe {
            if CH347SPI_Init(self.get_dev_index(), &mut self.spi_cfg) == 0 {
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

    pub fn i2c_set(&self, speed: I2cSpeed) {
        unsafe {
            CH347I2C_Set(self.get_dev_index(), speed as ULONG);
        }
    }

    pub fn i2c_device_detect(&self, addr: u8) -> bool {
        unsafe {
            let mut wbuf: [u8; 1] = [addr << 1];
            if CH347StreamI2C(
                self.get_dev_index(),
                1,
                wbuf.as_mut_ptr() as *mut libc::c_void,
                0,
                std::ptr::null_mut::<libc::c_void>(),
            ) == 0
            {
                return false;
            }
        }
        true
    }

    pub fn i2c_stream(&self, wbuf: &[u8], rbuf: &mut [u8]) -> Result<(), ()> {
        i2c_stream(
            self.get_dev_index(),
            wbuf.len() as u32,
            wbuf.as_ptr(),
            rbuf.len() as u32,
            rbuf.as_mut_ptr(),
        )
    }
}

impl SpiDrive for Ch347Device {
    fn transfer(&self, iobuf: &mut [u8]) -> Result<(), &'static str> {
        unsafe {
            if CH347StreamSPI4(
                self.get_dev_index(),
                0x80,
                iobuf.len() as ULONG,
                iobuf.as_mut_ptr() as *mut libc::c_void,
            ) == 0
            {
                return Err("ch347 transfer failed");
            }
        }

        Ok(())
    }

    fn write_after_read(
        &self,
        write_len: u32,
        read_len: u32,
        iobuf: &mut [u8],
    ) -> Result<(), &'static str> {
        unsafe {
            if CH347SPI_Read(
                self.get_dev_index(),
                0x80,
                write_len as ULONG,
                &mut (read_len as ULONG),
                iobuf.as_mut_ptr() as *mut libc::c_void,
            ) == 0
            {
                return Err("ch347 transfer failed");
            }
        }

        Ok(())
    }
}

impl Drop for Ch347Device {
    fn drop(&mut self) {
        unsafe {
            CH347CloseDevice(self.get_dev_index());
        }
    }
}
