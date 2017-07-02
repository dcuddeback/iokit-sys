// exports from <IOKit/IOKitKeys.h>

use libc::{c_char};

pub fn kIOServiceClass() -> *const c_char {
    b"IOService\0".as_ptr() as *const c_char
}
pub fn kIOPublishNotification() -> *const c_char{
	b"IOServicePublish\0".as_ptr() as *const c_char
}
pub fn kIOFirstPublishNotification() -> *const c_char{
	b"IOServiceFirstPublish\0".as_ptr() as *const c_char
}
pub fn kIOMatchedNotification() -> *const c_char{
	b"IOServiceMatched\0".as_ptr() as *const c_char
}
pub fn kIOFirstMatchNotification() -> *const c_char{
	b"IOServiceFirstMatch\0".as_ptr() as *const c_char
}
pub fn kIOTerminatedNotification() -> *const c_char{
	b"IOServiceTerminate\0".as_ptr() as *const c_char
}
