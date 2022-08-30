use crate::windows::basetsd::*;

pub const INVALID_HANDLE_VALUE: HANDLE = -1 as LONG as HANDLE;

// enum UsbType {
//     Ch347UsbCh341 = 0,
//     Ch347UsbHid = 2,
//     Ch347UsbVcp = 3,
// }

// enum Ch347FuncType{
//     Ch347FuncUart=0,
//     Ch347FuncSpiI2c=1,
//     Ch347FuncJtagI2c=2,
// }

/// 设备信息
#[repr(C)]
pub struct DeviceInfo {
    pub index: UCHAR,                  // 当前打开序号
    pub device_path: [CHAR; MAX_PATH], // 设备链接名,用于CreateFile

    /// 0:CH347_USB_CH341, 2:CH347_USB_HID,3:CH347_USB_VCP
    pub usb_class: UCHAR,

    /// - 0: CH347_FUNC_UART
    /// - 1: CH347_FUNC_SPI_I2C
    /// - 2: CH347_FUNC_JTAG_I2C
    pub func_type: UCHAR,

    /// USB\VID_xxxx&PID_xxxx
    pub device_id: [CHAR; 64],

    /// 芯片模式
    /// - 0: Mode0(UART0/1);
    /// - 1: Mode1(Uart1+SPI+I2C);
    /// - 2: Mode2(HID Uart1+SPI+I2C)
    /// - 3: Mode3(Uart1+Jtag+IIC)
    pub chip_mode: UCHAR,

    /// 设备句柄
    pub device_handle: HANDLE,
    /// 上传端点大小
    pub bulk_out_ep_max_size: USHORT,
    /// 下传端点大小
    pub bulk_in_ep_max_size: USHORT,
    /// USB速度类型，0:FS,1:HS,2:SS
    pub usb_speed_type: UCHAR,
    /// 设备接口号: 0:UART,1:SPI/IIC/JTAG/GPIO
    pub ch347_if_num: UCHAR,
    /// 端点地址
    pub data_up_ep: UCHAR,
    /// 端点地址
    pub data_down_ep: UCHAR,
    /// USB产品字符串
    pub rpoduct_string: [CHAR; 64],
    /// USB厂商字符串
    pub manufacturer_string: CHAR,
    /// USB写超时
    pub write_timeout: ULONG,
    /// USB读超时
    pub read_timeout: ULONG,
    /// 接口功能描述符
    pub func_desc_str: [CHAR; 64],
    /// 固件版本
    pub fw_ver: UCHAR,
}

impl DeviceInfo {
    fn default<'a>() -> *mut DeviceInfo {
        return &mut DeviceInfo {
            index: 0,
            device_path: [0; 256],
            usb_class: 0,
            func_type: 0,
            device_id: [0; 64],
            chip_mode: 0,
            device_handle: INVALID_HANDLE_VALUE,
            bulk_out_ep_max_size: 0,
            bulk_in_ep_max_size: 0,
            usb_speed_type: 0,
            ch347_if_num: 0,
            data_up_ep: 0,
            data_down_ep: 0,
            rpoduct_string: [0; 64],
            manufacturer_string: 0,
            write_timeout: 0,
            read_timeout: 0,
            func_desc_str: [0; 64],
            fw_ver: 0,
        };
    }
}

#[link(name = "CH347DLLA64")]
extern "C" {

    /// 该函数用于获得驱动版本、库版本、设备版本、芯片类型(CH341(FS)/CH347(HS))
    /// # Arguments
    ///
    /// * `iIndex` - 指定操作设备序号
    /// * `iDriverVer` - 驱动版本信息
    /// * `iDLLVer` - 库版本信息
    /// * `ibcdDevice` - 设备版本信息
    /// * `iChipType` - 芯片类型
    ///
    /// # Return
    ///
    /// 执行成功返回 1，失败返回 0
    ///
    pub fn CH347GetVersion(
        iIndex: libc::c_ulong,
        iDriverVer: *mut libc::c_uchar, // 驱动版本
        iDLLVer: *mut libc::c_uchar,    // 库版本
        ibcdDevice: *mut libc::c_uchar, // 设备版本
        iChipType: *mut libc::c_uchar,  // 芯片类型(CH341(FS)/CH347HS)
    ) -> BOOL;

    /// 该函数用于打开 CH347 设备，支持 CH347 所有模式下的 SPI/I2C/JTAG 接口的打开
    ///
    /// # Arguments
    ///
    /// * `DevI` - 指定操作设备序号
    ///
    /// # Return
    ///
    /// 执行成功返回设备序号
    ///
    pub fn CH347OpenDevice(DevI: ULONG) -> HANDLE;

    /// 该函数用于关闭 CH347 设备，支持 CH347 所有模式下 SPI/I2C/JTAG 接口的关闭
    ///
    /// # Arguments
    ///
    /// * `DevI` - 指定操作设备序号
    ///
    /// # Return
    ///
    /// 执行成功返回 1，失败返回 0
    ///
    pub fn CH347CloseDevice(DevI: ULONG) -> BOOL;

    /// 该函数用于获取设备当前接口模式、VID/PID 等信息
    ///
    /// # Arguments
    ///
    /// * `iIndex` - 指定操作设备序号
    /// * `DevInformation` - 设备信息结构体
    ///
    /// # Return
    ///
    /// 执行成功返回 1，失败返回 0
    ///
    pub fn CH347GetDeviceInfor(iIndex: ULONG, DevInformation: *mut DeviceInfo) -> BOOL;

    /// 设定设备事件通知程序
    ///
    /// 该函数用于指定设备事件通知程序，可用于 CH347 所有模式下 SPI/I2C/JTAG 接口的动态插拔检测
    ///
    /// # Arguments
    ///
    /// * `iIndex` - 指定设备序号,0对应第一个设备
    /// * `iDeviceID` - 可选参数,指向字符串,指定被监控的设备的ID,字符串以\0终止
    /// * `iNotifyRoutine` - 指定设备事件回调程序,为NULL则取消事件通知,否则在检测到事件时调用该程序
    ///
    /// # Return
    ///
    /// 执行成功返回 1，失败返回 0
    ///
    /// # Example
    ///
    /// ```c
    /// // 启用 CH347 同步串行接口 USB 的插入和移除的监测:
    /// CH347SetDeviceNotify(DevIndex, USBDevID, UsbDevPnpNotify);
    /// // 关闭 CH347 同步串行接口 USB 的插入和移除的监测，在程序退出时一定要关闭。
    /// CH347SetDeviceNotify(DevIndex, USBDevID, NULL);
    ///
    /// // CH347 设备插拔检测通知程序
    /// VOID CALLBACK UsbDevPnpNotify (ULONG iEventStatus )
    /// {
    ///     if(iEventStatus==CH347_DEVICE_ARRIVAL) // 设备插入事件,已经插入
    ///         PostMessage(DebugHwnd,WM_CH347DevArrive,0,0);
    ///     else if(iEventStatus==CH347_DEVICE_REMOVE) // 设备拔出事件,已经拔出
    ///         PostMessage(DebugHwnd,WM_CH347DevRemove,0,0);
    ///     return;
    /// }
    /// ```
    /// 若需做到准确检测各模式下的 SPI/I2C/JTAG 接口插拔信息，可写下如下完整 USBID，在使用 CH347SetDeviceNotify 时将 iDeviceID 替换成相应的 USBID 宏即可。
    /// ```c
    /// //MODE1 SPI/I2C
    /// #define USBID_VEN_Mode1_SPI_I2C "VID_1A86&PID_55DB&MI_02\0"
    /// //MODE2 SPI/I2C
    /// #define USBID_HID_Mode2_SPI_I2C "VID_1A86&PID_55DC&MI_01\0"
    /// //MODE3 JTAG/I2C
    /// #define USBID_VEN_Mode3_JTAG_I2C "VID_1A86&PID_55DA&MI_02\0"
    /// ```
    ///
    /// # Comment
    ///
    /// iDeviceID 该参数为可变参数，若需实现 CH347 设备的插拔检测，可定义宏如下:。
    /// ```c
    /// #define CH347DevID "VID_1A86&PID_55D\0"
    /// ```
    /// 传参时 iDeviceID 替换为 CH347DevID 即可实现对 CH347 同步串行接口的动态插拔检测
    ///
    /// 若需准确检测各模式下接口的插拔动作，可写下完整的 USBID，以模式 1 中 SPI 接口为例，可定义下方宏：
    /// ```c
    /// #define USBID_VEN_SPI_I2C “VID_1A86&PID_55DB&MI_02\0”
    /// ```
    /// 传参时 iDeviceID 替换为 USBID_VEN_SPI_I2C 即可实现对 CH347 模式 1 的 SPI&I2C 接口的动态插拔检测
    ///
    pub fn CH347SetDeviceNotify(
        iIndex: ULONG,
        iDeviceID: PCHAR,
        iNotifyRoutine: extern "C" fn(iEventStatus: ULONG),
    ) -> BOOL;
}

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
pub fn enum_device() -> Vec<*mut DeviceInfo> {
    let mut device_info_list = Vec::new();

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
pub fn get_version(device_index: u64) -> (BOOL, u8, u8, u8, u8) {
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

pub fn get_device_info(device_index: u64) -> Option<*mut DeviceInfo> {
    let device_info = DeviceInfo::default();
    let ret: BOOL;
    unsafe {
        ret = CH347GetDeviceInfor(device_index as libc::c_ulong, device_info);
    }

    if ret == 0 {
        return None;
    }

    return Some(device_info);
}