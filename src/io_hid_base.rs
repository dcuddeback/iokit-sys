// exports from <IOKit/hid/IOHIDBase.h>

use libc::{c_int, c_void};
use cf::{CFIndex, CFDictionaryRef};
use io_return::IOReturn;
use io_hid_keys::IOHIDReportType;

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDDevice {
    __private: c_void,
}
pub type IOHIDDeviceRef = *mut __IOHIDDevice;

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDElement {
    __private: c_void,
}
pub type IOHIDElementRef = *mut __IOHIDElement;

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDValue {
    __private: c_void,
}
pub type IOHIDValueRef = *mut __IOHIDValue;

// IOHIDTransactionDirectionType
pub type IOHIDTransactionDirectionType = u32;
pub const kIOHIDTransactionDirectionTypeInput:  IOHIDTransactionDirectionType = 0;
pub const kIOHIDTransactionDirectionTypeOutput: IOHIDTransactionDirectionType = 1;

// IOHIDTransactionOption
pub type IOHIDTransactionOption = c_int;
pub const kIOHIDTransactionOptionDefaultOutputValue: IOHIDTransactionOption = 0x0001;

pub type IOHIDCallback = extern fn(context: *mut c_void, result: IOReturn, sender: *mut c_void);

pub type IOHIDReportCallback = extern fn(context: *mut c_void, result: IOReturn, sender: *mut c_void, repotType: IOHIDReportType, reportID: u32, report: *mut u8, reportLength: CFIndex);

pub type IOHIDReportWithTimeStampCallback  = extern fn(context: *mut c_void, result: IOReturn, sender: *mut c_void, repotType: IOHIDReportType, reportID: u32, report: *mut u8, reportLength: CFIndex, timeStamp: u64);

pub type IOHIDValueCallback = extern fn(context: *mut c_void, result: IOReturn, sender: *mut c_void, value: IOHIDValueRef);

pub type IOHIDValueMultipleCallback = extern fn(context: *mut c_void, result: IOReturn, sender: *mut c_void, multiple: CFDictionaryRef);

pub type IOHIDDeviceCallback = extern fn(context: *mut c_void, result: IOReturn, sender: *mut c_void, device: IOHIDDeviceRef);
