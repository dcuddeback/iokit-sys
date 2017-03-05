// exports from <IOKit/hid/IOHIDKeys.h>

use libc::c_int;
use types::IOOptionBits;

pub type IOHIDReportType = c_int;
pub const kIOHIDReportTypeInput: IOHIDReportType   = 0;
pub const kIOHIDReportTypeOutput: IOHIDReportType  = 1;
pub const kIOHIDReportTypeFeature: IOHIDReportType = 2;
pub const kIOHIDReportTypeCount: IOHIDReportType   = 3;

pub type IOHIDManagerOptions = IOOptionBits;
pub const kIOHIDManagerOptionNone: IOHIDManagerOptions                    = 0;
pub const kIOHIDManagerOptionUsePersistentProperties: IOHIDManagerOptions = 1;
pub const kIOHIDManagerOptionDoNotLoadProperties: IOHIDManagerOptions     = 2;
pub const kIOHIDManagerOptionDoNotSaveProperties: IOHIDManagerOptions     = 4;
