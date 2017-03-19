// exports from <IOKit/hid/IOHIDKeys.h>

use libc::c_int;

pub type IOHIDReportType = c_int;
pub const kIOHIDReportTypeInput: IOHIDReportType   = 0x0;
pub const kIOHIDReportTypeOutput: IOHIDReportType  = 0x1;
pub const kIOHIDReportTypeFeature: IOHIDReportType = 0x2;
pub const kIOHIDReportTypeCount: IOHIDReportType   = 0x3;
