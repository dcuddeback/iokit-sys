// exports from <IOKit/IOKitKeys.h>

use libc::{c_char};

pub fn kIOServiceClass() -> *const c_char {
    b"IOService\0".as_ptr() as *const c_char
}
