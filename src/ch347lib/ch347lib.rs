use super::ch347dll::*;
use crate::windows::basetsd::*;

/// 枚举设备列表
///
/// # Arguments
///
///     list of device_info
///
/// # Examples
///
/// ```rust
/// println!("enum_device: {:?}", ch347lib::enum_device());
/// ```
pub fn enum_device() -> Vec<DeviceInfo> {
    let mut device_info_list: Vec<DeviceInfo> = Vec::new();

    for i in 0..16 {
        unsafe {
            if CH347OpenDevice(i) == INVALID_HANDLE_VALUE {
                continue;
            }

            if let Some(info) = get_device_info(i as u64) {
                device_info_list.push(info);
            }

            CH347CloseDevice(i);
        }
    }

    return device_info_list;
}

pub fn enum_uart_device() -> Vec<DeviceInfo> {
    let mut device_info_list: Vec<DeviceInfo> = Vec::new();

    for i in 0..16 {
        unsafe {
            if CH347Uart_Open(i) == INVALID_HANDLE_VALUE {
                continue;
            }

            if let Some(info) = get_uart_device_info(i as u64) {
                device_info_list.push(info);
            }

            CH347Uart_Close(i);
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
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
/// use doc::Person;
/// let person = Person::new("name");
/// ```
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
