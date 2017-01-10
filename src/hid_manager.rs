use libc::c_void;
use cf::{CFAllocatorRef, CFDictionaryRef, CFIndex};
use types::IOOptionBits;
use io_return::IOReturn;

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDManager {
    __private: c_void,
}
pub type IOHIDManagerRef = *const __IOHIDManager;

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDDevice {
    __private: c_void,
}
pub type IOHIDDeviceRef = *const __IOHIDDevice;

pub type IOHIDDeviceCallback = extern fn(context: *mut c_void, result: IOReturn, sender: *mut c_void, device: IOHIDDeviceRef);

pub type IOHIDReportCallback = extern fn(context: *mut c_void, result: IOReturn, sender: *mut c_void, report_type: IOHIDReportType, 
                                         report_id: u32, report: *mut u8, report_len: CFIndex);

pub type IOHIDReportType = u32;
pub const kIOHIDReportTypeInput: IOHIDReportType   = 0;
pub const kIOHIDReportTypeOutput: IOHIDReportType  = 1;
pub const kIOHIDReportTypeFeature: IOHIDReportType = 2;
pub const kIOHIDReportTypeCount: IOHIDReportType   = 3;

pub const kIOHIDManagerOptionNone: IOOptionBits                    = 0;
pub const kIOHIDManagerOptionUsePersistentProperties: IOOptionBits = 1;

extern "C" {
    pub fn IOHIDManagerCreate(allocator: CFAllocatorRef, options: IOOptionBits) -> IOHIDManagerRef;
    pub fn IOHIDManagerSetDeviceMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);
    pub fn IOHIDManagerRegisterDeviceMatchingCallback(manager: IOHIDManagerRef, callback: IOHIDDeviceCallback, context: *mut c_void);
    pub fn IOHIDManagerRegisterDeviceRemovalCallback(manager: IOHIDManagerRef, callback: IOHIDDeviceCallback, context: *mut c_void);
    pub fn IOHIDManagerRegisterInputReportCallback(manager: IOHIDManagerRef, callback: IOHIDReportCallback, context: *mut c_void);
    pub fn IOHIDManagerOpen(manager: IOHIDManagerRef, options: IOOptionBits) -> IOReturn;
    pub fn IOHIDManagerClose(manager: IOHIDManagerRef, options: IOOptionBits) -> IOReturn;
}
