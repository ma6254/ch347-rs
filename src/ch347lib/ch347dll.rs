use crate::windows::basetsd::*;
use std::ffi::CStr;
use std::fmt;

pub const INVALID_HANDLE_VALUE: HANDLE = -1 as LONG as HANDLE;

#[derive(Debug)]
pub enum NotifyiEventStatus {
    Inserted,
    Removed,
    Unknow(ULONG),
}

#[derive(Debug)]
pub enum UsbClass {
    Ch341,
    Hid,
    Vcp,
}

#[derive(Debug)]
pub enum UsbSpeedType {
    FS, // USB1.0 12Mbit/s
    HS, // USB2.0 480Mbit/s
    SS, // USB3.0 5GMbit/s
}

#[derive(Debug)]
pub enum FuncType {
    Uart,
    SpiI2c,
    JtagI2c,
}

#[derive(Debug)]
pub enum SpiClockLevel {
    S60M,
    S30M,
    S15M,
    S7_5M,
    S3_75M,
    S1_875M,
    S937_5K,
    S468_75K,
}

impl SpiClockLevel {
    pub fn from_byte(data: u8) -> Option<SpiClockLevel> {
        match data {
            0 => Some(SpiClockLevel::S60M),
            1 => Some(SpiClockLevel::S30M),
            2 => Some(SpiClockLevel::S15M),
            3 => Some(SpiClockLevel::S7_5M),
            4 => Some(SpiClockLevel::S3_75M),
            5 => Some(SpiClockLevel::S1_875M),
            6 => Some(SpiClockLevel::S937_5K),
            7 => Some(SpiClockLevel::S468_75K),
            _ => None,
        }
    }
}

impl fmt::Display for SpiClockLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SpiClockLevel::S60M => "60MHz",
                SpiClockLevel::S30M => "30MHz",
                SpiClockLevel::S15M => "15MHz",
                SpiClockLevel::S7_5M => "7.5MHz",
                SpiClockLevel::S3_75M => "3.75MHz",
                SpiClockLevel::S1_875M => "1.875MHz",
                SpiClockLevel::S937_5K => "937kHz",
                SpiClockLevel::S468_75K => "468.75kHz",
            }
        )
    }
}

/// 设备信息
#[repr(C)]
#[derive(Debug)]
pub struct DeviceInfo {
    pub index: UCHAR,                   // 当前打开序号
    pub device_path: [UCHAR; MAX_PATH], // 设备链接名,用于CreateFile

    /// 0:CH347_USB_CH341, 2:CH347_USB_HID,3:CH347_USB_VCP
    usb_class: UCHAR,

    /// - 0: CH347_FUNC_UART
    /// - 1: CH347_FUNC_SPI_I2C
    /// - 2: CH347_FUNC_JTAG_I2C
    func_type: UCHAR,

    /// USB\VID_xxxx&PID_xxxx
    device_id: [UCHAR; 64],

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
    usb_speed_type: UCHAR,
    /// 设备接口号: 0:UART,1:SPI/IIC/JTAG/GPIO
    pub ch347_if_num: UCHAR,
    /// 端点地址
    pub data_up_ep: UCHAR,
    /// 端点地址
    pub data_down_ep: UCHAR,
    /// USB产品字符串
    rpoduct_string: [UCHAR; 64],
    /// USB厂商字符串
    manufacturer_string: [UCHAR; 64],
    /// USB写超时
    pub write_timeout: ULONG,
    /// USB读超时
    pub read_timeout: ULONG,
    /// 接口功能描述符
    func_desc_str: [UCHAR; 64],
    /// 固件版本
    pub fw_ver: UCHAR,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct SpiConfig {
    /// 0-3:SPI Mode0/1/2/3
    pub mode: UCHAR,

    // 0=60MHz, 1=30MHz, 2=15MHz, 3=7.5MHz, 4=3.75MHz, 5=1.875MHz, 6=937.5KHz，7=468.75KHz
    pub clock: UCHAR,

    /// 0=低位在前(LSB), 1=高位在前(MSB)
    pub byte_order: UCHAR,

    /// SPI接口常规读取写入数据命令，单位为uS
    pub write_read_interval: USHORT,

    /// SPI读数据时默认输出数据
    pub out_default_data: UCHAR,

    /// 片选控制, 位7为0则忽略片选控制, 位7为1则参数有效: 位1位0为00/01分别选择CS1/CS2引脚作为低电平有效片选
    pub chip_select: ULONG,

    // 位0：片选CS1极性控制：0：低电平有效；1：高电平有效；
    pub cs1_polarity: UCHAR,

    /// 位0：片选CS2极性控制：0：低电平有效；1：高电平有效；
    pub cs2_polarity: UCHAR,

    /// 操作完成后是否自动撤消片选
    pub is_auto_deative_cs: USHORT,

    /// 设置片选后执行读写操作的延时时间,单位us
    pub active_delay: USHORT,

    /// 撤消片选后执行读写操作的延时时间,单位us
    pub delay_deactive: ULONG,
}

impl DeviceInfo {
    pub fn default() -> DeviceInfo {
        DeviceInfo {
            index: 0,
            device_path: [0; 260],
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
            manufacturer_string: [0; 64],
            write_timeout: 0,
            read_timeout: 0,
            func_desc_str: [0; 64],
            fw_ver: 0,
        }
    }

    pub fn get_device_path(&self) -> String {
        unsafe {
            let str = CStr::from_bytes_with_nul_unchecked(&self.device_path);
            return String::from(str.to_str().unwrap().trim_end_matches('\0'));
        }
    }

    pub fn get_usb_class(&self) -> UsbClass {
        match self.usb_class {
            0 => UsbClass::Ch341,
            2 => UsbClass::Hid,
            3 => UsbClass::Vcp,
            _ => panic!("Unknown usb class {}", self.usb_class),
        }
    }

    pub fn get_func_type(&self) -> FuncType {
        match self.func_type {
            0 => FuncType::Uart,
            1 => FuncType::SpiI2c,
            2 => FuncType::JtagI2c,
            _ => panic!("Unknown func type {}", self.usb_class),
        }
    }

    pub fn get_device_id(&self) -> String {
        unsafe {
            let str = CStr::from_bytes_with_nul_unchecked(&self.device_id);
            return String::from(str.to_str().unwrap().trim_end_matches('\0'));
        }
    }

    pub fn get_rpoduct_string(&self) -> String {
        unsafe {
            let str = CStr::from_bytes_with_nul_unchecked(&self.rpoduct_string);
            return String::from(str.to_str().unwrap().trim_end_matches('\0'));
        }
    }

    pub fn get_usb_speed_type(&self) -> Option<UsbSpeedType> {
        match self.usb_speed_type {
            0 => Some(UsbSpeedType::FS),
            1 => Some(UsbSpeedType::HS),
            2 => Some(UsbSpeedType::SS),
            _ => None,
        }
    }

    pub fn get_manufacturer_string(&self) -> String {
        unsafe {
            let str = CStr::from_bytes_with_nul_unchecked(&self.manufacturer_string);
            return String::from(str.to_str().unwrap().trim_end_matches('\0'));
        }
    }

    pub fn get_func_desc_str(&self) -> String {
        unsafe {
            let str = CStr::from_bytes_with_nul_unchecked(&self.func_desc_str);
            return String::from(str.to_str().unwrap().trim_end_matches('\0'));
        }
    }
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#{} usb:{:?}, func:{:?}, id:{}",
            self.index,
            self.get_usb_class(),
            self.get_func_type(),
            self.get_device_id()
        )
    }
}

// NOTE: The following are stubs for functions missing in the closed source
// .so library for Linux.

#[cfg(target_os = "linux")]
#[allow(non_snake_case)]
/// # Safety
/// This comes from a closed source C library. Rewrite it in Rust.
pub unsafe fn CH347Uart_GetDeviceInfor(_iIndex: ULONG, _DevInformation: *const DeviceInfo) -> BOOL {
    0
}

#[cfg(target_os = "linux")]
#[allow(non_snake_case)]
/// # Safety
/// This comes from a closed source C library. Rewrite it in Rust.
pub unsafe fn CH347Uart_Open(_DevI: ULONG) -> HANDLE {
    std::ptr::null_mut::<libc::c_void>()
}

#[cfg(target_os = "linux")]
#[allow(non_snake_case)]
/// # Safety
/// This comes from a closed source C library. Rewrite it in Rust.
pub unsafe fn CH347Uart_SetDeviceNotify() {}

#[cfg_attr(target_os = "linux", link(name = "ch347"))]
#[cfg_attr(target_os = "windows", link(name = "CH347DLLA64"))]
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
    pub fn CH347GetDeviceInfor(iIndex: ULONG, DevInformation: *const DeviceInfo) -> BOOL;

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
        iDeviceID: *const libc::c_uchar,
        iNotifyRoutine: *mut libc::c_void,
        // iNotifyRoutine: Ch347NotifyRoutine,
    ) -> BOOL;

    /// 该函数用于打开 CH347 串口
    ///
    /// ```c
    /// HANDLE WINAPI CH347Uart_Open(ULONG iIndex);
    /// ```
    #[cfg(target_os = "windows")]
    pub fn CH347Uart_Open(DevI: ULONG) -> HANDLE;

    /// 该函数用于关闭 CH347 串口
    ///
    /// ```c
    /// HANDLE WINAPI CH347Uart_Close(ULONG iIndex);
    /// ```
    pub fn CH347Uart_Close(DevI: ULONG) -> HANDLE;

    /// ```c
    /// BOOL WINAPI CH347Uart_GetDeviceInfor(ULONG iIndex,mDeviceInforS *DevInformation);
    /// ```
    #[cfg(target_os = "windows")]
    pub fn CH347Uart_GetDeviceInfor(iIndex: ULONG, DevInformation: *const DeviceInfo) -> BOOL;

    // pub fn CH347Uart_SetDeviceNotify

    /// 获取CH347的GPIO方向和引脚电平值
    ///
    /// ```c
    /// BOOL WINAPI CH347GPIO_Get(ULONG iIndex,
    ///     UCHAR *iDir,   //引脚方向:GPIO0-7对应位0-7.0：输入；1：输出
    ///     UCHAR *iData); //GPIO0电平:GPIO0-7对应位0-7,0：低电平；1：高电平)
    /// ```
    pub fn CH347GPIO_Get(iIndex: ULONG, iDir: PUCHAR, iData: PUCHAR) -> BOOL;

    /// 设置CH347的GPIO方向和引脚电平值
    ///
    /// ```c
    /// BOOL WINAPI CH347GPIO_Set(ULONG iIndex,
    ///     UCHAR iEnable,      //数据有效标志:对应位0-7,对应GPIO0-7.
    ///     UCHAR iSetDirOut,   //设置I/O方向,某位清0则对应引脚为输入,某位置1则对应引脚为输出.GPIO0-7对应位0-7.
    ///     UCHAR iSetDataOut); //输出数据,如果I/O方向为输出,那么某位清0时对应引脚输出低电平,某位置1时对应引脚输出高电平
    /// ```
    pub fn CH347GPIO_Set(
        iIndex: ULONG,
        iEnable: UCHAR,
        iSetDirOut: UCHAR,
        iSetDataOut: UCHAR,
    ) -> BOOL;

    /// ```c
    /// BOOL WINAPI CH347I2C_Set(
    ///     ULONG iIndex, // 指定设备序号
    ///     ULONG iMode );  // 指定模式,见下行
    /// // 位1-位0: I2C接口速度/SCL频率, 00=低速/20KHz,01=标准/100KHz(默认值),10=快速/400KHz,11=高速/750KHz
    /// // 其它保留,必须为0
    /// ```
    pub fn CH347I2C_Set(iIndex: ULONG, iMode: ULONG) -> BOOL;

    /// 处理I2C数据流,2线接口,时钟线为SCL引脚,数据线为SDA引脚
    ///
    /// ```c
    /// BOOL WINAPI CH347StreamI2C(
    ///     ULONG    iIndex,        // 指定设备序号
    ///     ULONG    iWriteLength,  // 准备写出的数据字节数
    ///     PVOID    iWriteBuffer,  // 指向一个缓冲区,放置准备写出的数据,首字节通常是I2C设备地址及读写方向位
    ///     ULONG    iReadLength,   // 准备读取的数据字节数
    ///     PVOID    oReadBuffer ); // 指向一个缓冲区,返回后是读入的数据
    /// ```
    pub fn CH347StreamI2C(
        iIndex: ULONG,
        iWriteLength: ULONG,
        iWriteBuffer: PVOID,
        iReadLength: ULONG,
        oReadBuffer: PVOID,
    ) -> BOOL;

    /// SPI控制器初始化
    ///
    /// ``` c
    /// BOOL WINAPI CH347SPI_Init(ULONG iIndex,mSpiCfgS *SpiCfg);
    /// ```
    pub fn CH347SPI_Init(iIndex: ULONG, mSpiCfgS: *mut SpiConfig) -> BOOL;

    /// 获取SPI控制器配置信息
    /// ```c
    /// BOOL WINAPI CH347SPI_GetCfg(ULONG iIndex,mSpiCfgS *SpiCfg);
    /// ```
    pub fn CH347SPI_GetCfg(iIndex: ULONG, mSpiCfgS: *mut SpiConfig) -> BOOL;

    pub fn CH347SPI_ChangeCS(iIndex: ULONG, iStatus: UCHAR);

    /// 该函数用于设置 SPI 片选
    pub fn CH347SPI_SetChipSelect(
        iIndex: ULONG,
        iEnableSelect: USHORT,
        iChipSelect: USHORT,
        iIsAutoDeativeCS: ULONG,
        iActiveDelay: ULONG,
        iDelayDeactive: ULONG,
    );

    /// 该函数用于 SPI 写数据
    pub fn CH347SPI_Write(
        iIndex: ULONG,
        iChipSelect: ULONG,
        iLength: ULONG,
        iWriteStep: ULONG,
        ioBuffer: PVOID,
    ) -> BOOL;

    /// 该函数用于读取 SPI 数据
    pub fn CH347SPI_Read(
        iIndex: ULONG,
        iChipSelect: ULONG,
        oLength: ULONG,
        iLength: PULONG,
        ioBuffer: PVOID,
    ) -> BOOL;

    /// 该函数用于写入和读取 SPI 数据流
    pub fn CH347SPI_WriteRead(
        iIndex: ULONG,
        iChipSelect: ULONG,
        iLength: ULONG,
        ioBuffer: PVOID,
    ) -> BOOL;

    /// 该函数用于处理 SPI 数据流，写入的同时读出数据
    pub fn CH347StreamSPI4(
        iIndex: ULONG,
        iChipSelect: ULONG,
        iLength: ULONG,
        ioBuffer: PVOID,
    ) -> BOOL;
}
