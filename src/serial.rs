// exports from <IOKit/serial/IOSerialKeys.h>

use libc::{c_char};

pub fn kIOSerialBSDServiceValue() -> *const c_char {
    b"IOSerialBSDClient\0".as_ptr() as *const c_char
}

pub fn kIOSerialBSDTypeKey() -> *const c_char {
    b"IOSerialBSDClientType\0".as_ptr() as *const c_char
}

pub fn kIOSerialBSDAllTypes() -> *const c_char {
    b"IOSerialStream\0".as_ptr() as *const c_char
}

pub fn kIOSerialBSDModemType() -> *const c_char {
    b"IOModemSerialStream\0".as_ptr() as *const c_char
}

pub fn kIOSerialBSDRS232Type() -> *const c_char {
    b"IORS232SerialStream\0".as_ptr() as *const c_char
}

pub fn kIOUSBDeviceClassName() -> *const c_char {
    b"IOUSBDevice\0".as_ptr() as *const c_char
}

pub fn kIOUSBInterfaceClassName() -> *const c_char {
    b"IOUSBInterface\0".as_ptr() as *const c_char
}

pub fn kIOServiceClass() -> *const c_char {
    b"IOService\0".as_ptr() as *const c_char
}

pub fn kIOTTYDeviceKey() -> *const c_char {
    b"IOTTYDevice\0".as_ptr() as *const c_char
}

pub fn kIOTTYBaseNameKey() -> *const c_char {
    b"IOTTYBaseName\0".as_ptr() as *const c_char
}

pub fn kIOTTYSuffixKey() -> *const c_char {
    b"IOTTYSuffix\0".as_ptr() as *const c_char
}

pub fn kIOCalloutDeviceKey() -> *const c_char {
    b"IOCalloutDevice\0".as_ptr() as *const c_char
}

pub fn kIODialinDeviceKey() -> *const c_char {
    b"IODialinDevice\0".as_ptr() as *const c_char
}

pub fn kIOTTYWaitForIdleKey() -> *const c_char {
    b"IOTTYWaitForIdle\0".as_ptr() as *const c_char
}
