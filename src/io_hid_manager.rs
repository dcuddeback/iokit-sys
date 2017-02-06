// exports from <IOKit/hid/IOHIDManager.h>

use libc::c_void;
use cf::{CFAllocatorRef, CFDictionaryRef};
use io_return::IOReturn;
use io_hid_keys::IOHIDManagerOptions;
use io_hid_base::{IOHIDDeviceCallback, IOHIDReportCallback};

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDManager {
    __private: c_void,
}
pub type IOHIDManagerRef = *mut __IOHIDManager;

extern "C" {
    pub fn IOHIDManagerCreate(allocator: CFAllocatorRef, options: IOHIDManagerOptions) -> IOHIDManagerRef;
    pub fn IOHIDManagerSetDeviceMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);
    pub fn IOHIDManagerRegisterDeviceMatchingCallback(manager: IOHIDManagerRef, callback: IOHIDDeviceCallback, context: *mut c_void);
    pub fn IOHIDManagerRegisterDeviceRemovalCallback(manager: IOHIDManagerRef, callback: IOHIDDeviceCallback, context: *mut c_void);
    pub fn IOHIDManagerRegisterInputReportCallback(manager: IOHIDManagerRef, callback: IOHIDReportCallback, context: *mut c_void);
    pub fn IOHIDManagerOpen(manager: IOHIDManagerRef, options: IOHIDManagerOptions) -> IOReturn;
    pub fn IOHIDManagerClose(manager: IOHIDManagerRef, options: IOHIDManagerOptions) -> IOReturn;
}
