// exports from <IOKit/hid/IOHIDBase.h>

use libc::c_void;
use cf::CFIndex;
use io_return::IOReturn;
use io_hid_keys::IOHIDReportType;

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDDevice {
    __private: c_void,
}
pub type IOHIDDeviceRef = *mut __IOHIDDevice;

pub type IOHIDDeviceCallback = extern fn(context: *mut c_void, result: IOReturn, sender: *mut c_void, device: IOHIDDeviceRef);

pub type IOHIDReportCallback = extern fn(context: *mut c_void, result: IOReturn, sender: *mut c_void, report_type: IOHIDReportType, 
                                         report_id: u32, report: *mut u8, report_len: CFIndex);
