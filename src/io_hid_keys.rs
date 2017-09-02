// exports from <IOKit/hid/IOHIDKeys.h>

use libc::{c_char, c_int};

// kIOHIDDeviceKey
pub fn kIOHIDDeviceKey() -> *const c_char {
    b"IOHIDDevice\0".as_ptr() as *const c_char
}

// HID Device Property Keys
pub fn kIOHIDTransportKey() -> *const c_char {
    b"Transport\0".as_ptr() as *const c_char
}

pub fn kIOHIDVendorIDKey() -> *const c_char {
    b"VendorID\0".as_ptr() as *const c_char
}

pub fn kIOHIDVendorIDSourceKey() -> *const c_char {
    b"VendorIDSource\0".as_ptr() as *const c_char
}

pub fn kIOHIDProductIDKey() -> *const c_char {
    b"ProductID\0".as_ptr() as *const c_char
}

pub fn kIOHIDVersionNumberKey() -> *const c_char {
    b"VersionNumber\0".as_ptr() as *const c_char
}

pub fn kIOHIDManufacturerKey() -> *const c_char {
    b"Manufacturer\0".as_ptr() as *const c_char
}

pub fn kIOHIDProductKey() -> *const c_char {
    b"Product\0".as_ptr() as *const c_char
}

pub fn kIOHIDSerialNumberKey() -> *const c_char {
    b"SerialNumber\0".as_ptr() as *const c_char
}

pub fn kIOHIDCountryCodeKey() -> *const c_char {
    b"CountryCode\0".as_ptr() as *const c_char
}

pub fn kIOHIDStandardTypeKey() -> *const c_char {
    b"StandardType\0".as_ptr() as *const c_char
}

pub fn kIOHIDLocationIDKey() -> *const c_char {
    b"LocationID\0".as_ptr() as *const c_char
}

pub fn kIOHIDDeviceUsageKey() -> *const c_char {
    b"DeviceUsage\0".as_ptr() as *const c_char
}

pub fn kIOHIDDeviceUsagePageKey() -> *const c_char {
    b"DeviceUsagePage\0".as_ptr() as *const c_char
}

pub fn kIOHIDDeviceUsagePairsKey() -> *const c_char {
    b"DeviceUsagePairs\0".as_ptr() as *const c_char
}

pub fn kIOHIDPrimaryUsageKey() -> *const c_char {
    b"PrimaryUsage\0".as_ptr() as *const c_char
}

pub fn kIOHIDPrimaryUsagePageKey() -> *const c_char {
    b"PrimaryUsagePage\0".as_ptr() as *const c_char
}

pub fn kIOHIDMaxInputReportSizeKey() -> *const c_char {
    b"MaxInputReportSize\0".as_ptr() as *const c_char
}

pub fn kIOHIDMaxOutputReportSizeKey() -> *const c_char {
    b"MaxOutputReportSize\0".as_ptr() as *const c_char
}

pub fn kIOHIDMaxFeatureReportSizeKey() -> *const c_char {
    b"MaxFeatureReportSize\0".as_ptr() as *const c_char
}

pub fn kIOHIDReportIntervalKey() -> *const c_char {
    b"ReportInterval\0".as_ptr() as *const c_char
}

pub fn kIOHIDSampleIntervalKey() -> *const c_char {
    b"SampleInterval\0".as_ptr() as *const c_char
}

pub fn kIOHIDBatchIntervalKey() -> *const c_char {
    b"BatchInterval\0".as_ptr() as *const c_char
}

pub fn kIOHIDRequestTimeoutKey() -> *const c_char {
    b"RequestTimeout\0".as_ptr() as *const c_char
}

pub fn kIOHIDReportDescriptorKey() -> *const c_char {
    b"ReportDescriptor\0".as_ptr() as *const c_char
}

pub fn kIOHIDResetKey() -> *const c_char {
    b"Reset\0".as_ptr() as *const c_char
}

pub fn kIOHIDKeyboardLanguageKey() -> *const c_char {
    b"KeyboardLanguage\0".as_ptr() as *const c_char
}

pub fn kIOHIDAltHandlerIdKey() -> *const c_char {
    b"alt_handler_id\0".as_ptr() as *const c_char
}

pub fn kIOHIDBuiltInKey() -> *const c_char {
    b"Built-In\0".as_ptr() as *const c_char
}

pub fn kIOHIDDisplayIntegratedKey() -> *const c_char {
    b"DisplayIntegrated\0".as_ptr() as *const c_char
}

pub fn kIOHIDProductIDMaskKey() -> *const c_char {
    b"ProductIDMask\0".as_ptr() as *const c_char
}

pub fn kIOHIDProductIDArrayKey() -> *const c_char {
    b"ProductIDArray\0".as_ptr() as *const c_char
}

pub fn kIOHIDPowerOnDelayNSKey() -> *const c_char {
    b"HIDPowerOnDelayNS\0".as_ptr() as *const c_char
}

pub fn kIOHIDCategoryKey() -> *const c_char {
    b"Category\0".as_ptr() as *const c_char
}

pub fn kIOHIDMaxResponseLatencyKey() -> *const c_char {
    b"MaxResponseLatency\0".as_ptr() as *const c_char
}

pub fn kIOHIDUniqueIDKey() -> *const c_char {
    b"UniqueID\0".as_ptr() as *const c_char
}

pub fn kIOHIDPhysicalDeviceUniqueIDKey() -> *const c_char {
    b"PhysicalDeviceUniqueID\0".as_ptr() as *const c_char
}

pub fn kIOHIDTransportUSBValue() -> *const c_char {
    b"USB\0".as_ptr() as *const c_char
}

pub fn kIOHIDTransportBluetoothValue() -> *const c_char {
    b"Bluetooth\0".as_ptr() as *const c_char
}

pub fn kIOHIDTransportBluetoothLowEnergyValue() -> *const c_char {
    b"BluetoothLowEnergy\0".as_ptr() as *const c_char
}

pub fn kIOHIDTransportAIDBValue() -> *const c_char {
    b"AIDB\0".as_ptr() as *const c_char
}

pub fn kIOHIDTransportI2CValue() -> *const c_char {
    b"I2C\0".as_ptr() as *const c_char
}

pub fn kIOHIDTransportSPIValue() -> *const c_char {
    b"SPI\0".as_ptr() as *const c_char
}

pub fn kIOHIDTransportSerialValue() -> *const c_char {
    b"Serial\0".as_ptr() as *const c_char
}

pub fn kIOHIDTransportIAPValue() -> *const c_char {
    b"IAP\0".as_ptr() as *const c_char
}

pub fn kIOHIDTransportAirPlayValue() -> *const c_char {
    b"AirPlay\0".as_ptr() as *const c_char
}

pub fn kIOHIDTransportSPUValue() -> *const c_char {
    b"SPU\0".as_ptr() as *const c_char
}

pub fn kIOHIDCategoryAutomotiveValue() -> *const c_char {
    b"Automotive\0".as_ptr() as *const c_char
}

// kIOHIDElementKey
pub fn kIOHIDElementKey() -> *const c_char {
    b"Elements\0".as_ptr() as *const c_char
}

// HID Element Dictionary Keys
pub fn kIOHIDElementCookieKey() -> *const c_char {
    b"ElementCookie\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementTypeKey() -> *const c_char {
    b"Type\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementCollectionTypeKey() -> *const c_char {
    b"CollectionType\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementUsageKey() -> *const c_char {
    b"Usage\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementUsagePageKey() -> *const c_char {
    b"UsagePage\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementMinKey() -> *const c_char {
    b"Min\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementMaxKey() -> *const c_char {
    b"Max\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementScaledMinKey() -> *const c_char {
    b"ScaledMin\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementScaledMaxKey() -> *const c_char {
    b"ScaledMax\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementSizeKey() -> *const c_char {
    b"Size\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementReportSizeKey() -> *const c_char {
    b"ReportSize\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementReportCountKey() -> *const c_char {
    b"ReportCount\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementReportIDKey() -> *const c_char {
    b"ReportID\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementIsArrayKey() -> *const c_char {
    b"IsArray\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementIsRelativeKey() -> *const c_char {
    b"IsRelative\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementIsWrappingKey() -> *const c_char {
    b"IsWrapping\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementIsNonLinearKey() -> *const c_char {
    b"IsNonLinear\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementHasPreferredStateKey() -> *const c_char {
    b"HasPreferredState\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementHasNullStateKey() -> *const c_char {
    b"HasNullState\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementFlagsKey() -> *const c_char {
    b"Flags\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementUnitKey() -> *const c_char {
    b"Unit\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementUnitExponentKey() -> *const c_char {
    b"UnitExponent\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementNameKey() -> *const c_char {
    b"Name\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementValueLocationKey() -> *const c_char {
    b"ValueLocation\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementDuplicateIndexKey() -> *const c_char {
    b"DuplicateIndex\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementParentCollectionKey() -> *const c_char {
    b"ParentCollection\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementVendorSpecificKey() -> *const c_char {
    b"VendorSpecifc\0".as_ptr() as *const c_char
}

// HID Element Match Keys
pub fn kIOHIDElementCookieMinKey() -> *const c_char {
    b"ElementCookieMin\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementCookieMaxKey() -> *const c_char {
    b"ElementCookieMax\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementUsageMinKey() -> *const c_char {
    b"UsageMin\0".as_ptr() as *const c_char
}

pub fn kIOHIDElementUsageMaxKey() -> *const c_char {
    b"UsageMax\0".as_ptr() as *const c_char
}

// kIOHIDElementCalibrationMinKey
pub fn kIOHIDElementCalibrationMinKey() -> *const c_char {
    b"CalibrationMin\0".as_ptr() as *const c_char
}

// kIOHIDElementCalibrationMaxKey
pub fn kIOHIDElementCalibrationMaxKey() -> *const c_char {
    b"CalibrationMax\0".as_ptr() as *const c_char
}

// kIOHIDElementCalibrationSaturationMinKey
pub fn kIOHIDElementCalibrationSaturationMinKey() -> *const c_char {
    b"CalibrationSaturationMin\0".as_ptr() as *const c_char
}

// kIOHIDElementCalibrationSaturationMaxKey
pub fn kIOHIDElementCalibrationSaturationMaxKey() -> *const c_char {
    b"CalibrationSaturationMax\0".as_ptr() as *const c_char
}

// kIOHIDElementCalibrationDeadZoneMinKey
pub fn kIOHIDElementCalibrationDeadZoneMinKey() -> *const c_char {
    b"CalibrationDeadZoneMin\0".as_ptr() as *const c_char
}

// kIOHIDElementCalibrationDeadZoneMaxKey
pub fn kIOHIDElementCalibrationDeadZoneMaxKey() -> *const c_char {
    b"CalibrationDeadZoneMax\0".as_ptr() as *const c_char
}

// kIOHIDElementCalibrationGranularityKey
pub fn kIOHIDElementCalibrationGranularityKey() -> *const c_char {
    b"CalibrationGranularity\0".as_ptr() as *const c_char
}

// IOHIDElementType
pub type IOHIDElementType = c_int;
pub const kIOHIDElementTypeInput_Misc:      IOHIDElementType = 1;
pub const kIOHIDElementTypeInput_Button:    IOHIDElementType = 2;
pub const kIOHIDElementTypeInput_Axis:      IOHIDElementType = 3;
pub const kIOHIDElementTypeInput_ScanCodes: IOHIDElementType = 4;
pub const kIOHIDElementTypeOutput:          IOHIDElementType = 129;
pub const kIOHIDElementTypeFeature:         IOHIDElementType = 257;
pub const kIOHIDElementTypeCollection:      IOHIDElementType = 513;

// IOHIDElementCollectionType
pub type IOHIDElementCollectionType = c_int;
pub const kIOHIDElementCollectionTypePhysical:      IOHIDElementCollectionType = 0x00;
pub const kIOHIDElementCollectionTypeApplication:   IOHIDElementCollectionType = 0x01;
pub const kIOHIDElementCollectionTypeLogical:       IOHIDElementCollectionType = 0x02;
pub const kIOHIDElementCollectionTypeReport:        IOHIDElementCollectionType = 0x03;
pub const kIOHIDElementCollectionTypeNamedArray:    IOHIDElementCollectionType = 0x04;
pub const kIOHIDElementCollectionTypeUsageSwitch:   IOHIDElementCollectionType = 0x05;
pub const kIOHIDElementCollectionTypeUsageModifier: IOHIDElementCollectionType = 0x06;

// IOHIDReportType
pub type IOHIDReportType = c_int;
pub const kIOHIDReportTypeInput:   IOHIDReportType = 0;
pub const kIOHIDReportTypeOutput:  IOHIDReportType = 1;
pub const kIOHIDReportTypeFeature: IOHIDReportType = 2;
pub const kIOHIDReportTypeCount:   IOHIDReportType = 3;

// IOHIDOptionsType
pub type IOHIDOptionsType = u32;
pub const kIOHIDOptionsTypeNone:        IOHIDOptionsType = 0x00;
pub const kIOHIDOptionsTypeSeizeDevice: IOHIDOptionsType = 0x01;

// IOHIDQueueOptionsType
pub type IOHIDQueueOptionsType = u32;
pub const kIOHIDQueueOptionsTypeNone:       IOHIDQueueOptionsType = 0x00;
pub const kIOHIDQueueOptionsTypeEnqueueAll: IOHIDQueueOptionsType = 0x01;

// IOHIDElementFlags
pub type IOHIDElementFlags = u32;
pub const kIOHIDElementFlagsConstantMask:     IOHIDElementFlags = 0x0001;
pub const kIOHIDElementFlagsVariableMask:     IOHIDElementFlags = 0x0002;
pub const kIOHIDElementFlagsRelativeMask:     IOHIDElementFlags = 0x0004;
pub const kIOHIDElementFlagsWrapMask:         IOHIDElementFlags = 0x0008;
pub const kIOHIDElementFlagsNonLinearMask:    IOHIDElementFlags = 0x0010;
pub const kIOHIDElementFlagsNoPreferredMask:  IOHIDElementFlags = 0x0020;
pub const kIOHIDElementFlagsNullStateMask:    IOHIDElementFlags = 0x0040;
pub const kIOHIDElementFlagsVolativeMask:     IOHIDElementFlags = 0x0080;
pub const kIOHIDElementFlagsBufferedByteMask: IOHIDElementFlags = 0x0100;

// IOHIDStandardType
pub type IOHIDStandardType = u32;
pub const kIOHIDStandardTypeANSI: IOHIDStandardType = 0;
pub const kIOHIDStandardTypeISO:  IOHIDStandardType = 1;
pub const kIOHIDStandardTypeJIS:  IOHIDStandardType = 2;

// IOHIDValueScaleType
pub type IOHIDValueScaleType = u32;
pub const kIOHIDValueScaleTypeCalibrated: IOHIDValueScaleType = 0;
pub const kIOHIDValueScaleTypePhysical:   IOHIDValueScaleType = 1;

// IOHIDValueOptions
pub type IOHIDValueOptions = u32;
pub const kIOHIDValueOptionsFlagRelativeSimple: IOHIDValueOptions = (1<<0);
pub const kIOHIDValueOptionsFlagPrevious:       IOHIDValueOptions = (1<<1);

// kIOHIDDigitizerGestureCharacterStateKey
pub fn kIOHIDDigitizerGestureCharacterStateKey() -> *const c_char {
    b"DigitizerCharacterGestureState\0".as_ptr() as *const c_char
}
