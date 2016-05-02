// exports from <IOKit/usb/IOUSBLib.h>

use libc::{c_char};

pub fn kIOUSBDeviceClassName() -> *const c_char {
    b"IOUSBDevice\0".as_ptr() as *const c_char
}

pub fn kIOUSBInterfaceClassName() -> *const c_char {
    b"IOUSBInterface\0".as_ptr() as *const c_char
}
