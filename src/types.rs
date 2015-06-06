// exports from <IOKit/IOTypes.h>

use libc::{c_char,c_int,c_uint};

use mach::port::mach_port_t;
use mach::vm_types::mach_vm_address_t;

pub type IOOptionBits = u32;
pub type IOFixed      = i32;
pub type IOVersion    = u32;
pub type IOItemCount  = u32;
pub type IOCacheMode  = u32;

pub type IOByteCount32 = u32;
pub type IOByteCount64 = u64;

pub type IOPhysicalAddress32 = u32;
pub type IOPhysicalAddress64 = u64;
pub type IOPhysicalLength32  = u32;
pub type IOPhysicalLength64  = u64;


#[cfg(target_pointer_width = "64")] pub type IOVirtualAddress = mach_vm_address_t;
#[cfg(target_pointer_width = "32")] pub type IOVirtualAddress = vm_address_t;

#[cfg(target_pointer_width = "64")] pub type IOByteCount = IOByteCount64;
#[cfg(target_pointer_width = "32")] pub type IOByteCount = IOByteCount32;

pub type IOLogicalAddress = IOVirtualAddress;

#[cfg(target_pointer_width = "64")] pub type IOPhysicalAddress = IOPhysicalAddress64;
#[cfg(target_pointer_width = "64")] pub type IOPhysicalLength = IOPhysicalLength64;

#[cfg(target_pointer_width = "32")] pub type IOPhysicalAddress = IOPhysicalAddress32;
#[cfg(target_pointer_width = "32")] pub type IOPhysicalLength = IOPhysicalLength32;

#[repr(C)]
pub struct IOPhysicalRange {
    address: IOPhysicalAddress,
    length: IOByteCount
}

#[repr(C)]
pub struct IOVirtualRange {
    address: IOVirtualAddress,
    length: IOByteCount
}

#[cfg(target_pointer_width = "64")]
pub type IOAddressRange = IOVirtualRange;

#[cfg(target_pointer_width = "32")]
#[repr(C)]
pub struct IOAddressRange {
    address: mach_vm_address_t,
    length: mach_vm_size_t
}

#[repr(C)]
pub struct IONamedValue {
    value: c_int,
    name: *const c_char
}

pub type IOAlignment = c_uint;

pub type io_object_t = mach_port_t;

pub type io_connect_t        = io_object_t;
pub type io_enumerator_t     = io_object_t;
pub type io_iterator_t       = io_object_t;
pub type io_registry_entry_t = io_object_t;
pub type io_service_t        = io_object_t;

pub const IO_OBJECT_NULL: io_object_t = 0;

// IOConnectMapMemory memory types
pub const kIODefaultMemoryType: c_int = 0;

// cache types
pub const kIODefaultCache:       c_int = 0;
pub const kIOInhibitCache:       c_int = 1;
pub const kIOWriteThruCache:     c_int = 2;
pub const kIOCopybackCache:      c_int = 3;
pub const kIOWriteCombineCache:  c_int = 4;
pub const kIOCopybackInnerCache: c_int = 5;

// IOMemory mapping options
pub const kIOMapAnywhere:           c_int = 0x00000001;
pub const kIOMapCacheMask:          c_int = 0x0000700;
pub const kIOMapCacheShift:         c_int = 8;
pub const kIOMapDefaultCache:       c_int = kIODefaultCache       << kIOMapCacheShift;
pub const kIOMapInhibitCache:       c_int = kIOInhibitCache       << kIOMapCacheShift;
pub const kIOMapWriteThruCache:     c_int = kIOWriteThruCache     << kIOMapCacheShift;
pub const kIOMapCopybackCache:      c_int = kIOCopybackCache      << kIOMapCacheShift;
pub const kIOMapWriteCombineCache:  c_int = kIOWriteCombineCache  << kIOMapCacheShift;
pub const kIOMapCopybackInnerCache: c_int = kIOCopybackInnerCache << kIOMapCacheShift;
pub const kIOMapUserOptionsMask:    c_int = 0x00000FFF;
pub const kIOMapReadOnly:           c_int = 0x00001000;
pub const kIOMapStatic:             c_int = 0x01000000;
pub const kIOMapReference:          c_int = 0x02000000;
pub const kIOMapUnique:             c_int = 0x04000000;
pub const kIOMapPrefault:           c_int = 0x10000000;

// scale factors
pub const kNanosecondScale:   c_int = 1;
pub const kMicrosecondScale:  c_int = 1000;
pub const kMillisecondScale:  c_int = 1000 * 1000;
pub const kSecondScale:       c_int = 1000 * 1000 * 1000;
pub const kTickScale:         c_int = (kSecondScale / 100);

pub const kIOConnectMethodVarOutputSize: c_int = -3;

pub type IODeviceNumber = c_uint;
