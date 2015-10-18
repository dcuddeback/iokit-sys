extern crate IOKit_sys;
extern crate CoreFoundation_sys as cf;
extern crate libc;
extern crate mach;

use std::mem;
use std::ptr;

use std::ffi::{CString,CStr};

use libc::{c_char,c_void};

use mach::port::{mach_port_t,MACH_PORT_NULL};
use mach::kern_return::KERN_SUCCESS;

use IOKit_sys::*;
use cf::*;


fn main() {
    unsafe {
        let mut master_port: mach_port_t = MACH_PORT_NULL;

        let classes_to_match = IOServiceMatching(kIOSerialBSDServiceValue());
        if classes_to_match.is_null() {
            panic!("IOServiceMatching returned a NULL dictionary.");
        }

        // build key
        let key = CFStringCreateWithCString(kCFAllocatorDefault, kIOSerialBSDTypeKey(), kCFStringEncodingUTF8);
        if key.is_null() {
            panic!("failed to allocate key string");
        }

        // build value
        let val = CFStringCreateWithCString(kCFAllocatorDefault, kIOSerialBSDRS232Type(), kCFStringEncodingUTF8);
        if val.is_null() {
            panic!("failed to allocate value string");
        }

        // set value in dictionary
        CFDictionarySetValue(classes_to_match, key as CFTypeRef, val as CFTypeRef);

        let mut kern_result = IOMasterPort(MACH_PORT_NULL, &mut master_port);
        if kern_result != KERN_SUCCESS {
            panic!("ERROR: {}", kern_result);
        }

        let mut matching_services: io_iterator_t = mem::uninitialized();

        kern_result = IOServiceGetMatchingServices(kIOMasterPortDefault, classes_to_match, &mut matching_services);
        if kern_result != KERN_SUCCESS {
            panic!("ERROR: {}", kern_result);
        }

        loop {
            let modem_service = IOIteratorNext(matching_services);

            if modem_service == MACH_PORT_NULL {
                break;
            }

            let mut props = mem::uninitialized();

            let result = IORegistryEntryCreateCFProperties(modem_service, &mut props, kCFAllocatorDefault, 0);
            if result == KERN_SUCCESS {
                CFDictionaryApplyFunction(props, print_property_entry, ptr::null());
            }

            IOObjectRelease(modem_service);
        }
    }
}

extern "C" fn print_property_entry(key: CFTypeRef, value: CFTypeRef, _context: *const c_void) {
    unsafe {
        let mut buf = Vec::<c_char>::with_capacity(256);

        CFStringGetCString(key as CFStringRef, buf.as_mut_ptr(), 256, kCFStringEncodingUTF8);
        print!("{:?} => ", CString::new(CStr::from_ptr(buf.as_ptr()).to_bytes()).unwrap());

        print_object(value);
    }
}

unsafe fn print_object(obj: CFTypeRef) {
    let type_id = CFGetTypeID(obj);

    if type_id == CFStringGetTypeID() {
        let mut buf = Vec::<c_char>::with_capacity(256);

        CFStringGetCString(obj as CFStringRef, buf.as_mut_ptr(), 256, kCFStringEncodingUTF8);
        println!("{:?}", CString::new(CStr::from_ptr(buf.as_ptr()).to_bytes()).unwrap());
    }
    else {
        println!("unhandled type ID: {}", type_id);
    }
}
