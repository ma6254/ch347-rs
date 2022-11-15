/*
 * ch34x_lib.h for ch341 in Epp/MEM/I2C/SPI/GPIO
 * Copyright (C) WCH 2019
 * Running Environment: Linux
 * Version:2.3
 */

#ifndef _CH34X_LIB_H
#define _CH34X_LIB_H

#ifndef CHAR
#define CHAR char
#endif

#ifndef UCHAR
#define UCHAR unsigned char
#endif

#ifndef USHORT
#define USHORT unsigned short
#endif

#ifndef ULONG
#define ULONG unsigned long
#endif

#ifndef LONGLONG
#define LONGLONG unsigned long long
#endif

#ifndef PUCHAR
#define PUCHAR unsigned char *
#endif

#ifndef PCHAR
#define PCHAR char *
#endif

#ifndef PUSHORT
#define PUSHORT unsigned short *
#endif

#ifndef PULONG
#define PULONG unsigned long *
#endif

#ifndef VOID
#define VOID void
#endif

#ifndef PVOID
#define PVOID void *
#endif

#define true 1
#define false 0

#define TRUE true
#define FALSE false

#ifndef min
#define min(x, y) (((x) < (y)) ? (x) : (y))
#endif

#ifndef max
#define max(x, y) (((x) < (y)) ? (y) : (x))
#endif

typedef enum
{
    FALSE_H = 0,
    TRUE_H = !FALSE_H
} BOOL;

#define MAX_PATH 512

#define CH341_PACKET_LENGTH 32
#define CH341_PKT_LEN_SHORT 8

#define CH341_MAX_NUMBER 16
#define MAX_BUFFER_LENGTH 0x1000
#define DEFAULT_BUFFER_LEN 0x0400

#define mCH341_PACKET_LENGTH 32
#define mCH341_MAX_NUMBER 16

#define CH347_USB_VENDOR 0
#define CH347_USB_HID 2
#define CH347_USB_VCP 3

// USB to SPI CMD
#define SET_CS 0
#define CLR_CS 1

#define USB20_CMD_SPI_INIT 0xC0    // 用于初始化SPI接口，设置SPI接口的数据位、时钟分频、高低字节顺序等等参数。
#define USB20_CMD_SPI_CONTROL 0xC1 // SPI接口控制命令,用于控制SPI接口片选引脚输出高低电平以及电平延时时间。
#define USB20_CMD_SPI_RD_WR 0xC2   // SPI接口常规读取写入数据命令,用于SPI接口通用读取写入操作，一般用于简短常规命令操作。该命令写N个字节数据的同时会回读N个字节数据。
#define USB20_CMD_SPI_BLCK_RD 0xC3 // SPI接口批量读取数据命令,用于SPI接口批量读取数据，一般用于批量数据的读取操作。启用该命令读取数据后，读取的数据会按最大包大小进行打包上传，直到所有数据读取返回完毕。
#define USB20_CMD_SPI_BLCK_WR 0xC4 // SPI接口批量写入数据命令,用于SPI接口批量写入数据，一般用于批量数据的写入操作。
#define USB20_CMD_INFO_RD 0xCA     // 参数获取,用于获取SPI接口相关参数等

#define mMAX_BUFFER_LENGTH 0x1000
#define USB20_CMD_HEADER 3

#define SPI_CS_ACTIVE 0x00
#define SPI_CS_DEACTIVE 0x01

/* SPI_data_direction */
#define SPI_Direction_2Lines_FullDuplex ((USHORT)0x0000)
#define SPI_Direction_2Lines_RxOnly ((USHORT)0x0400)
#define SPI_Direction_1Line_Rx ((USHORT)0x8000)
#define SPI_Direction_1Line_Tx ((USHORT)0xC000)

/* SPI_mode */
#define SPI_Mode_Master ((USHORT)0x0104)
#define SPI_Mode_Slave ((USHORT)0x0000)

/* SPI_data_size */
#define SPI_DataSize_16b ((USHORT)0x0800)
#define SPI_DataSize_8b ((USHORT)0x0000)

/* SPI_Clock_Polarity */
#define SPI_CPOL_Low ((USHORT)0x0000)
#define SPI_CPOL_High ((USHORT)0x0002)

/* SPI_Clock_Phase */
#define SPI_CPHA_1Edge ((USHORT)0x0000)
#define SPI_CPHA_2Edge ((USHORT)0x0001)

/* SPI_Slave_Select_management */
#define SPI_NSS_Soft ((USHORT)0x0200)
#define SPI_NSS_Hard ((USHORT)0x0000)

#define mWIN32_COMMAND_HEAD 32 // WIN32命令接口的头长度

/* SPI_MSB_LSB_transmission */
#define SPI_FirstBit_MSB ((USHORT)0x0000)
#define SPI_FirstBit_LSB ((USHORT)0x0080)

#define mMAX_BUFFER_LENGTH 0x1000 // 数据缓冲区最大长度4096

#define mMAX_COMMAND_LENGTH (mWIN32_COMMAND_HEAD + mMAX_BUFFER_LENGTH) // 最大数据长度加上命令结构头的长度

#define mDEFAULT_BUFFER_LEN 0x0400 // 数据缓冲区默认长度1024

#define mDEFAULT_COMMAND_LEN (mWIN32_COMMAND_HEAD + mDEFAULT_BUFFER_LEN) // 默认数据长度加上命令结构头的长度

/* SPI Init structure definition */
typedef struct _SPI_InitTypeDef
{
    USHORT SPI_Direction; /* Specifies th   e SPI unidirectional or bidirectional data mode.
                               This parameter can be a value of @ref SPI_data_direction */

    USHORT SPI_Mode; /* Specifies the SPI operating mode.
                          This parameter can be a value of @ref SPI_mode */

    USHORT SPI_DataSize; /* Specifies the SPI data size.
                              This parameter can be a value of @ref SPI_data_size */

    USHORT SPI_CPOL; /* Specifies the serial clock steady state.
                          This parameter can be a value of @ref SPI_Clock_Polarity */

    USHORT SPI_CPHA; /* Specifies the clock active edge for the bit capture.
                          This parameter can be a value of @ref SPI_Clock_Phase */

    USHORT SPI_NSS; /* Specifies whether the NSS signal is managed by
                         hardware (NSS pin) or by software using the SSI bit.
                         This parameter can be a value of @ref SPI_Slave_Select_management */

    USHORT SPI_BaudRatePrescaler; /* Specifies the Baud Rate prescaler value which will be
                                       used to configure the transmit and receive SCK clock.
                                       This parameter can be a value of @ref SPI_BaudRate_Prescaler.
                                       @note The communication clock is derived from the master
                                             clock. The slave clock does not need to be set. */

    USHORT SPI_FirstBit; /* Specifies whether data transfers start from MSB or LSB bit.
                              This parameter can be a value of @ref SPI_MSB_LSB_transmission */

    USHORT SPI_CRCPolynomial; /* Specifies the polynomial used for the CRC calculation. */
} SPI_InitTypeDef;

typedef struct _SpiUSBCFG
{
    SPI_InitTypeDef SPIInitCfg;
    USHORT SpiWriteReadInterval; // SPI接口常规读取写入数据命令(DEF_CMD_SPI_RD_WR))，单位为uS
    UCHAR SpiOutDefaultData;     // SPI读数据时默认输出数据
    UCHAR OtherCfg;              // 1个字节杂项控制；
                                 // 位7：片选CS1极性控制：0：低电平有效；1：有电平有效；
                                 // 位6：片选CS2极性控制：0：低电平有效；1：有电平有效；
                                 // 位5：IIC时钟延展功能控制：0：禁止；1：使能；
                                 // 位4：IIC读取最后1个字节完成时生成或不生成NACK；
                                 // 位3-0：保留；
    UCHAR Reserved[4];           // 保留
} SpiHwCfgS, *PSpiHwCfgS;

typedef struct _CH347_USB_CMD_S
{
    UCHAR mFunction;
    USHORT mLength;
    UCHAR mBuffer[512];
} CH347SPI_CMD, *mPCH347SPI_CMD;

typedef struct _StreamUSBCFG
{
    SPI_InitTypeDef SPIInitCfg;
    USHORT SpiWriteReadInterval; // SPI接口常规读取写入数据命令(DEF_CMD_SPI_RD_WR))，单位为uS
    UCHAR SpiOutDefaultData;     // SPI读数据时默认输出数据
    UCHAR OtherCfg;              // 1个字节杂项控制；
                                 // 位7：片选CS1极性控制：0：低电平有效；1：有电平有效；
                                 // 位6：片选CS2极性控制：0：低电平有效；1：有电平有效；
                                 // 位5：IIC时钟延展功能控制：0：禁止；1：使能；
                                 // 位4：IIC读取最后1个字节完成时生成或不生成NACK；
                                 // 位3-0：保留；
    UCHAR Reserved[4];           // 保留
} StreamHwCfgS, *PStreamHwCfgS;

#pragma pack(1)
// SPI控制器配置
typedef struct _SPI_CONFIG
{
    UCHAR iMode;                  // 0-3:SPI Mode0/1/2/3
    UCHAR iClock;                 // 0=60MHz, 1=30MHz, 2=15MHz, 3=7.5MHz, 4=3.75MHz, 5=1.875MHz, 6=937.5KHz，7=468.75KHz
    UCHAR iByteOrder;             // 0=低位在前(LSB), 1=高位在前(MSB)
    USHORT iSpiWriteReadInterval; // SPI接口常规读取写入数据命令，单位为uS
    UCHAR iSpiOutDefaultData;     // SPI读数据时默认输出数据
    ULONG iChipSelect;            // 片选控制, 位7为0则忽略片选控制, 位7为1则参数有效: 位1位0为00/01分别选择CS1/CS2引脚作为低电平有效片选
    UCHAR CS1Polarity;            // 位0：片选CS1极性控制：0：低电平有效；1：高电平有效；
    UCHAR CS2Polarity;            // 位0：片选CS2极性控制：0：低电平有效；1：高电平有效；
    USHORT iIsAutoDeativeCS;      // 操作完成后是否自动撤消片选
    USHORT iActiveDelay;          // 设置片选后执行读写操作的延时时间,单位us
    ULONG iDelayDeactive;         // 撤消片选后执行读写操作的延时时间,单位us
} mSpiCfgS, *mPSpiCfgS;

//设备信息
typedef struct _DEV_INFOR
{
    UCHAR iIndex;                // 当前打开序号
    UCHAR DevicePath[MAX_PATH];  // 设备链接名,用于CreateFile
    UCHAR UsbClass;              // 0:CH347_USB_CH341, 2:CH347_USB_HID,3:CH347_USB_VCP
    UCHAR FuncType;              // 0:CH347_FUNC_UART,1:CH347_FUNC_SPI_I2C,2:CH347_FUNC_JTAG_I2C
    CHAR DeviceID[64];           // USB\VID_xxxx&PID_xxxx
    UCHAR ChipMode;              // 芯片模式,0:Mode0(UART0/1); 1:Mode1(Uart1+SPI+I2C); 2:Mode2(HID Uart1+SPI+I2C) 3:Mode3(Uart1+Jtag+IIC)
    int DevHandle;               // 设备句柄
    USHORT BulkOutEndpMaxSize;   // 上传端点大小
    USHORT BulkInEndpMaxSize;    // 下传端点大小
    UCHAR UsbSpeedType;          // USB速度类型，0:FS,1:HS,2:SS
    UCHAR CH347IfNum;            // 设备接口号: 0:UART,1:SPI/IIC/JTAG/GPIO
    UCHAR DataUpEndp;            // 端点地址
    UCHAR DataDnEndp;            // 端点地址
    CHAR ProductString[64];      // USB产品字符串
    CHAR ManufacturerString[64]; // USB厂商字符串
    ULONG WriteTimeout;          // USB写超时
    ULONG ReadTimeout;           // USB读超时
    CHAR FuncDescStr[64];        // 接口功能描述符
    UCHAR FirewareVer;           // 固件版本
} mDeviceInforS, *mPDeviceInforS;

typedef struct _DevObj
{
    UCHAR iIndex; //当前打开序号
    UCHAR DevicePath[MAX_PATH];
    UCHAR UsbClass;              // 0:CH341 Vendor; 1:CH347 Vendor; 2:HID
    UCHAR FuncType;              // 0:UART1;        1:SPI+IIC;      2:JTAG+IIC
    CHAR DeviceID[64];           // USB\VID_xxxx&PID_xxxx
    UCHAR Mode;                  //芯片模式,0:Mode0(UART*2); 1:Mode1(Uart1+SPI+IIC); 2:Mode2(HID Uart1+SPI+IIC) 3:Mode3(Uart1+Jtag+IIC)
    USHORT BulkOutEndpMaxSize;   //上传端点大小
    USHORT BulkInEndpMaxSize;    //下传端点大小
    UCHAR UsbSpeedType;          // USB速度类型，0:FS,1:HS,2:SS
    UCHAR CH347IfNum;            // USB接口号
    UCHAR DataUpEndp;            //端点地址
    UCHAR DataDnEndp;            //端点地址
    CHAR ProductString[64];      // USB产品字符串
    CHAR ManufacturerString[64]; // USB厂商字符串
    ULONG WriteTimeout;          // USB写超时
    ULONG ReadTimeout;           // USB读超时
    CHAR FuncDescStr[64];
    UCHAR FirewareVer; //固件版本
    ULONG CmdDataMaxSize;
} mDevObjS, *mPDevObj;
#pragma pack()

// CH347模式公用函数,支持CH347所有模式下的打开、关闭、USB读、USB写，包含HID
//打开USB设备
int CH347OpenDevice(ULONG DevI);

//关闭USB设备
BOOL CH347CloseDevice(ULONG iIndex);

// 读取USB数据块
BOOL CH347ReadData(ULONG iIndex,     // 指定设备序号
                   PVOID oBuffer,    // 指向一个足够大的缓冲区,用于保存读取的数据
                   PULONG ioLength); // 指向长度单元,输入时为准备读取的长度,返回后为实际读取的长度

// 写取USB数据块
BOOL CH347WriteData(ULONG iIndex,     // 指定设备序号
                    PVOID iBuffer,    // 指向一个缓冲区,放置准备写出的数据
                    PULONG ioLength); // 指向长度单元,输入时为准备写出的长度,返回后为实际写出的长度

/***************SPI********************/
// SPI控制器初始化
BOOL CH347SPI_Init(ULONG iIndex, mSpiCfgS *SpiCfg);

//获取SPI控制器配置信息
BOOL CH347SPI_GetCfg(ULONG iIndex, mSpiCfgS *SpiCfg);

//设置片选状态,使用前需先调用CH347SPI_Init对CS进行设置
BOOL CH347SPI_ChangeCS(ULONG iIndex,   // 指定设备序号
                       UCHAR iStatus); // 0=撤消片选,1=设置片选

//设置SPI片选
BOOL CH347SPI_SetChipSelect(ULONG iIndex,           // 指定设备序号
                            USHORT iEnableSelect,   // 低八位为CS1，高八位为CS2; 字节值为1=设置CS,为0=忽略此CS设置
                            USHORT iChipSelect,     // 低八位为CS1，高八位为CS2;片选输出,0=撤消片选,1=设置片选
                            ULONG iIsAutoDeativeCS, // 低16位为CS1，高16位为CS2;操作完成后是否自动撤消片选
                            ULONG iActiveDelay,     // 低16位为CS1，高16位为CS2;设置片选后执行读写操作的延时时间,单位us
                            ULONG iDelayDeactive);  // 低16位为CS1，高16位为CS2;撤消片选后执行读写操作的延时时间,单位us

// SPI4写数据
BOOL CH347SPI_Write(ULONG iIndex,      // 指定设备序号
                    ULONG iChipSelect, // 片选控制, 位7为0则忽略片选控制, 位7为1进行片选操作
                    ULONG iLength,     // 准备传输的数据字节数
                    ULONG iWriteStep,  // 准备读取的单个块的长度
                    PVOID ioBuffer);   // 指向一个缓冲区,放置准备从MOSI写出的数据

// SPI4读数据.无需先写数据，效率较CH347SPI_WriteRead高很多
BOOL CH347SPI_Read(ULONG iIndex,      // 指定设备序号
                   ULONG iChipSelect, // 片选控制, 位7为0则忽略片选控制, 位7为1进行片选操作
                   ULONG oLength,     // 准备发出的字节数
                   PULONG iLength,    // 准备读入的数据字节数
                   PVOID ioBuffer);   // 指向一个缓冲区,放置准备从DOUT写出的数据,返回后是从DIN读入的数据

// 处理SPI数据流,4线接口
BOOL CH347SPI_WriteRead(ULONG iIndex,      // 指定设备序号
                        ULONG iChipSelect, // 片选控制, 位7为0则忽略片选控制, 位7为1则操作片选
                        ULONG iLength,     // 准备传输的数据字节数
                        PVOID ioBuffer);   // 指向一个缓冲区,放置准备从DOUT写出的数据,返回后是从DIN读入的数据

// 处理SPI数据流,4线接口
BOOL CH347StreamSPI4(ULONG iIndex,      // 指定设备序号
                     ULONG iChipSelect, // 片选控制, 位7为0则忽略片选控制, 位7为1则参数有效
                     ULONG iLength,     // 准备传输的数据字节数
                     PVOID ioBuffer);   // 指向一个缓冲区,放置准备从DOUT写出的数据,返回后是从DIN读入的数据

/********IIC***********/
typedef enum _EEPROM_TYPE
{ // EEPROM型号
    ID_24C01,
    ID_24C02,
    ID_24C04,
    ID_24C08,
    ID_24C16,
    ID_24C32,
    ID_24C64,
    ID_24C128,
    ID_24C256,
    ID_24C512,
    ID_24C1024,
    ID_24C2048,
    ID_24C4096
} EEPROM_TYPE;

// 设置串口流模式
BOOL CH347I2C_Set(ULONG iIndex, // 指定设备序号
                  ULONG iMode); // 指定模式,见下行
// 位1-位0: I2C接口速度/SCL频率, 00=低速/20KHz,01=标准/100KHz(默认值),10=快速/400KHz,11=高速/750KHz
// 其它保留,必须为0

// 设置硬件异步延时,调用后很快返回,而在下一个流操作之前延时指定毫秒数
BOOL CH347I2C_SetDelaymS(ULONG iIndex,  // 指定设备序号
                         ULONG iDelay); // 指定延时的毫秒数

// 处理I2C数据流,2线接口,时钟线为SCL引脚,数据线为SDA引脚
BOOL CH347StreamI2C(ULONG iIndex,       // 指定设备序号
                    ULONG iWriteLength, // 准备写出的数据字节数
                    PVOID iWriteBuffer, // 指向一个缓冲区,放置准备写出的数据,首字节通常是I2C设备地址及读写方向位
                    ULONG iReadLength,  // 准备读取的数据字节数
                    PVOID oReadBuffer); // 指向一个缓冲区,返回后是读入的数据
                    
BOOL CH347ReadEEPROM(                   // 从EEPROM中读取数据块,速度约56K字节
    ULONG iIndex,                       // 指定CH341设备序号
    EEPROM_TYPE iEepromID,              // 指定EEPROM型号
    ULONG iAddr,                        // 指定数据单元的地址
    ULONG iLength,                      // 准备读取的数据字节数
    PUCHAR oBuffer);                    // 指向一个缓冲区,返回后是读入的数据

// 向EEPROM中写入数据块
BOOL CH347WriteEEPROM(ULONG iIndex,          // 指定设备序号
                      EEPROM_TYPE iEepromID, // 指定EEPROM型号
                      ULONG iAddr,           // 指定数据单元的地址
                      ULONG iLength,         // 准备写出的数据字节数
                      PUCHAR iBuffer);       // 指向一个缓冲区,放置准备写出的数据
#endif