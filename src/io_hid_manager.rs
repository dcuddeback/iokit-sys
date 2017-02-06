// exports from <IOKit/hid/IOHIDManager.h>

use libc::{c_void, c_int};
use cf::{CFAllocatorRef, CFDictionaryRef};
use io_return::IOReturn;
use io_hid_base::{IOHIDDeviceCallback, IOHIDReportCallback};

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDManager {
    __private: c_void,
}
pub type IOHIDManagerRef = *mut __IOHIDManager;

pub type IOHIDReportType = c_int;
pub const kIOHIDReportTypeInput: IOHIDReportType   = 0;
pub const kIOHIDReportTypeOutput: IOHIDReportType  = 1;
pub const kIOHIDReportTypeFeature: IOHIDReportType = 2;
pub const kIOHIDReportTypeCount: IOHIDReportType   = 3;

pub type IOHIDManagerOptions = c_int;
pub const kIOHIDManagerOptionNone: IOHIDManagerOptions                    = 0;
pub const kIOHIDManagerOptionUsePersistentProperties: IOHIDManagerOptions = 1;
pub const kIOHIDManagerOptionDoNotLoadProperties: IOHIDManagerOptions     = 2;
pub const kIOHIDManagerOptionDoNotSaveProperties: IOHIDManagerOptions     = 4;

extern "C" {
    pub fn IOHIDManagerCreate(allocator: CFAllocatorRef, options: IOHIDManagerOptions) -> IOHIDManagerRef;
    pub fn IOHIDManagerSetDeviceMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);
    pub fn IOHIDManagerRegisterDeviceMatchingCallback(manager: IOHIDManagerRef, callback: IOHIDDeviceCallback, context: *mut c_void);
    pub fn IOHIDManagerRegisterDeviceRemovalCallback(manager: IOHIDManagerRef, callback: IOHIDDeviceCallback, context: *mut c_void);
    pub fn IOHIDManagerRegisterInputReportCallback(manager: IOHIDManagerRef, callback: IOHIDReportCallback, context: *mut c_void);
    pub fn IOHIDManagerOpen(manager: IOHIDManagerRef, options: IOHIDManagerOptions) -> IOReturn;
    pub fn IOHIDManagerClose(manager: IOHIDManagerRef, options: IOHIDManagerOptions) -> IOReturn;
}
