use libc;

// see https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types

/// windows filesystem path limit
///
/// <https://docs.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation>
#[allow(dead_code)]
pub const MAX_PATH: usize = 260;

/// A Boolean variable (should be TRUE or FALSE).
///
/// This type is declared in `WinDef.h` as follows:
/// ```c
/// typedef int BOOL;
/// ```
#[allow(dead_code)]
pub type BOOL = libc::c_int;

/// An 8-bit Windows (ANSI) character. For more information, see Character Sets Used By Fonts.
///
/// This type is declared in `WinNT.h` as follows:
/// ```c
/// typedef char CHAR;
/// ```
#[allow(dead_code)]
pub type CHAR = libc::c_char;

/// An unsigned CHAR.
///
/// This type is declared in `WinDef.h` as follows:
/// ```c
/// typedef unsigned char UCHAR;
/// ```
#[allow(dead_code)]
pub type UCHAR = libc::c_uchar;

#[allow(dead_code)]
pub type PCHAR = *mut libc::c_char;

#[allow(dead_code)]
pub type PUCHAR = *mut libc::c_uchar;

#[allow(dead_code)]
pub type USHORT = libc::c_ushort;

#[allow(dead_code)]
pub type SHORT = libc::c_short;

#[allow(dead_code)]
pub type ULONG = libc::c_ulong;

#[allow(dead_code)]
pub type PULONG = *mut ULONG;

#[allow(dead_code)]
pub type LONG = libc::c_long;

#[allow(dead_code)]
pub type VOID = libc::c_void;

#[allow(dead_code)]
pub type PVOID = *mut VOID;

/// A handle to an object.
///
/// This type is declared in WinNT.h as follows:
/// ```c
/// typedef PVOID HANDLE;
/// ```
#[allow(dead_code)]
pub type HANDLE = PVOID;

/// The calling convention for callback functions.
///
/// This type is declared in WinDef.h as follows:
/// ```c
/// #define CALLBACK __stdcall
/// ```
///
/// CALLBACK, WINAPI, and APIENTRY are all used to define functions with the __stdcall calling convention.
///
/// Most functions in the Windows API are declared using WINAPI. You may wish to use CALLBACK for the callback functions that you implement to help identify the function as a callback function.
#[allow(dead_code)]
pub type CALLBACK = fn();
