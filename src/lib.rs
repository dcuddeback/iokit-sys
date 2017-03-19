#![allow(non_camel_case_types,non_upper_case_globals,non_snake_case)]

extern crate CoreFoundation_sys as cf;
extern crate libc;
extern crate mach;

use cf::{CFTypeRef,CFDictionaryRef,CFMutableDictionaryRef,CFStringRef,CFAllocatorRef};
use libc::{c_void,c_char,c_int,size_t,uintptr_t};

use mach::boolean::boolean_t;
use mach::clock_types::mach_timespec_t;
use mach::kern_return::kern_return_t;
use mach::port::mach_port_t;
use mach::types::task_port_t;
use mach::vm_types::{mach_vm_address_t,mach_vm_size_t};

pub use io_return::*;
pub use keys::*;
pub use serial::*;
pub use types::*;
pub use usb::*;
pub use io_hid_keys::*;
pub use io_hid_base::*;
pub use io_hid_manager::*;

mod io_return;
mod keys;
mod serial;
mod types;
mod usb;
mod io_hid_keys;
mod io_hid_base;
mod io_hid_manager;

// exports from <IOKit/IOKitLib.h>

#[repr(C)]
pub struct IONotificationPort {
    __private: c_void,
}

pub type IONotificationPortRef = *mut IONotificationPort;

pub type IOServiceMatchingCallback = extern fn (refcon: *mut c_void, iterator: io_iterator_t);
pub type IOServiceInterestCallback = extern fn (refcon: *mut c_void, service: io_service_t, messageType: u32, messageArgument: *mut c_void);

// options for IOServiceAuthorize()
pub const kIOServiceInteractionAllowed: u32 = 0x00000001;

// options for IORegistryCreateIterator(), IORegistryEntryCreateIterator(), IORegistryEntrySearchCFProperty()
pub const kIORegistryIterateRecursively: IOOptionBits = 0x00000001;
pub const kIORegistryIterateParents:     IOOptionBits = 0x00000002;

type IOAsyncCallback0 = extern fn (refcon: *mut c_void, result: IOReturn);
type IOAsyncCallback1 = extern fn (refcon: *mut c_void, result: IOReturn, arg0: *mut c_void);
type IOAsyncCallback2 = extern fn (refcon: *mut c_void, result: IOReturn, arg0: *mut c_void, arg1: *mut c_void);
type IOAsyncCallback = extern fn (refcon: *mut c_void, result: IOReturn, args: *mut *mut c_void, numArgs: u32);

extern "C" {
    pub static kIOMasterPortDefault: mach_port_t;

    pub fn IOMasterPort(bootstrapPort: mach_port_t, masterPort: *mut mach_port_t) -> kern_return_t;

    pub fn IONotificationPortCreate(masterPort: mach_port_t) -> IONotificationPortRef;
    pub fn IONotificationPortDestroy(notify: IONotificationPortRef);
    pub fn IONotificationPortGetMachPort(notify: IONotificationPortRef) -> mach_port_t;

    pub fn IOCreateReceivePort(msgType: u32, recvPort: *mut mach_port_t) -> kern_return_t;

    // IOObject

    pub fn IOObjectRelease(object: io_object_t) -> kern_return_t;
    pub fn IOObjectRetain(object: io_object_t) -> kern_return_t;
    pub fn IOObjectGetClass(object: io_object_t, className: *mut c_char) -> kern_return_t;
    pub fn IOObjectCopyClass(object: io_object_t) -> CFStringRef;
    pub fn IOObjectCopySuperclassForClass(classname: CFStringRef) -> CFStringRef;
    pub fn IOObjectCopyBundleIdentifierForClass(classname: CFStringRef) -> CFStringRef;
    pub fn IOObjectConformsTo(object: io_object_t, className: *const c_char) -> boolean_t;
    pub fn IOObjectGetKernelRetainCount(object: io_object_t) -> u32;
    pub fn IOObjectGetUserRetainCount(object: io_object_t) -> u32;
    pub fn IOObjectGetRetainCount(object: io_object_t) -> u32;

    // IOIterator, subclass of IOObject

    pub fn IOIteratorNext(iterator: io_iterator_t) -> io_object_t;
    pub fn IOIteratorReset(iterator: io_iterator_t);
    pub fn IOIteratorIsValid(iterator: io_iterator_t) -> boolean_t;

    // IOService, subclass of IORegistryEntry

    pub fn IOServiceGetMatchingService(masterPort: mach_port_t, matching: CFDictionaryRef) -> io_service_t;
    pub fn IOServiceGetMatchingServices(masterPort: mach_port_t, matching: CFDictionaryRef, existing: *mut io_iterator_t) -> kern_return_t;
    pub fn IOServiceAddNotification(masterPort: mach_port_t, notificationType: *const c_char, matching: CFDictionaryRef, wakePort: mach_port_t, reference: uintptr_t, notification: *mut io_iterator_t) -> kern_return_t;
    pub fn IOServiceAddMatchingNotification(notifyPort: IONotificationPortRef, notificationType: *const c_char, matching: CFDictionaryRef, callback: IOServiceMatchingCallback, refCon: *mut c_void, notification: *mut io_iterator_t) -> kern_return_t;
    pub fn IOServiceAddInterestNotification(notifyPort: IONotificationPortRef, service: io_service_t, interestType: *const c_char, callback: IOServiceInterestCallback, refCon: *mut c_void, notification: *mut io_object_t) -> kern_return_t;
    pub fn IOServiceMatchPropertyTable(service: io_service_t, matching: CFDictionaryRef, matches: *mut boolean_t) -> kern_return_t;
    pub fn IOServiceGetBusyState(service: io_service_t, busyState: *mut u32) -> kern_return_t;
    pub fn IOServiceWaitQuiet(service: io_service_t, waitTime: *mut mach_timespec_t) -> kern_return_t;

    pub fn IOKitGetBusyState(masterPort: mach_port_t, busyState: *mut u32) -> kern_return_t;
    pub fn IOKitWaitQuiet(masterPort: mach_port_t, waitTime: *mut mach_timespec_t) -> kern_return_t;

    pub fn IOServiceOpen(service: io_service_t, owningTask: task_port_t, connectionType: u32, connect: *mut io_connect_t) -> kern_return_t;
    pub fn IOServiceRequestProbe(service: io_service_t, options: u32) -> kern_return_t;
    pub fn IOServiceAuthorize(service: io_service_t, options: u32) -> kern_return_t;
    pub fn IOServiceOpenAsFileDescriptor(service: io_service_t, oflag: c_int) -> c_int;

    // IOService connection

    pub fn IOServiceClose(connect: io_connect_t) -> kern_return_t;
    pub fn IOConnectAddRef(connect: io_connect_t) -> kern_return_t;
    pub fn IOConnectRelease(connect: io_connect_t) -> kern_return_t;
    pub fn IOConnectGetService(connect: io_connect_t, service: *mut io_service_t) -> kern_return_t;
    pub fn IOConnectSetNotificationPort(connect: io_connect_t, notificationType: u32, port: mach_port_t, reference: uintptr_t) -> kern_return_t;
    pub fn IOConnectMapMemory64(connect: io_connect_t, memoryType: u32, intoTask: task_port_t, atAddress: *mut mach_vm_address_t, ofSize: *mut mach_vm_size_t, options: IOOptionBits) -> kern_return_t;
    pub fn IOConnectUnmapMemory64(connect: io_connect_t, memoryType: u32, fromTask: task_port_t, atAddress: mach_vm_address_t) -> kern_return_t;
    pub fn IOConnectSetCFProperties(connect: io_connect_t, properties: CFTypeRef) -> kern_return_t;
    pub fn IOConnectSetCFProperty(conect: io_connect_t, propertyName: CFStringRef, property: CFTypeRef) -> kern_return_t;

    pub fn IOConnectCallMethod(connection: mach_port_t, selector: u32, input: *const u64, inputCnt: u32, inputStruct: *const c_void, inputStructCnt: size_t, output: *mut u64, outputCnt: *mut u32, outputStruct: *mut c_void, outputStructCnt: *mut size_t) -> kern_return_t;
    pub fn IOConnectCallAsyncMethod(connection: mach_port_t, selector: u32, wake_port: mach_port_t, reference: *mut u64, referenceCnt: u32, input: *const u64, inputStruct: *const c_void, inputStructCnt: size_t, output: *mut u64, outputCnt: *mut u32, outputStruct: *mut c_void, outputStructCnt: *mut size_t) -> kern_return_t;
    pub fn IOConnectCallStructMethod(connection: mach_port_t, selector: u32, inputStruct: *const c_void, inputStructCnt: size_t, outputStruct: *mut c_void, outputStructCnt: *mut size_t) -> kern_return_t;
    pub fn IOConnectCallAsyncStructMethod(connection: mach_port_t, selector: u32, wake_port: mach_port_t, reference: *mut u64, referenceCnt: u32, inputStruct: *const c_void, inputStructCnt: size_t, outputStruct: *mut c_void, outputStructCnt: *mut size_t) -> kern_return_t;
    pub fn IOConnectCallScalarMethod(connection: mach_port_t, selector: u32, input: *const u64, inputCnt: u32, output: *mut u64, outputCnt: *mut u32) -> kern_return_t;
    pub fn IOConnectCallAsyncScalarMethod(connection: mach_port_t, selector: u32, wake_port: mach_port_t, reference: *mut u64, referenceCnt: u32, input: *const u64, inputCnt: u32, output: *mut u64, outputCnt: *mut u32) -> kern_return_t;

    pub fn IOConnectTrap0(connect: io_connect_t, index: u32) -> kern_return_t;
    pub fn IOConnectTrap1(connect: io_connect_t, index: u32, p1: uintptr_t) -> kern_return_t;
    pub fn IOConnectTrap2(connect: io_connect_t, index: u32, p1: uintptr_t, p2: uintptr_t) -> kern_return_t;
    pub fn IOConnectTrap3(connect: io_connect_t, index: u32, p1: uintptr_t, p2: uintptr_t, p3: uintptr_t) -> kern_return_t;
    pub fn IOConnectTrap4(connect: io_connect_t, index: u32, p1: uintptr_t, p2: uintptr_t, p3: uintptr_t, p4: uintptr_t) -> kern_return_t;
    pub fn IOConnectTrap5(connect: io_connect_t, index: u32, p1: uintptr_t, p2: uintptr_t, p3: uintptr_t, p4: uintptr_t, p5: uintptr_t) -> kern_return_t;
    pub fn IOConnectTrap6(connect: io_connect_t, index: u32, p1: uintptr_t, p2: uintptr_t, p3: uintptr_t, p4: uintptr_t, p5: uintptr_t, p6: uintptr_t) -> kern_return_t;
    pub fn IOConnectAddClient(connect: io_connect_t, client: io_connect_t) -> kern_return_t;

    // IORegistry accessors

    pub fn IORegistryGetRootEntry(masterPort: mach_port_t) -> io_registry_entry_t;
    pub fn IORegistryEntryFromPath(masterPort: mach_port_t, path: *const c_char) -> io_registry_entry_t;
    pub fn IORegistryCreateIterator(masterPort: mach_port_t, plane: *const c_char, options: IOOptionBits, iterator: *mut io_iterator_t) -> kern_return_t;
    pub fn IORegistryEntryCreateIterator(masterPort: mach_port_t, plane: *const c_char, options: IOOptionBits, iterator: *mut io_iterator_t) -> kern_return_t;

    // IORegistryIterator, subclass of IOIterator

    pub fn IORegistryIteratorEnterEntry(iterator: io_iterator_t) -> kern_return_t;
    pub fn IORegistryIteratorExitEntry(iterator: io_iterator_t) -> kern_return_t;

    // IORegistryEntry, subclass of IOObject

    pub fn IORegistryEntryGetName(entry: io_registry_entry_t, name: *mut c_char) -> kern_return_t;
    pub fn IORegistryEntryGetNameInPlane(entry: io_registry_entry_t, plane: *const c_char, name: *mut c_char) -> kern_return_t;
    pub fn IORegistryEntryGetLocationInPlane(entry: io_registry_entry_t, plane: *const c_char, location: *mut c_char) -> kern_return_t;
    pub fn IORegistryEntryGetPath(entry: io_registry_entry_t, plane: *const c_char, path: *mut c_char) -> kern_return_t;
    pub fn IORegistryEntryGetRegistryEntryID(entry: io_registry_entry_t, entryID: *mut u64) -> kern_return_t;
    pub fn IORegistryEntryCreateCFProperties(entry: io_registry_entry_t, properties: *mut CFMutableDictionaryRef, allocator: CFAllocatorRef, options: IOOptionBits) -> kern_return_t;
    pub fn IORegistryEntryCreateCFProperty(entry: io_registry_entry_t, key: CFStringRef, allocator: CFAllocatorRef, options: IOOptionBits) -> CFTypeRef;
    pub fn IORegistryEntrySetCFProperties(entry: io_registry_entry_t, properties: CFTypeRef) -> kern_return_t;
    pub fn IORegistryEntrySetCFProperty(entry: io_registry_entry_t, propertyName: CFStringRef, property: CFTypeRef) -> kern_return_t;
    pub fn IORegistryEntryGetChildIterator(entry: io_registry_entry_t, plane: *const c_char, iterator: *mut io_iterator_t) -> kern_return_t;
    pub fn IORegistryEntryGetChildEntry(entry: io_registry_entry_t, plane: *const c_char, child: *mut io_registry_entry_t) -> kern_return_t;
    pub fn IORegistryEntryGetParentIterator(entry: io_registry_entry_t, plane: *const c_char, iterator: *mut io_iterator_t) -> kern_return_t;
    pub fn IORegistryEntryGetParentEntry(entry: io_registry_entry_t, plane: *const c_char, parent: *mut io_registry_entry_t) -> kern_return_t;
    pub fn IORegistryEntryInPlane(entry: io_registry_entry_t, plane: *const c_char) -> boolean_t;

    // matching dictionary creation helpers

    pub fn IOServiceMatching(name: *const c_char) -> CFMutableDictionaryRef;
    pub fn IOServiceNameMatching(name: *const c_char) -> CFMutableDictionaryRef;
    pub fn IOBSDNameMatching(masterPort: mach_port_t, options: u32, bsdName: *const c_char) -> CFMutableDictionaryRef;
    pub fn IORegistryEntryIDMatching(entryID: u64) -> CFMutableDictionaryRef;
}
