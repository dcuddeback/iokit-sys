// exports from <IOKit/hid/IOHIDUsageTables.h>

use libc::c_int;

// Usage Pages
pub const kHIDPage_Undefined:              c_int = 0x00;
pub const kHIDPage_GenericDesktop:         c_int = 0x01;
pub const kHIDPage_Simulation:             c_int = 0x02;
pub const kHIDPage_VR:                     c_int = 0x03;
pub const kHIDPage_Sport:                  c_int = 0x04;
pub const kHIDPage_Game:                   c_int = 0x05;
pub const kHIDPage_GenericDeviceControls:  c_int = 0x06;
pub const kHIDPage_KeyboardOrKeypad:       c_int = 0x07;
pub const kHIDPage_LEDs:                   c_int = 0x08;
pub const kHIDPage_Button:                 c_int = 0x09;
pub const kHIDPage_Ordinal:                c_int = 0x0A;
pub const kHIDPage_Telephony:              c_int = 0x0B;
pub const kHIDPage_Consumer:               c_int = 0x0C;
pub const kHIDPage_Digitizer:              c_int = 0x0D;
// Reserved 0x0E
pub const kHIDPage_PID:                    c_int = 0x0F;
pub const kHIDPage_Unicode:                c_int = 0x10;
// Reserved 0x11 - 0x13
pub const kHIDPage_AlphanumericDisplay:    c_int = 0x14;
// Reserved 0x15 - 0x1F
pub const kHIDPage_Sensor:                 c_int = 0x20;
// Reserved 0x21 - 0x7f
pub const kHIDPage_Monitor:                c_int = 0x80;
pub const kHIDPage_MonitorEnumerated:      c_int = 0x81;
pub const kHIDPage_MonitorVirtual:         c_int = 0x82;
pub const kHIDPage_MonitorReserved:        c_int = 0x83;
// Power 0x84 - 0x87     USB Device Class Definition for Power Devices
pub const kHIDPage_PowerDevice:            c_int = 0x84;
pub const kHIDPage_BatterySystem:          c_int = 0x85;
pub const kHIDPage_PowerReserved:          c_int = 0x86;
pub const kHIDPage_PowerReserved2:         c_int = 0x87;
// Reserved 0x88 - 0x8B
pub const kHIDPage_BarCodeScanner:         c_int = 0x8C;
pub const kHIDPage_WeighingDevice:         c_int = 0x8D;
pub const kHIDPage_Scale:                  c_int = 0x8D;
pub const kHIDPage_MagneticStripeReader:   c_int = 0x8E;
// ReservedPointofSalepages 0x8F
pub const kHIDPage_CameraControl:          c_int = 0x90;
pub const kHIDPage_Arcade:                 c_int = 0x91;
// Reserved 0x92 - 0xFEFF
// VendorDefined 0xFF00 - 0xFFFF
pub const kHIDPage_VendorDefinedStart:     c_int = 0xFF00;

// Undefined Usage for all usage pages
pub const kHIDUsage_Undefined:    c_int = 0x00;

// GenericDesktop Page (0x01)
pub const kHIDUsage_GD_Pointer:    c_int = 0x01;
pub const kHIDUsage_GD_Mouse:    c_int = 0x02;
// 0x03 Reserved
pub const kHIDUsage_GD_Joystick:    c_int = 0x04;
pub const kHIDUsage_GD_GamePad:    c_int = 0x05;
pub const kHIDUsage_GD_Keyboard:    c_int = 0x06;
pub const kHIDUsage_GD_Keypad:    c_int = 0x07;
pub const kHIDUsage_GD_MultiAxisController:    c_int = 0x08;
// 0x09 - 0x2F Reserved
pub const kHIDUsage_GD_X:    c_int = 0x30;
pub const kHIDUsage_GD_Y:    c_int = 0x31;
pub const kHIDUsage_GD_Z:    c_int = 0x32;
pub const kHIDUsage_GD_Rx:    c_int = 0x33;
pub const kHIDUsage_GD_Ry:    c_int = 0x34;
pub const kHIDUsage_GD_Rz:    c_int = 0x35;
pub const kHIDUsage_GD_Slider:    c_int = 0x36;
pub const kHIDUsage_GD_Dial:    c_int = 0x37;
pub const kHIDUsage_GD_Wheel:    c_int = 0x38;
pub const kHIDUsage_GD_Hatswitch:    c_int = 0x39;
pub const kHIDUsage_GD_CountedBuffer:    c_int = 0x3A;
pub const kHIDUsage_GD_ByteCount:    c_int = 0x3B;
pub const kHIDUsage_GD_MotionWakeup:    c_int = 0x3C;
pub const kHIDUsage_GD_Start:    c_int = 0x3D;
pub const kHIDUsage_GD_Select:    c_int = 0x3E;
// 0x3F Reserved
pub const kHIDUsage_GD_Vx:    c_int = 0x40;
pub const kHIDUsage_GD_Vy:    c_int = 0x41;
pub const kHIDUsage_GD_Vz:    c_int = 0x42;
pub const kHIDUsage_GD_Vbrx:    c_int = 0x43;
pub const kHIDUsage_GD_Vbry:    c_int = 0x44;
pub const kHIDUsage_GD_Vbrz:    c_int = 0x45;
pub const kHIDUsage_GD_Vno:    c_int = 0x46;
// 0x47 - 0x7F Reserved
pub const kHIDUsage_GD_SystemControl:    c_int = 0x80;
pub const kHIDUsage_GD_SystemPowerDown:    c_int = 0x81;
pub const kHIDUsage_GD_SystemSleep:    c_int = 0x82;
pub const kHIDUsage_GD_SystemWakeUp:    c_int = 0x83;
pub const kHIDUsage_GD_SystemContextMenu:    c_int = 0x84;
pub const kHIDUsage_GD_SystemMainMenu:    c_int = 0x85;
pub const kHIDUsage_GD_SystemAppMenu:    c_int = 0x86;
pub const kHIDUsage_GD_SystemMenuHelp:    c_int = 0x87;
pub const kHIDUsage_GD_SystemMenuExit:    c_int = 0x88;
pub const kHIDUsage_GD_SystemMenuSelect:    c_int = 0x89;
pub const kHIDUsage_GD_SystemMenu:    c_int = kHIDUsage_GD_SystemMenuSelect;
pub const kHIDUsage_GD_SystemMenuRight:    c_int = 0x8A;
pub const kHIDUsage_GD_SystemMenuLeft:    c_int = 0x8B;
pub const kHIDUsage_GD_SystemMenuUp:    c_int = 0x8C;
pub const kHIDUsage_GD_SystemMenuDown:    c_int = 0x8D;
// 0x8E - 0x8F Reserved
pub const kHIDUsage_GD_DPadUp:    c_int = 0x90;
pub const kHIDUsage_GD_DPadDown:    c_int = 0x91;
pub const kHIDUsage_GD_DPadRight:    c_int = 0x92;
pub const kHIDUsage_GD_DPadLeft:    c_int = 0x93;
// 0x94 - 0xFFFF Reserved
pub const kHIDUsage_GD_Reserved: c_int = 0xFFFF;

// Simulation Page (0x02)
pub const kHIDUsage_Sim_FlightSimulationDevice:    c_int = 0x01;
pub const kHIDUsage_Sim_AutomobileSimulationDevice:    c_int = 0x02;
pub const kHIDUsage_Sim_TankSimulationDevice:    c_int = 0x03;
pub const kHIDUsage_Sim_SpaceshipSimulationDevice:    c_int = 0x04;
pub const kHIDUsage_Sim_SubmarineSimulationDevice:    c_int = 0x05;
pub const kHIDUsage_Sim_SailingSimulationDevice:    c_int = 0x06;
pub const kHIDUsage_Sim_MotorcycleSimulationDevice:    c_int = 0x07;
pub const kHIDUsage_Sim_SportsSimulationDevice:    c_int = 0x08;
pub const kHIDUsage_Sim_AirplaneSimulationDevice:    c_int = 0x09;
pub const kHIDUsage_Sim_HelicopterSimulationDevice:    c_int = 0x0A;
pub const kHIDUsage_Sim_MagicCarpetSimulationDevice:    c_int = 0x0B;
pub const kHIDUsage_Sim_BicycleSimulationDevice:    c_int = 0x0C;
// 0x0D - 0x1F Reserved
pub const kHIDUsage_Sim_FlightControlStick:    c_int = 0x20;
pub const kHIDUsage_Sim_FlightStick:    c_int = 0x21;
pub const kHIDUsage_Sim_CyclicControl:    c_int = 0x22;
pub const kHIDUsage_Sim_CyclicTrim:    c_int = 0x23;
pub const kHIDUsage_Sim_FlightYoke:    c_int = 0x24;
pub const kHIDUsage_Sim_TrackControl:    c_int = 0x25;
// 0x26 - 0xAF Reserved
pub const kHIDUsage_Sim_Aileron:    c_int = 0xB0;
pub const kHIDUsage_Sim_AileronTrim:    c_int = 0xB1;
pub const kHIDUsage_Sim_AntiTorqueControl:    c_int = 0xB2;
pub const kHIDUsage_Sim_AutopilotEnable:    c_int = 0xB3;
pub const kHIDUsage_Sim_ChaffRelease:    c_int = 0xB4;
pub const kHIDUsage_Sim_CollectiveControl:    c_int = 0xB5;
pub const kHIDUsage_Sim_DiveBrake:    c_int = 0xB6;
pub const kHIDUsage_Sim_ElectronicCountermeasures:    c_int = 0xB7;
pub const kHIDUsage_Sim_Elevator:    c_int = 0xB8;
pub const kHIDUsage_Sim_ElevatorTrim:    c_int = 0xB9;
pub const kHIDUsage_Sim_Rudder:    c_int = 0xBA;
pub const kHIDUsage_Sim_Throttle:    c_int = 0xBB;
pub const kHIDUsage_Sim_FlightCommunications:    c_int = 0xBC;
pub const kHIDUsage_Sim_FlareRelease:    c_int = 0xBD;
pub const kHIDUsage_Sim_LandingGear:    c_int = 0xBE;
pub const kHIDUsage_Sim_ToeBrake:    c_int = 0xBF;
pub const kHIDUsage_Sim_Trigger:    c_int = 0xC0;
pub const kHIDUsage_Sim_WeaponsArm:    c_int = 0xC1;
pub const kHIDUsage_Sim_Weapons:    c_int = 0xC2;
pub const kHIDUsage_Sim_WingFlaps:    c_int = 0xC3;
pub const kHIDUsage_Sim_Accelerator:    c_int = 0xC4;
pub const kHIDUsage_Sim_Brake:    c_int = 0xC5;
pub const kHIDUsage_Sim_Clutch:    c_int = 0xC6;
pub const kHIDUsage_Sim_Shifter:    c_int = 0xC7;
pub const kHIDUsage_Sim_Steering:    c_int = 0xC8;
pub const kHIDUsage_Sim_TurretDirection:    c_int = 0xC9;
pub const kHIDUsage_Sim_BarrelElevation:    c_int = 0xCA;
pub const kHIDUsage_Sim_DivePlane:    c_int = 0xCB;
pub const kHIDUsage_Sim_Ballast:    c_int = 0xCC;
pub const kHIDUsage_Sim_BicycleCrank:    c_int = 0xCD;
pub const kHIDUsage_Sim_HandleBars:    c_int = 0xCE;
pub const kHIDUsage_Sim_FrontBrake:    c_int = 0xCF;
pub const kHIDUsage_Sim_RearBrake:    c_int = 0xD0;
// 0xD1 - 0xFFFF Reserved
pub const kHIDUsage_Sim_Reserved: c_int = 0xFFFF;

// VR Page (0x03)
pub const kHIDUsage_VR_Belt:    c_int = 0x01;
pub const kHIDUsage_VR_BodySuit:    c_int = 0x02;
pub const kHIDUsage_VR_Flexor:    c_int = 0x03;
pub const kHIDUsage_VR_Glove:    c_int = 0x04;
pub const kHIDUsage_VR_HeadTracker:    c_int = 0x05;
pub const kHIDUsage_VR_HeadMountedDisplay:    c_int = 0x06;
pub const kHIDUsage_VR_HandTracker:    c_int = 0x07;
pub const kHIDUsage_VR_Oculometer:    c_int = 0x08;
pub const kHIDUsage_VR_Vest:    c_int = 0x09;
pub const kHIDUsage_VR_AnimatronicDevice:    c_int = 0x0A;
// 0x0B - 0x1F Reserved
pub const kHIDUsage_VR_StereoEnable:    c_int = 0x20;
pub const kHIDUsage_VR_DisplayEnable:    c_int = 0x21;
// 0x22 - 0xFFFF Reserved
pub const kHIDUsage_VR_Reserved: c_int = 0xFFFF;

// Sport Page (0x04)
pub const kHIDUsage_Sprt_BaseballBat:    c_int = 0x01;
pub const kHIDUsage_Sprt_GolfClub:    c_int = 0x02;
pub const kHIDUsage_Sprt_RowingMachine:    c_int = 0x03;
pub const kHIDUsage_Sprt_Treadmill:    c_int = 0x04;
// 0x05 - 0x2F Reserved
pub const kHIDUsage_Sprt_Oar:    c_int = 0x30;
pub const kHIDUsage_Sprt_Slope:    c_int = 0x31;
pub const kHIDUsage_Sprt_Rate:    c_int = 0x32;
pub const kHIDUsage_Sprt_StickSpeed:    c_int = 0x33;
pub const kHIDUsage_Sprt_StickFaceAngle:    c_int = 0x34;
pub const kHIDUsage_Sprt_StickHeelOrToe:    c_int = 0x35;
pub const kHIDUsage_Sprt_StickFollowThrough:    c_int = 0x36;
pub const kHIDUsage_Sprt_StickTempo:    c_int = 0x37;
pub const kHIDUsage_Sprt_StickType:    c_int = 0x38;
pub const kHIDUsage_Sprt_StickHeight:    c_int = 0x39;
// 0x3A - 0x4F Reserved
pub const kHIDUsage_Sprt_Putter:    c_int = 0x50;
pub const kHIDUsage_Sprt_1Iron:    c_int = 0x51;
pub const kHIDUsage_Sprt_2Iron:    c_int = 0x52;
pub const kHIDUsage_Sprt_3Iron:    c_int = 0x53;
pub const kHIDUsage_Sprt_4Iron:    c_int = 0x54;
pub const kHIDUsage_Sprt_5Iron:    c_int = 0x55;
pub const kHIDUsage_Sprt_6Iron:    c_int = 0x56;
pub const kHIDUsage_Sprt_7Iron:    c_int = 0x57;
pub const kHIDUsage_Sprt_8Iron:    c_int = 0x58;
pub const kHIDUsage_Sprt_9Iron:    c_int = 0x59;
pub const kHIDUsage_Sprt_10Iron:    c_int = 0x5A;
pub const kHIDUsage_Sprt_11Iron:    c_int = 0x5B;
pub const kHIDUsage_Sprt_SandWedge:    c_int = 0x5C;
pub const kHIDUsage_Sprt_LoftWedge:    c_int = 0x5D;
pub const kHIDUsage_Sprt_PowerWedge:    c_int = 0x5E;
pub const kHIDUsage_Sprt_1Wood:    c_int = 0x5F;
pub const kHIDUsage_Sprt_3Wood:    c_int = 0x60;
pub const kHIDUsage_Sprt_5Wood:    c_int = 0x61;
pub const kHIDUsage_Sprt_7Wood:    c_int = 0x62;
pub const kHIDUsage_Sprt_9Wood:    c_int = 0x63;
// 0x64 - 0xFFFF Reserved
pub const kHIDUsage_Sprt_Reserved: c_int = 0xFFFF;

// Game Page (0x05)
pub const kHIDUsage_Game_3DGameController:    c_int = 0x01;
pub const kHIDUsage_Game_PinballDevice:    c_int = 0x02;
pub const kHIDUsage_Game_GunDevice:    c_int = 0x03;
// 0x04 - 0x1F Reserved
pub const kHIDUsage_Game_PointofView:    c_int = 0x20;
pub const kHIDUsage_Game_TurnRightOrLeft:    c_int = 0x21;
pub const kHIDUsage_Game_PitchUpOrDown:    c_int = 0x22;
pub const kHIDUsage_Game_RollRightOrLeft:    c_int = 0x23;
pub const kHIDUsage_Game_MoveRightOrLeft:    c_int = 0x24;
pub const kHIDUsage_Game_MoveForwardOrBackward:    c_int = 0x25;
pub const kHIDUsage_Game_MoveUpOrDown:    c_int = 0x26;
pub const kHIDUsage_Game_LeanRightOrLeft:    c_int = 0x27;
pub const kHIDUsage_Game_LeanForwardOrBackward:    c_int = 0x28;
pub const kHIDUsage_Game_HeightOfPOV:    c_int = 0x29;
pub const kHIDUsage_Game_Flipper:    c_int = 0x2A;
pub const kHIDUsage_Game_SecondaryFlipper:    c_int = 0x2B;
pub const kHIDUsage_Game_Bump:    c_int = 0x2C;
pub const kHIDUsage_Game_NewGame:    c_int = 0x2D;
pub const kHIDUsage_Game_ShootBall:    c_int = 0x2E;
pub const kHIDUsage_Game_Player:    c_int = 0x2F;
pub const kHIDUsage_Game_GunBolt:    c_int = 0x30;
pub const kHIDUsage_Game_GunClip:    c_int = 0x31;
pub const kHIDUsage_Game_Gun:    c_int = 0x32;
pub const kHIDUsage_Game_GunSingleShot:    c_int = 0x33;
pub const kHIDUsage_Game_GunBurst:    c_int = 0x34;
pub const kHIDUsage_Game_GunAutomatic:    c_int = 0x35;
pub const kHIDUsage_Game_GunSafety:    c_int = 0x36;
pub const kHIDUsage_Game_GamepadFireOrJump:    c_int = 0x37;
pub const kHIDUsage_Game_GamepadTrigger:    c_int = 0x39;
// 0x3A - 0xFFFF Reserved
pub const kHIDUsage_Game_Reserved: c_int = 0xFFFF;

// Generic Device Controls (0x0g)
pub const kHIDUsage_GenDevControls_BackgroundControls:     c_int = 0x01;

// KeyboardOrKeypad Page (0x07)
pub const kHIDUsage_KeyboardErrorRollOver:    c_int = 0x01;
pub const kHIDUsage_KeyboardPOSTFail:    c_int = 0x02;
pub const kHIDUsage_KeyboardErrorUndefined:    c_int = 0x03;
pub const kHIDUsage_KeyboardA:    c_int = 0x04;
pub const kHIDUsage_KeyboardB:    c_int = 0x05;
pub const kHIDUsage_KeyboardC:    c_int = 0x06;
pub const kHIDUsage_KeyboardD:    c_int = 0x07;
pub const kHIDUsage_KeyboardE:    c_int = 0x08;
pub const kHIDUsage_KeyboardF:    c_int = 0x09;
pub const kHIDUsage_KeyboardG:    c_int = 0x0A;
pub const kHIDUsage_KeyboardH:    c_int = 0x0B;
pub const kHIDUsage_KeyboardI:    c_int = 0x0C;
pub const kHIDUsage_KeyboardJ:    c_int = 0x0D;
pub const kHIDUsage_KeyboardK:    c_int = 0x0E;
pub const kHIDUsage_KeyboardL:    c_int = 0x0F;
pub const kHIDUsage_KeyboardM:    c_int = 0x10;
pub const kHIDUsage_KeyboardN:    c_int = 0x11;
pub const kHIDUsage_KeyboardO:    c_int = 0x12;
pub const kHIDUsage_KeyboardP:    c_int = 0x13;
pub const kHIDUsage_KeyboardQ:    c_int = 0x14;
pub const kHIDUsage_KeyboardR:    c_int = 0x15;
pub const kHIDUsage_KeyboardS:    c_int = 0x16;
pub const kHIDUsage_KeyboardT:    c_int = 0x17;
pub const kHIDUsage_KeyboardU:    c_int = 0x18;
pub const kHIDUsage_KeyboardV:    c_int = 0x19;
pub const kHIDUsage_KeyboardW:    c_int = 0x1A;
pub const kHIDUsage_KeyboardX:    c_int = 0x1B;
pub const kHIDUsage_KeyboardY:    c_int = 0x1C;
pub const kHIDUsage_KeyboardZ:    c_int = 0x1D;
pub const kHIDUsage_Keyboard1:    c_int = 0x1E;
pub const kHIDUsage_Keyboard2:    c_int = 0x1F;
pub const kHIDUsage_Keyboard3:    c_int = 0x20;
pub const kHIDUsage_Keyboard4:    c_int = 0x21;
pub const kHIDUsage_Keyboard5:    c_int = 0x22;
pub const kHIDUsage_Keyboard6:    c_int = 0x23;
pub const kHIDUsage_Keyboard7:    c_int = 0x24;
pub const kHIDUsage_Keyboard8:    c_int = 0x25;
pub const kHIDUsage_Keyboard9:    c_int = 0x26;
pub const kHIDUsage_Keyboard0:    c_int = 0x27;
pub const kHIDUsage_KeyboardReturnOrEnter:    c_int = 0x28;
pub const kHIDUsage_KeyboardEscape:    c_int = 0x29;
pub const kHIDUsage_KeyboardDeleteOrBackspace:    c_int = 0x2A;
pub const kHIDUsage_KeyboardTab:    c_int = 0x2B;
pub const kHIDUsage_KeyboardSpacebar:    c_int = 0x2C;
pub const kHIDUsage_KeyboardHyphen:    c_int = 0x2D;
pub const kHIDUsage_KeyboardEqualSign:    c_int = 0x2E;
pub const kHIDUsage_KeyboardOpenBracket:    c_int = 0x2F;
pub const kHIDUsage_KeyboardCloseBracket:    c_int = 0x30;
pub const kHIDUsage_KeyboardBackslash:    c_int = 0x31;
pub const kHIDUsage_KeyboardNonUSPound:    c_int = 0x32;
pub const kHIDUsage_KeyboardSemicolon:    c_int = 0x33;
pub const kHIDUsage_KeyboardQuote:    c_int = 0x34;
pub const kHIDUsage_KeyboardGraveAccentAndTilde:    c_int = 0x35;
pub const kHIDUsage_KeyboardComma:    c_int = 0x36;
pub const kHIDUsage_KeyboardPeriod:    c_int = 0x37;
pub const kHIDUsage_KeyboardSlash:    c_int = 0x38;
pub const kHIDUsage_KeyboardCapsLock:    c_int = 0x39;
pub const kHIDUsage_KeyboardF1:    c_int = 0x3A;
pub const kHIDUsage_KeyboardF2:    c_int = 0x3B;
pub const kHIDUsage_KeyboardF3:    c_int = 0x3C;
pub const kHIDUsage_KeyboardF4:    c_int = 0x3D;
pub const kHIDUsage_KeyboardF5:    c_int = 0x3E;
pub const kHIDUsage_KeyboardF6:    c_int = 0x3F;
pub const kHIDUsage_KeyboardF7:    c_int = 0x40;
pub const kHIDUsage_KeyboardF8:    c_int = 0x41;
pub const kHIDUsage_KeyboardF9:    c_int = 0x42;
pub const kHIDUsage_KeyboardF10:    c_int = 0x43;
pub const kHIDUsage_KeyboardF11:    c_int = 0x44;
pub const kHIDUsage_KeyboardF12:    c_int = 0x45;
pub const kHIDUsage_KeyboardPrintScreen:    c_int = 0x46;
pub const kHIDUsage_KeyboardScrollLock:    c_int = 0x47;
pub const kHIDUsage_KeyboardPause:    c_int = 0x48;
pub const kHIDUsage_KeyboardInsert:    c_int = 0x49;
pub const kHIDUsage_KeyboardHome:    c_int = 0x4A;
pub const kHIDUsage_KeyboardPageUp:    c_int = 0x4B;
pub const kHIDUsage_KeyboardDeleteForward:    c_int = 0x4C;
pub const kHIDUsage_KeyboardEnd:    c_int = 0x4D;
pub const kHIDUsage_KeyboardPageDown:    c_int = 0x4E;
pub const kHIDUsage_KeyboardRightArrow:    c_int = 0x4F;
pub const kHIDUsage_KeyboardLeftArrow:    c_int = 0x50;
pub const kHIDUsage_KeyboardDownArrow:    c_int = 0x51;
pub const kHIDUsage_KeyboardUpArrow:    c_int = 0x52;
pub const kHIDUsage_KeypadNumLock:    c_int = 0x53;
pub const kHIDUsage_KeypadSlash:    c_int = 0x54;
pub const kHIDUsage_KeypadAsterisk:    c_int = 0x55;
pub const kHIDUsage_KeypadHyphen:    c_int = 0x56;
pub const kHIDUsage_KeypadPlus:    c_int = 0x57;
pub const kHIDUsage_KeypadEnter:    c_int = 0x58;
pub const kHIDUsage_Keypad1:    c_int = 0x59;
pub const kHIDUsage_Keypad2:    c_int = 0x5A;
pub const kHIDUsage_Keypad3:    c_int = 0x5B;
pub const kHIDUsage_Keypad4:    c_int = 0x5C;
pub const kHIDUsage_Keypad5:    c_int = 0x5D;
pub const kHIDUsage_Keypad6:    c_int = 0x5E;
pub const kHIDUsage_Keypad7:    c_int = 0x5F;
pub const kHIDUsage_Keypad8:    c_int = 0x60;
pub const kHIDUsage_Keypad9:    c_int = 0x61;
pub const kHIDUsage_Keypad0:    c_int = 0x62;
pub const kHIDUsage_KeypadPeriod:    c_int = 0x63;
pub const kHIDUsage_KeyboardNonUSBackslash:    c_int = 0x64;
pub const kHIDUsage_KeyboardApplication:    c_int = 0x65;
pub const kHIDUsage_KeyboardPower:    c_int = 0x66;
pub const kHIDUsage_KeypadEqualSign:    c_int = 0x67;
pub const kHIDUsage_KeyboardF13:    c_int = 0x68;
pub const kHIDUsage_KeyboardF14:    c_int = 0x69;
pub const kHIDUsage_KeyboardF15:    c_int = 0x6A;
pub const kHIDUsage_KeyboardF16:    c_int = 0x6B;
pub const kHIDUsage_KeyboardF17:    c_int = 0x6C;
pub const kHIDUsage_KeyboardF18:    c_int = 0x6D;
pub const kHIDUsage_KeyboardF19:    c_int = 0x6E;
pub const kHIDUsage_KeyboardF20:    c_int = 0x6F;
pub const kHIDUsage_KeyboardF21:    c_int = 0x70;
pub const kHIDUsage_KeyboardF22:    c_int = 0x71;
pub const kHIDUsage_KeyboardF23:    c_int = 0x72;
pub const kHIDUsage_KeyboardF24:    c_int = 0x73;
pub const kHIDUsage_KeyboardExecute:    c_int = 0x74;
pub const kHIDUsage_KeyboardHelp:    c_int = 0x75;
pub const kHIDUsage_KeyboardMenu:    c_int = 0x76;
pub const kHIDUsage_KeyboardSelect:    c_int = 0x77;
pub const kHIDUsage_KeyboardStop:    c_int = 0x78;
pub const kHIDUsage_KeyboardAgain:    c_int = 0x79;
pub const kHIDUsage_KeyboardUndo:    c_int = 0x7A;
pub const kHIDUsage_KeyboardCut:    c_int = 0x7B;
pub const kHIDUsage_KeyboardCopy:    c_int = 0x7C;
pub const kHIDUsage_KeyboardPaste:    c_int = 0x7D;
pub const kHIDUsage_KeyboardFind:    c_int = 0x7E;
pub const kHIDUsage_KeyboardMute:    c_int = 0x7F;
pub const kHIDUsage_KeyboardVolumeUp:    c_int = 0x80;
pub const kHIDUsage_KeyboardVolumeDown:    c_int = 0x81;
pub const kHIDUsage_KeyboardLockingCapsLock:    c_int = 0x82;
pub const kHIDUsage_KeyboardLockingNumLock:    c_int = 0x83;
pub const kHIDUsage_KeyboardLockingScrollLock:    c_int = 0x84;
pub const kHIDUsage_KeypadComma:    c_int = 0x85;
pub const kHIDUsage_KeypadEqualSignAS400:    c_int = 0x86;
pub const kHIDUsage_KeyboardInternational1:    c_int = 0x87;
pub const kHIDUsage_KeyboardInternational2:    c_int = 0x88;
pub const kHIDUsage_KeyboardInternational3:    c_int = 0x89;
pub const kHIDUsage_KeyboardInternational4:    c_int = 0x8A;
pub const kHIDUsage_KeyboardInternational5:    c_int = 0x8B;
pub const kHIDUsage_KeyboardInternational6:    c_int = 0x8C;
pub const kHIDUsage_KeyboardInternational7:    c_int = 0x8D;
pub const kHIDUsage_KeyboardInternational8:    c_int = 0x8E;
pub const kHIDUsage_KeyboardInternational9:    c_int = 0x8F;
pub const kHIDUsage_KeyboardLANG1:    c_int = 0x90;
pub const kHIDUsage_KeyboardLANG2:    c_int = 0x91;
pub const kHIDUsage_KeyboardLANG3:    c_int = 0x92;
pub const kHIDUsage_KeyboardLANG4:    c_int = 0x93;
pub const kHIDUsage_KeyboardLANG5:    c_int = 0x94;
pub const kHIDUsage_KeyboardLANG6:    c_int = 0x95;
pub const kHIDUsage_KeyboardLANG7:    c_int = 0x96;
pub const kHIDUsage_KeyboardLANG8:    c_int = 0x97;
pub const kHIDUsage_KeyboardLANG9:    c_int = 0x98;
pub const kHIDUsage_KeyboardAlternateErase:    c_int = 0x99;
pub const kHIDUsage_KeyboardSysReqOrAttention:    c_int = 0x9A;
pub const kHIDUsage_KeyboardCancel:    c_int = 0x9B;
pub const kHIDUsage_KeyboardClear:    c_int = 0x9C;
pub const kHIDUsage_KeyboardPrior:    c_int = 0x9D;
pub const kHIDUsage_KeyboardReturn:    c_int = 0x9E;
pub const kHIDUsage_KeyboardSeparator:    c_int = 0x9F;
pub const kHIDUsage_KeyboardOut:    c_int = 0xA0;
pub const kHIDUsage_KeyboardOper:    c_int = 0xA1;
pub const kHIDUsage_KeyboardClearOrAgain:    c_int = 0xA2;
pub const kHIDUsage_KeyboardCrSelOrProps:    c_int = 0xA3;
pub const kHIDUsage_KeyboardExSel:    c_int = 0xA4;
// 0xA5-0xDF Reserved
pub const kHIDUsage_KeyboardLeftControl:    c_int = 0xE0;
pub const kHIDUsage_KeyboardLeftShift:    c_int = 0xE1;
pub const kHIDUsage_KeyboardLeftAlt:    c_int = 0xE2;
pub const kHIDUsage_KeyboardLeftGUI:    c_int = 0xE3;
pub const kHIDUsage_KeyboardRightControl:    c_int = 0xE4;
pub const kHIDUsage_KeyboardRightShift:    c_int = 0xE5;
pub const kHIDUsage_KeyboardRightAlt:    c_int = 0xE6;
pub const kHIDUsage_KeyboardRightGUI:    c_int = 0xE7;
// 0xE8-0xFFFF Reserved
pub const kHIDUsage_Keyboard_Reserved: c_int = 0xFFFF;

// LEDs Page (0x08)
pub const kHIDUsage_LED_NumLock:    c_int = 0x01;
pub const kHIDUsage_LED_CapsLock:    c_int = 0x02;
pub const kHIDUsage_LED_ScrollLock:    c_int = 0x03;
pub const kHIDUsage_LED_Compose:    c_int = 0x04;
pub const kHIDUsage_LED_Kana:    c_int = 0x05;
pub const kHIDUsage_LED_Power:    c_int = 0x06;
pub const kHIDUsage_LED_Shift:    c_int = 0x07;
pub const kHIDUsage_LED_DoNotDisturb:    c_int = 0x08;
pub const kHIDUsage_LED_Mute:    c_int = 0x09;
pub const kHIDUsage_LED_ToneEnable:    c_int = 0x0A;
pub const kHIDUsage_LED_HighCutFilter:    c_int = 0x0B;
pub const kHIDUsage_LED_LowCutFilter:    c_int = 0x0C;
pub const kHIDUsage_LED_EqualizerEnable:    c_int = 0x0D;
pub const kHIDUsage_LED_SoundFieldOn:    c_int = 0x0E;
pub const kHIDUsage_LED_SurroundOn:    c_int = 0x0F;
pub const kHIDUsage_LED_Repeat:    c_int = 0x10;
pub const kHIDUsage_LED_Stereo:    c_int = 0x11;
pub const kHIDUsage_LED_SamplingRateDetect:    c_int = 0x12;
pub const kHIDUsage_LED_Spinning:    c_int = 0x13;
pub const kHIDUsage_LED_CAV:    c_int = 0x14;
pub const kHIDUsage_LED_CLV:    c_int = 0x15;
pub const kHIDUsage_LED_RecordingFormatDetect:    c_int = 0x16;
pub const kHIDUsage_LED_OffHook:    c_int = 0x17;
pub const kHIDUsage_LED_Ring:    c_int = 0x18;
pub const kHIDUsage_LED_MessageWaiting:    c_int = 0x19;
pub const kHIDUsage_LED_DataMode:    c_int = 0x1A;
pub const kHIDUsage_LED_BatteryOperation:    c_int = 0x1B;
pub const kHIDUsage_LED_BatteryOK:    c_int = 0x1C;
pub const kHIDUsage_LED_BatteryLow:    c_int = 0x1D;
pub const kHIDUsage_LED_Speaker:    c_int = 0x1E;
pub const kHIDUsage_LED_HeadSet:    c_int = 0x1F;
pub const kHIDUsage_LED_Hold:    c_int = 0x20;
pub const kHIDUsage_LED_Microphone:    c_int = 0x21;
pub const kHIDUsage_LED_Coverage:    c_int = 0x22;
pub const kHIDUsage_LED_NightMode:    c_int = 0x23;
pub const kHIDUsage_LED_SendCalls:    c_int = 0x24;
pub const kHIDUsage_LED_CallPickup:    c_int = 0x25;
pub const kHIDUsage_LED_Conference:    c_int = 0x26;
pub const kHIDUsage_LED_StandBy:    c_int = 0x27;
pub const kHIDUsage_LED_CameraOn:    c_int = 0x28;
pub const kHIDUsage_LED_CameraOff:    c_int = 0x29;
pub const kHIDUsage_LED_OnLine:    c_int = 0x2A;
pub const kHIDUsage_LED_OffLine:    c_int = 0x2B;
pub const kHIDUsage_LED_Busy:    c_int = 0x2C;
pub const kHIDUsage_LED_Ready:    c_int = 0x2D;
pub const kHIDUsage_LED_PaperOut:    c_int = 0x2E;
pub const kHIDUsage_LED_PaperJam:    c_int = 0x2F;
pub const kHIDUsage_LED_Remote:    c_int = 0x30;
pub const kHIDUsage_LED_Forward:    c_int = 0x31;
pub const kHIDUsage_LED_Reverse:    c_int = 0x32;
pub const kHIDUsage_LED_Stop:    c_int = 0x33;
pub const kHIDUsage_LED_Rewind:    c_int = 0x34;
pub const kHIDUsage_LED_FastForward:    c_int = 0x35;
pub const kHIDUsage_LED_Play:    c_int = 0x36;
pub const kHIDUsage_LED_Pause:    c_int = 0x37;
pub const kHIDUsage_LED_Record:    c_int = 0x38;
pub const kHIDUsage_LED_Error:    c_int = 0x39;
pub const kHIDUsage_LED_Usage:    c_int = 0x3A;
pub const kHIDUsage_LED_UsageInUseIndicator:    c_int = 0x3B;
pub const kHIDUsage_LED_UsageMultiModeIndicator:    c_int = 0x3C;
pub const kHIDUsage_LED_IndicatorOn:    c_int = 0x3D;
pub const kHIDUsage_LED_IndicatorFlash:    c_int = 0x3E;
pub const kHIDUsage_LED_IndicatorSlowBlink:    c_int = 0x3F;
pub const kHIDUsage_LED_IndicatorFastBlink:    c_int = 0x40;
pub const kHIDUsage_LED_IndicatorOff:    c_int = 0x41;
pub const kHIDUsage_LED_FlashOnTime:    c_int = 0x42;
pub const kHIDUsage_LED_SlowBlinkOnTime:    c_int = 0x43;
pub const kHIDUsage_LED_SlowBlinkOffTime:    c_int = 0x44;
pub const kHIDUsage_LED_FastBlinkOnTime:    c_int = 0x45;
pub const kHIDUsage_LED_FastBlinkOffTime:    c_int = 0x46;
pub const kHIDUsage_LED_UsageIndicatorColor:    c_int = 0x47;
pub const kHIDUsage_LED_IndicatorRed:    c_int = 0x48;
pub const kHIDUsage_LED_IndicatorGreen:    c_int = 0x49;
pub const kHIDUsage_LED_IndicatorAmber:    c_int = 0x4A;
pub const kHIDUsage_LED_GenericIndicator:    c_int = 0x4B;
pub const kHIDUsage_LED_SystemSuspend:    c_int = 0x4C;
pub const kHIDUsage_LED_ExternalPowerConnected:    c_int = 0x4D;
// 0x4E - 0xFFFF Reserved
pub const kHIDUsage_LED_Reserved: c_int = 0xFFFF;

// Button Page (0x09)
pub const kHIDUsage_Button_1:    c_int = 0x01;
pub const kHIDUsage_Button_2:    c_int = 0x02;
pub const kHIDUsage_Button_3:    c_int = 0x03;
pub const kHIDUsage_Button_4:    c_int = 0x04;
// ...
pub const kHIDUsage_Button_65535:    c_int = 0xFFFF;
// Ordinal Page (0x0A)
// 0x00 Reserved
pub const kHIDUsage_Ord_Instance1:    c_int = 0x01;
pub const kHIDUsage_Ord_Instance2:    c_int = 0x02;
pub const kHIDUsage_Ord_Instance3:    c_int = 0x03;
pub const kHIDUsage_Ord_Instance4:    c_int = 0x04;
pub const kHIDUsage_Ord_Instance65535:    c_int = 0xFFFF;

// Telephony Page (0x0B)
pub const kHIDUsage_Tfon_Phone:    c_int = 0x01;
pub const kHIDUsage_Tfon_AnsweringMachine:    c_int = 0x02;
pub const kHIDUsage_Tfon_MessageControls:    c_int = 0x03;
pub const kHIDUsage_Tfon_Handset:    c_int = 0x04;
pub const kHIDUsage_Tfon_Headset:    c_int = 0x05;
pub const kHIDUsage_Tfon_TelephonyKeyPad:    c_int = 0x06;
pub const kHIDUsage_Tfon_ProgrammableButton:    c_int = 0x07;
// 0x08 - 0x1F Reserved
pub const kHIDUsage_Tfon_HookSwitch:    c_int = 0x20;
pub const kHIDUsage_Tfon_Flash:    c_int = 0x21;
pub const kHIDUsage_Tfon_Feature:    c_int = 0x22;
pub const kHIDUsage_Tfon_Hold:    c_int = 0x23;
pub const kHIDUsage_Tfon_Redial:    c_int = 0x24;
pub const kHIDUsage_Tfon_Transfer:    c_int = 0x25;
pub const kHIDUsage_Tfon_Drop:    c_int = 0x26;
pub const kHIDUsage_Tfon_Park:    c_int = 0x27;
pub const kHIDUsage_Tfon_ForwardCalls:    c_int = 0x28;
pub const kHIDUsage_Tfon_AlternateFunction:    c_int = 0x29;
pub const kHIDUsage_Tfon_Line:    c_int = 0x2A;
pub const kHIDUsage_Tfon_SpeakerPhone:    c_int = 0x2B;
pub const kHIDUsage_Tfon_Conference:    c_int = 0x2C;
pub const kHIDUsage_Tfon_RingEnable:    c_int = 0x2D;
pub const kHIDUsage_Tfon_Ring:    c_int = 0x2E;
pub const kHIDUsage_Tfon_PhoneMute:    c_int = 0x2F;
pub const kHIDUsage_Tfon_CallerID:    c_int = 0x30;
// 0x31 - 0x4F Reserved
pub const kHIDUsage_Tfon_SpeedDial:    c_int = 0x50;
pub const kHIDUsage_Tfon_StoreNumber:    c_int = 0x51;
pub const kHIDUsage_Tfon_RecallNumber:    c_int = 0x52;
pub const kHIDUsage_Tfon_PhoneDirectory:    c_int = 0x53;
// 0x54 - 0x6F Reserved
pub const kHIDUsage_Tfon_VoiceMail:    c_int = 0x70;
pub const kHIDUsage_Tfon_ScreenCalls:    c_int = 0x71;
pub const kHIDUsage_Tfon_DoNotDisturb:    c_int = 0x72;
pub const kHIDUsage_Tfon_Message:    c_int = 0x73;
pub const kHIDUsage_Tfon_AnswerOnOrOff:    c_int = 0x74;
// 0x75 - 0x8F Reserved
pub const kHIDUsage_Tfon_InsideDialTone:    c_int = 0x90;
pub const kHIDUsage_Tfon_OutsideDialTone:    c_int = 0x91;
pub const kHIDUsage_Tfon_InsideRingTone:    c_int = 0x92;
pub const kHIDUsage_Tfon_OutsideRingTone:    c_int = 0x93;
pub const kHIDUsage_Tfon_PriorityRingTone:    c_int = 0x94;
pub const kHIDUsage_Tfon_InsideRingback:    c_int = 0x95;
pub const kHIDUsage_Tfon_PriorityRingback:    c_int = 0x96;
pub const kHIDUsage_Tfon_LineBusyTone:    c_int = 0x97;
pub const kHIDUsage_Tfon_ReorderTone:    c_int = 0x98;
pub const kHIDUsage_Tfon_CallWaitingTone:    c_int = 0x99;
pub const kHIDUsage_Tfon_ConfirmationTone1:    c_int = 0x9A;
pub const kHIDUsage_Tfon_ConfirmationTone2:    c_int = 0x9B;
pub const kHIDUsage_Tfon_TonesOff:    c_int = 0x9C;
pub const kHIDUsage_Tfon_OutsideRingback:    c_int = 0x9D;
// 0x9E - 0xAF Reserved
pub const kHIDUsage_Tfon_PhoneKey0:    c_int = 0xB0;
pub const kHIDUsage_Tfon_PhoneKey1:    c_int = 0xB1;
pub const kHIDUsage_Tfon_PhoneKey2:    c_int = 0xB2;
pub const kHIDUsage_Tfon_PhoneKey3:    c_int = 0xB3;
pub const kHIDUsage_Tfon_PhoneKey4:    c_int = 0xB4;
pub const kHIDUsage_Tfon_PhoneKey5:    c_int = 0xB5;
pub const kHIDUsage_Tfon_PhoneKey6:    c_int = 0xB6;
pub const kHIDUsage_Tfon_PhoneKey7:    c_int = 0xB7;
pub const kHIDUsage_Tfon_PhoneKey8:    c_int = 0xB8;
pub const kHIDUsage_Tfon_PhoneKey9:    c_int = 0xB9;
pub const kHIDUsage_Tfon_PhoneKeyStar:    c_int = 0xBA;
pub const kHIDUsage_Tfon_PhoneKeyPound:    c_int = 0xBB;
pub const kHIDUsage_Tfon_PhoneKeyA:    c_int = 0xBC;
pub const kHIDUsage_Tfon_PhoneKeyB:    c_int = 0xBD;
pub const kHIDUsage_Tfon_PhoneKeyC:    c_int = 0xBE;
pub const kHIDUsage_Tfon_PhoneKeyD:    c_int = 0xBF;
// 0xC0 - 0xFFFF Reserved
pub const kHIDUsage_TFon_Reserved: c_int = 0xFFFF;

// Consumer Page (0x0C)
pub const kHIDUsage_Csmr_ConsumerControl:    c_int = 0x01;
pub const kHIDUsage_Csmr_NumericKeyPad:    c_int = 0x02;
pub const kHIDUsage_Csmr_ProgrammableButtons:    c_int = 0x03;
pub const kHIDUsage_Csmr_Microphone:   c_int = 0x04;
pub const kHIDUsage_Csmr_Headphone:   c_int = 0x05;
pub const kHIDUsage_Csmr_GraphicEqualizer:   c_int = 0x06;
// 0x07 - 0x1F Reserved
pub const kHIDUsage_Csmr_Plus10:    c_int = 0x20;
pub const kHIDUsage_Csmr_Plus100:    c_int = 0x21;
pub const kHIDUsage_Csmr_AMOrPM:    c_int = 0x22;
// 0x23 - 0x3F Reserved
pub const kHIDUsage_Csmr_Power:    c_int = 0x30;
pub const kHIDUsage_Csmr_Reset:    c_int = 0x31;
pub const kHIDUsage_Csmr_Sleep:    c_int = 0x32;
pub const kHIDUsage_Csmr_SleepAfter:    c_int = 0x33;
pub const kHIDUsage_Csmr_SleepMode:    c_int = 0x34;
pub const kHIDUsage_Csmr_Illumination:    c_int = 0x35;
pub const kHIDUsage_Csmr_FunctionButtons:    c_int = 0x36;
// 0x37 - 0x3F Reserved
pub const kHIDUsage_Csmr_Menu:    c_int = 0x40;
pub const kHIDUsage_Csmr_MenuPick:    c_int = 0x41;
pub const kHIDUsage_Csmr_MenuUp:    c_int = 0x42;
pub const kHIDUsage_Csmr_MenuDown:    c_int = 0x43;
pub const kHIDUsage_Csmr_MenuLeft:    c_int = 0x44;
pub const kHIDUsage_Csmr_MenuRight:    c_int = 0x45;
pub const kHIDUsage_Csmr_MenuEscape:    c_int = 0x46;
pub const kHIDUsage_Csmr_MenuValueIncrease:    c_int = 0x47;
pub const kHIDUsage_Csmr_MenuValueDecrease:    c_int = 0x48;
// 0x49 - 0x5F Reserved
pub const kHIDUsage_Csmr_DataOnScreen:    c_int = 0x60;
pub const kHIDUsage_Csmr_ClosedCaption:    c_int = 0x61;
pub const kHIDUsage_Csmr_ClosedCaptionSelect:    c_int = 0x62;
pub const kHIDUsage_Csmr_VCROrTV:    c_int = 0x63;
pub const kHIDUsage_Csmr_BroadcastMode:    c_int = 0x64;
pub const kHIDUsage_Csmr_Snapshot:    c_int = 0x65;
pub const kHIDUsage_Csmr_Still:    c_int = 0x66;
pub const kHIDUsage_Csmr_PictureInPictureToggle: c_int = 0x67;
pub const kHIDUsage_Csmr_PictureInPictureSwap: c_int = 0x68;
pub const kHIDUsage_Csmr_RedMenuButton: c_int = 0x69;
pub const kHIDUsage_Csmr_GreenMenuButton: c_int = 0x6A;
pub const kHIDUsage_Csmr_BlueMenuButton: c_int = 0x6B;
pub const kHIDUsage_Csmr_YellowMenuButton: c_int = 0x6C;
pub const kHIDUsage_Csmr_Aspect: c_int = 0x6D;
pub const kHIDUsage_Csmr_3DModeSelect: c_int = 0x6E;
pub const kHIDUsage_Csmr_DisplayBrightnessIncrement: c_int = 0x6F;
pub const kHIDUsage_Csmr_DisplayBrightnessDecrement: c_int = 0x70;
pub const kHIDUsage_Csmr_DisplayBrightness: c_int = 0x71;
pub const kHIDUsage_Csmr_DisplayBacklightToggle: c_int = 0x72;
pub const kHIDUsage_Csmr_DisplayBrightnessMinimum: c_int = 0x73;
pub const kHIDUsage_Csmr_DisplayBrightnessMaximum: c_int = 0x74;
pub const kHIDUsage_Csmr_DisplayBrightnessSetAutoBrightness: c_int = 0x75;
// 0x76 - 0x7F Reserved
pub const kHIDUsage_Csmr_Selection:    c_int = 0x80;
pub const kHIDUsage_Csmr_Assign:    c_int = 0x81;
pub const kHIDUsage_Csmr_ModeStep:    c_int = 0x82;
pub const kHIDUsage_Csmr_RecallLast:    c_int = 0x83;
pub const kHIDUsage_Csmr_EnterChannel:    c_int = 0x84;
pub const kHIDUsage_Csmr_OrderMovie:    c_int = 0x85;
pub const kHIDUsage_Csmr_Channel:    c_int = 0x86;
pub const kHIDUsage_Csmr_MediaSelection:    c_int = 0x87;
pub const kHIDUsage_Csmr_MediaSelectComputer:    c_int = 0x88;
pub const kHIDUsage_Csmr_MediaSelectTV:    c_int = 0x89;
pub const kHIDUsage_Csmr_MediaSelectWWW:    c_int = 0x8A;
pub const kHIDUsage_Csmr_MediaSelectDVD:    c_int = 0x8B;
pub const kHIDUsage_Csmr_MediaSelectTelephone:    c_int = 0x8C;
pub const kHIDUsage_Csmr_MediaSelectProgramGuide:    c_int = 0x8D;
pub const kHIDUsage_Csmr_MediaSelectVideoPhone:    c_int = 0x8E;
pub const kHIDUsage_Csmr_MediaSelectGames:    c_int = 0x8F;
pub const kHIDUsage_Csmr_MediaSelectMessages:    c_int = 0x90;
pub const kHIDUsage_Csmr_MediaSelectCD:    c_int = 0x91;
pub const kHIDUsage_Csmr_MediaSelectVCR:    c_int = 0x92;
pub const kHIDUsage_Csmr_MediaSelectTuner:    c_int = 0x93;
pub const kHIDUsage_Csmr_Quit:    c_int = 0x94;
pub const kHIDUsage_Csmr_Help:    c_int = 0x95;
pub const kHIDUsage_Csmr_MediaSelectTape:    c_int = 0x96;
pub const kHIDUsage_Csmr_MediaSelectCable:    c_int = 0x97;
pub const kHIDUsage_Csmr_MediaSelectSatellite:    c_int = 0x98;
pub const kHIDUsage_Csmr_MediaSelectSecurity:    c_int = 0x99;
pub const kHIDUsage_Csmr_MediaSelectHome:    c_int = 0x9A;
pub const kHIDUsage_Csmr_MediaSelectCall:    c_int = 0x9B;
pub const kHIDUsage_Csmr_ChannelIncrement:    c_int = 0x9C;
pub const kHIDUsage_Csmr_ChannelDecrement:    c_int = 0x9D;
pub const kHIDUsage_Csmr_Media:    c_int = 0x9E;
// 0x9F Reserved
pub const kHIDUsage_Csmr_VCRPlus:    c_int = 0xA0;
pub const kHIDUsage_Csmr_Once:    c_int = 0xA1;
pub const kHIDUsage_Csmr_Daily:    c_int = 0xA2;
pub const kHIDUsage_Csmr_Weekly:    c_int = 0xA3;
pub const kHIDUsage_Csmr_Monthly:    c_int = 0xA4;
// 0xA5 - 0xAF Reserved
pub const kHIDUsage_Csmr_Play:    c_int = 0xB0;
pub const kHIDUsage_Csmr_Pause:    c_int = 0xB1;
pub const kHIDUsage_Csmr_Record:    c_int = 0xB2;
pub const kHIDUsage_Csmr_FastForward:    c_int = 0xB3;
pub const kHIDUsage_Csmr_Rewind:    c_int = 0xB4;
pub const kHIDUsage_Csmr_ScanNextTrack:    c_int = 0xB5;
pub const kHIDUsage_Csmr_ScanPreviousTrack:    c_int = 0xB6;
pub const kHIDUsage_Csmr_Stop:    c_int = 0xB7;
pub const kHIDUsage_Csmr_Eject:    c_int = 0xB8;
pub const kHIDUsage_Csmr_RandomPlay:    c_int = 0xB9;
pub const kHIDUsage_Csmr_SelectDisc:    c_int = 0xBA;
pub const kHIDUsage_Csmr_EnterDisc:    c_int = 0xBB;
pub const kHIDUsage_Csmr_Repeat:    c_int = 0xBC;
pub const kHIDUsage_Csmr_Tracking:    c_int = 0xBD;
pub const kHIDUsage_Csmr_TrackNormal:    c_int = 0xBE;
pub const kHIDUsage_Csmr_SlowTracking:    c_int = 0xBF;
pub const kHIDUsage_Csmr_FrameForward:    c_int = 0xC0;
pub const kHIDUsage_Csmr_FrameBack:    c_int = 0xC1;
pub const kHIDUsage_Csmr_Mark:    c_int = 0xC2;
pub const kHIDUsage_Csmr_ClearMark:    c_int = 0xC3;
pub const kHIDUsage_Csmr_RepeatFromMark:    c_int = 0xC4;
pub const kHIDUsage_Csmr_ReturnToMark:    c_int = 0xC5;
pub const kHIDUsage_Csmr_SearchMarkForward:    c_int = 0xC6;
pub const kHIDUsage_Csmr_SearchMarkBackwards:    c_int = 0xC7;
pub const kHIDUsage_Csmr_CounterReset:    c_int = 0xC8;
pub const kHIDUsage_Csmr_ShowCounter:    c_int = 0xC9;
pub const kHIDUsage_Csmr_TrackingIncrement:    c_int = 0xCA;
pub const kHIDUsage_Csmr_TrackingDecrement:    c_int = 0xCB;
pub const kHIDUsage_Csmr_StopOrEject:    c_int = 0xCC;
pub const kHIDUsage_Csmr_PlayOrPause:    c_int = 0xCD;
pub const kHIDUsage_Csmr_PlayOrSkip:    c_int = 0xCE;
pub const kHIDUsage_Csmr_VoiceCommand:    c_int = 0xCF;
// 0xCF - 0xDF Reserved
pub const kHIDUsage_Csmr_Volume:    c_int = 0xE0;
pub const kHIDUsage_Csmr_Balance:    c_int = 0xE1;
pub const kHIDUsage_Csmr_Mute:    c_int = 0xE2;
pub const kHIDUsage_Csmr_Bass:    c_int = 0xE3;
pub const kHIDUsage_Csmr_Treble:    c_int = 0xE4;
pub const kHIDUsage_Csmr_BassBoost:    c_int = 0xE5;
pub const kHIDUsage_Csmr_SurroundMode:    c_int = 0xE6;
pub const kHIDUsage_Csmr_Loudness:    c_int = 0xE7;
pub const kHIDUsage_Csmr_MPX:    c_int = 0xE8;
pub const kHIDUsage_Csmr_VolumeIncrement:    c_int = 0xE9;
pub const kHIDUsage_Csmr_VolumeDecrement:    c_int = 0xEA;
// 0xEB - 0xEF Reserved
pub const kHIDUsage_Csmr_Speed:    c_int = 0xF0;
pub const kHIDUsage_Csmr_PlaybackSpeed:    c_int = 0xF1;
pub const kHIDUsage_Csmr_StandardPlay:    c_int = 0xF2;
pub const kHIDUsage_Csmr_LongPlay:    c_int = 0xF3;
pub const kHIDUsage_Csmr_ExtendedPlay:    c_int = 0xF4;
pub const kHIDUsage_Csmr_Slow:    c_int = 0xF5;
// 0xF6 - 0xFF Reserved
pub const kHIDUsage_Csmr_FanEnable:    c_int = 0x100;
pub const kHIDUsage_Csmr_FanSpeed:    c_int = 0x101;
pub const kHIDUsage_Csmr_LightEnable:    c_int = 0x102;
pub const kHIDUsage_Csmr_LightIlluminationLevel:    c_int = 0x103;
pub const kHIDUsage_Csmr_ClimateControlEnable:    c_int = 0x104;
pub const kHIDUsage_Csmr_RoomTemperature:    c_int = 0x105;
pub const kHIDUsage_Csmr_SecurityEnable:    c_int = 0x106;
pub const kHIDUsage_Csmr_FireAlarm:    c_int = 0x107;
pub const kHIDUsage_Csmr_PoliceAlarm:    c_int = 0x108;
pub const kHIDUsage_Csmr_Proximity:    c_int = 0x109;
pub const kHIDUsage_Csmr_Motion:    c_int = 0x10A;
pub const kHIDUsage_Csmr_DuressAlarm:    c_int = 0x10B;
pub const kHIDUsage_Csmr_HoldupAlarm:    c_int = 0x10C;
pub const kHIDUsage_Csmr_MedicalAlarm:    c_int = 0x10D;
// 0x10E - 0x14F Reserved
pub const kHIDUsage_Csmr_BalanceRight:    c_int = 0x150;
pub const kHIDUsage_Csmr_BalanceLeft:    c_int = 0x151;
pub const kHIDUsage_Csmr_BassIncrement:    c_int = 0x152;
pub const kHIDUsage_Csmr_BassDecrement:    c_int = 0x153;
pub const kHIDUsage_Csmr_TrebleIncrement:    c_int = 0x154;
pub const kHIDUsage_Csmr_TrebleDecrement:    c_int = 0x155;
// 0x156 - 0x15F Reserved
pub const kHIDUsage_Csmr_SpeakerSystem:    c_int = 0x160;
pub const kHIDUsage_Csmr_ChannelLeft:    c_int = 0x161;
pub const kHIDUsage_Csmr_ChannelRight:    c_int = 0x162;
pub const kHIDUsage_Csmr_ChannelCenter:    c_int = 0x163;
pub const kHIDUsage_Csmr_ChannelFront:    c_int = 0x164;
pub const kHIDUsage_Csmr_ChannelCenterFront:    c_int = 0x165;
pub const kHIDUsage_Csmr_ChannelSide:    c_int = 0x166;
pub const kHIDUsage_Csmr_ChannelSurround:    c_int = 0x167;
pub const kHIDUsage_Csmr_ChannelLowFrequencyEnhancement:    c_int = 0x168;
pub const kHIDUsage_Csmr_ChannelTop:    c_int = 0x169;
pub const kHIDUsage_Csmr_ChannelUnknown:    c_int = 0x16A;
// 0x16B - 0x16F Reserved
pub const kHIDUsage_Csmr_SubChannel:    c_int = 0x170;
pub const kHIDUsage_Csmr_SubChannelIncrement:    c_int = 0x171;
pub const kHIDUsage_Csmr_SubChannelDecrement:    c_int = 0x172;
pub const kHIDUsage_Csmr_AlternateAudioIncrement:    c_int = 0x173;
pub const kHIDUsage_Csmr_AlternateAudioDecrement:    c_int = 0x174;
// 0x175 - 0x17F Reserved
pub const kHIDUsage_Csmr_ApplicationLaunchButtons:    c_int = 0x180;
pub const kHIDUsage_Csmr_ALLaunchButtonConfigurationTool:    c_int = 0x181;
pub const kHIDUsage_Csmr_ALProgrammableButtonConfiguration:    c_int = 0x182;
pub const kHIDUsage_Csmr_ALConsumerControlConfiguration:    c_int = 0x183;
pub const kHIDUsage_Csmr_ALWordProcessor:    c_int = 0x184;
pub const kHIDUsage_Csmr_ALTextEditor:    c_int = 0x185;
pub const kHIDUsage_Csmr_ALSpreadsheet:    c_int = 0x186;
pub const kHIDUsage_Csmr_ALGraphicsEditor:    c_int = 0x187;
pub const kHIDUsage_Csmr_ALPresentationApp:    c_int = 0x188;
pub const kHIDUsage_Csmr_ALDatabaseApp:    c_int = 0x189;
pub const kHIDUsage_Csmr_ALEmailReader:    c_int = 0x18A;
pub const kHIDUsage_Csmr_ALNewsreader:    c_int = 0x18B;
pub const kHIDUsage_Csmr_ALVoicemail:    c_int = 0x18C;
pub const kHIDUsage_Csmr_ALContactsOrAddressBook:    c_int = 0x18D;
pub const kHIDUsage_Csmr_ALCalendarOrSchedule:    c_int = 0x18E;
pub const kHIDUsage_Csmr_ALTaskOrProjectManager:    c_int = 0x18F;
pub const kHIDUsage_Csmr_ALLogOrJournalOrTimecard:    c_int = 0x190;
pub const kHIDUsage_Csmr_ALCheckbookOrFinance:    c_int = 0x191;
pub const kHIDUsage_Csmr_ALCalculator:    c_int = 0x192;
pub const kHIDUsage_Csmr_ALAOrVCaptureOrPlayback:    c_int = 0x193;
pub const kHIDUsage_Csmr_ALLocalMachineBrowser:    c_int = 0x194;
pub const kHIDUsage_Csmr_ALLANOrWANBrowser:    c_int = 0x195;
pub const kHIDUsage_Csmr_ALInternetBrowser:    c_int = 0x196;
pub const kHIDUsage_Csmr_ALRemoteNetworkingOrISPConnect:    c_int = 0x197;
pub const kHIDUsage_Csmr_ALNetworkConference:    c_int = 0x198;
pub const kHIDUsage_Csmr_ALNetworkChat:    c_int = 0x199;
pub const kHIDUsage_Csmr_ALTelephonyOrDialer:    c_int = 0x19A;
pub const kHIDUsage_Csmr_ALLogon:    c_int = 0x19B;
pub const kHIDUsage_Csmr_ALLogoff:    c_int = 0x19C;
pub const kHIDUsage_Csmr_ALLogonOrLogoff:    c_int = 0x19D;
pub const kHIDUsage_Csmr_ALTerminalLockOrScreensaver:    c_int = 0x19E;
pub const kHIDUsage_Csmr_ALControlPanel:    c_int = 0x19F;
pub const kHIDUsage_Csmr_ALCommandLineProcessorOrRun:    c_int = 0x1A0;
pub const kHIDUsage_Csmr_ALProcessOrTaskManager:    c_int = 0x1A1;
pub const kHIDUsage_Csmr_AL:    c_int = 0x1A2;
pub const kHIDUsage_Csmr_ALNextTaskOrApplication:    c_int = 0x1A3;
pub const kHIDUsage_Csmr_ALPreviousTaskOrApplication:    c_int = 0x1A4;
pub const kHIDUsage_Csmr_ALPreemptiveHaltTaskOrApplication:    c_int = 0x1A5;
pub const kHIDUsage_Csmr_ALIntegratedHelpCenter:   c_int = 0x1A6;
pub const kHIDUsage_Csmr_ALDocuments:   c_int = 0x1A7;
pub const kHIDUsage_Csmr_ALThesaurus:   c_int = 0x1A8;
pub const kHIDUsage_Csmr_ALDictionary:   c_int = 0x1A9;
pub const kHIDUsage_Csmr_ALDesktop:   c_int = 0x1AA;
pub const kHIDUsage_Csmr_ALSpellCheck:   c_int = 0x1AB;
pub const kHIDUsage_Csmr_ALGrammerCheck:   c_int = 0x1AC;
pub const kHIDUsage_Csmr_ALWirelessStatus:   c_int = 0x1AD;
pub const kHIDUsage_Csmr_ALKeyboardLayout:   c_int = 0x1AE;
pub const kHIDUsage_Csmr_ALVirusProtection:   c_int = 0x1AF;
pub const kHIDUsage_Csmr_ALEncryption:   c_int = 0x1B0;
pub const kHIDUsage_Csmr_ALScreenSaver:   c_int = 0x1B1;
pub const kHIDUsage_Csmr_ALAlarms:   c_int = 0x1B2;
pub const kHIDUsage_Csmr_ALClock:   c_int = 0x1B3;
pub const kHIDUsage_Csmr_ALFileBrowser:   c_int = 0x1B4;
pub const kHIDUsage_Csmr_ALPowerStatus:   c_int = 0x1B5;
pub const kHIDUsage_Csmr_ALImageBrowser:   c_int = 0x1B6;
pub const kHIDUsage_Csmr_ALAudioBrowser:   c_int = 0x1B7;
pub const kHIDUsage_Csmr_ALMovieBrowser:   c_int = 0x1B8;
pub const kHIDUsage_Csmr_ALDigitalRightsManager:   c_int = 0x1B9;
pub const kHIDUsage_Csmr_ALDigitalWallet:   c_int = 0x1BA;
// 0x1BB Reserved
pub const kHIDUsage_Csmr_ALInstantMessaging:   c_int = 0x1BC;
pub const kHIDUsage_Csmr_ALOEMFeatureBrowser:  c_int = 0x1BD;
pub const kHIDUsage_Csmr_ALOEMHelp:    c_int = 0x1BE;
pub const kHIDUsage_Csmr_ALOnlineCommunity:    c_int = 0x1BF;
pub const kHIDUsage_Csmr_ALEntertainmentContentBrowser:    c_int = 0x1C0;
pub const kHIDUsage_Csmr_ALOnlineShoppingBrowswer: c_int = 0x1C1;
pub const kHIDUsage_Csmr_ALSmartCardInformationOrHelp: c_int = 0x1C2;
pub const kHIDUsage_Csmr_ALMarketMonitorOrFinanceBrowser:  c_int = 0x1C3;
pub const kHIDUsage_Csmr_ALCustomizedCorporateNewsBrowser: c_int = 0x1C4;
pub const kHIDUsage_Csmr_ALOnlineActivityBrowswer: c_int = 0x1C5;
pub const kHIDUsage_Csmr_ALResearchOrSearchBrowswer:   c_int = 0x1C6;
pub const kHIDUsage_Csmr_ALAudioPlayer: c_int = 0x1C7;
// 0x1C8 - 0x1FF Reserved
pub const kHIDUsage_Csmr_GenericGUIApplicationControls:    c_int = 0x200;
pub const kHIDUsage_Csmr_ACNew:    c_int = 0x201;
pub const kHIDUsage_Csmr_ACOpen:    c_int = 0x202;
pub const kHIDUsage_Csmr_ACClose:    c_int = 0x203;
pub const kHIDUsage_Csmr_ACExit:    c_int = 0x204;
pub const kHIDUsage_Csmr_ACMaximize:    c_int = 0x205;
pub const kHIDUsage_Csmr_ACMinimize:    c_int = 0x206;
pub const kHIDUsage_Csmr_ACSave:    c_int = 0x207;
pub const kHIDUsage_Csmr_ACPrint:    c_int = 0x208;
pub const kHIDUsage_Csmr_ACProperties:    c_int = 0x209;
pub const kHIDUsage_Csmr_ACUndo:    c_int = 0x21A;
pub const kHIDUsage_Csmr_ACCopy:    c_int = 0x21B;
pub const kHIDUsage_Csmr_ACCut:    c_int = 0x21C;
pub const kHIDUsage_Csmr_ACPaste:    c_int = 0x21D;
pub const kHIDUsage_Csmr_AC:    c_int = 0x21E;
pub const kHIDUsage_Csmr_ACFind:    c_int = 0x21F;
pub const kHIDUsage_Csmr_ACFindandReplace:    c_int = 0x220;
pub const kHIDUsage_Csmr_ACSearch:    c_int = 0x221;
pub const kHIDUsage_Csmr_ACGoTo:    c_int = 0x222;
pub const kHIDUsage_Csmr_ACHome:    c_int = 0x223;
pub const kHIDUsage_Csmr_ACBack:    c_int = 0x224;
pub const kHIDUsage_Csmr_ACForward:    c_int = 0x225;
pub const kHIDUsage_Csmr_ACStop:    c_int = 0x226;
pub const kHIDUsage_Csmr_ACRefresh:    c_int = 0x227;
pub const kHIDUsage_Csmr_ACPreviousLink:    c_int = 0x228;
pub const kHIDUsage_Csmr_ACNextLink:    c_int = 0x229;
pub const kHIDUsage_Csmr_ACBookmarks:    c_int = 0x22A;
pub const kHIDUsage_Csmr_ACHistory:    c_int = 0x22B;
pub const kHIDUsage_Csmr_ACSubscriptions:    c_int = 0x22C;
pub const kHIDUsage_Csmr_ACZoomIn:    c_int = 0x22D;
pub const kHIDUsage_Csmr_ACZoomOut:    c_int = 0x22E;
pub const kHIDUsage_Csmr_ACZoom:    c_int = 0x22F;
pub const kHIDUsage_Csmr_ACFullScreenView:    c_int = 0x230;
pub const kHIDUsage_Csmr_ACNormalView:    c_int = 0x231;
pub const kHIDUsage_Csmr_ACViewToggle:    c_int = 0x232;
pub const kHIDUsage_Csmr_ACScrollUp:    c_int = 0x233;
pub const kHIDUsage_Csmr_ACScrollDown:    c_int = 0x234;
pub const kHIDUsage_Csmr_ACScroll:    c_int = 0x235;
pub const kHIDUsage_Csmr_ACPanLeft:    c_int = 0x236;
pub const kHIDUsage_Csmr_ACPanRight:    c_int = 0x237;
pub const kHIDUsage_Csmr_ACPan:    c_int = 0x238;
pub const kHIDUsage_Csmr_ACNewWindow:    c_int = 0x239;
pub const kHIDUsage_Csmr_ACTileHorizontally:    c_int = 0x23A;
pub const kHIDUsage_Csmr_ACTileVertically:    c_int = 0x23B;
pub const kHIDUsage_Csmr_ACFormat:    c_int = 0x23C;
pub const kHIDUsage_Csmr_ACEdit:   c_int = 0x23D;
pub const kHIDUsage_Csmr_ACBold:   c_int = 0x23E;
pub const kHIDUsage_Csmr_ACItalics:    c_int = 0x23F;
pub const kHIDUsage_Csmr_ACUnderline:  c_int = 0x240;
pub const kHIDUsage_Csmr_ACStrikethrough:  c_int = 0x241;
pub const kHIDUsage_Csmr_ACSubscript:  c_int = 0x242;
pub const kHIDUsage_Csmr_ACSuperscript:    c_int = 0x243;
pub const kHIDUsage_Csmr_ACAllCaps:    c_int = 0x244;
pub const kHIDUsage_Csmr_ACRotate: c_int = 0x245;
pub const kHIDUsage_Csmr_ACResize: c_int = 0x246;
pub const kHIDUsage_Csmr_ACFlipHorizontal: c_int = 0x247;
pub const kHIDUsage_Csmr_ACFlipVertical:   c_int = 0x248;
pub const kHIDUsage_Csmr_ACMirrorHorizontal:   c_int = 0x249;
pub const kHIDUsage_Csmr_ACMirrorVertical: c_int = 0x24A;
pub const kHIDUsage_Csmr_ACFontSelect: c_int = 0x24B;
pub const kHIDUsage_Csmr_ACFontColor:  c_int = 0x24C;
pub const kHIDUsage_Csmr_ACFontSize:   c_int = 0x24D;
pub const kHIDUsage_Csmr_ACJustifyLeft:    c_int = 0x24E;
pub const kHIDUsage_Csmr_ACJustifyCenterH: c_int = 0x24F;
pub const kHIDUsage_Csmr_ACJustifyRight:   c_int = 0x250;
pub const kHIDUsage_Csmr_ACJustifyBlockH:  c_int = 0x251;
pub const kHIDUsage_Csmr_ACJustifyTop:     c_int = 0x252;
pub const kHIDUsage_Csmr_ACJustifyCenterV: c_int = 0x253;
pub const kHIDUsage_Csmr_ACJustifyBottom:  c_int = 0x254;
pub const kHIDUsage_Csmr_ACJustifyBlockV:  c_int = 0x255;
pub const kHIDUsage_Csmr_ACIndentyDecrease:    c_int = 0x256;
pub const kHIDUsage_Csmr_ACIndentyIncrease:    c_int = 0x257;
pub const kHIDUsage_Csmr_ACNumberedList:   c_int = 0x258;
pub const kHIDUsage_Csmr_ACRestartNumbering:   c_int = 0x259;
pub const kHIDUsage_Csmr_ACBulletedList:   c_int = 0x25A;
pub const kHIDUsage_Csmr_ACPromote:    c_int = 0x25B;
pub const kHIDUsage_Csmr_ACDemote: c_int = 0x25C;
pub const kHIDUsage_Csmr_ACYes:    c_int = 0x25D;
pub const kHIDUsage_Csmr_ACNo: c_int = 0x25E;
pub const kHIDUsage_Csmr_ACCancel: c_int = 0x25F;
pub const kHIDUsage_Csmr_ACCatalog:    c_int = 0x260;
pub const kHIDUsage_Csmr_ACBuyOrCheckout:  c_int = 0x261;
pub const kHIDUsage_Csmr_ACAddToCart:  c_int = 0x262;
pub const kHIDUsage_Csmr_ACExpand: c_int = 0x263;
pub const kHIDUsage_Csmr_ACExpandAll:  c_int = 0x264;
pub const kHIDUsage_Csmr_ACCollapse:   c_int = 0x265;
pub const kHIDUsage_Csmr_ACCollapseAll:    c_int = 0x266;
pub const kHIDUsage_Csmr_ACPrintPreview:   c_int = 0x267;
pub const kHIDUsage_Csmr_ACPasteSpecial:   c_int = 0x268;
pub const kHIDUsage_Csmr_ACInsertMode: c_int = 0x269;
pub const kHIDUsage_Csmr_ACDelete: c_int = 0x26A;
pub const kHIDUsage_Csmr_ACLock:   c_int = 0x26B;
pub const kHIDUsage_Csmr_ACUnlock: c_int = 0x26C;
pub const kHIDUsage_Csmr_ACProtect:    c_int = 0x26D;
pub const kHIDUsage_Csmr_ACUnprotect:  c_int = 0x26E;
pub const kHIDUsage_Csmr_ACAttachComment:  c_int = 0x26F;
pub const kHIDUsage_Csmr_ACDetachComment:  c_int = 0x270;
pub const kHIDUsage_Csmr_ACViewComment:    c_int = 0x271;
pub const kHIDUsage_Csmr_ACSelectWord: c_int = 0x272;
pub const kHIDUsage_Csmr_ACSelectSentence: c_int = 0x273;
pub const kHIDUsage_Csmr_ACSelectParagraph:    c_int = 0x274;
pub const kHIDUsage_Csmr_ACSelectColumn:   c_int = 0x275;
pub const kHIDUsage_Csmr_ACSelectRow:  c_int = 0x276;
pub const kHIDUsage_Csmr_ACSelectTable:    c_int = 0x277;
pub const kHIDUsage_Csmr_ACSelectObject:   c_int = 0x278;
pub const kHIDUsage_Csmr_ACRedoOrRepeat:   c_int = 0x279;
pub const kHIDUsage_Csmr_ACSort:   c_int = 0x27A;
pub const kHIDUsage_Csmr_ACSortAscending:  c_int = 0x27B;
pub const kHIDUsage_Csmr_ACSortDescending: c_int = 0x27C;
pub const kHIDUsage_Csmr_ACFilter: c_int = 0x27D;
pub const kHIDUsage_Csmr_ACSetClock:   c_int = 0x27E;
pub const kHIDUsage_Csmr_ACViewClock:  c_int = 0x27F;
pub const kHIDUsage_Csmr_ACSelectTimeZone: c_int = 0x280;
pub const kHIDUsage_Csmr_ACEditTimeZones:  c_int = 0x281;
pub const kHIDUsage_Csmr_ACSetAlarm:   c_int = 0x282;
pub const kHIDUsage_Csmr_ACClearAlarm: c_int = 0x283;
pub const kHIDUsage_Csmr_ACSnoozeAlarm:    c_int = 0x284;
pub const kHIDUsage_Csmr_ACResetAlarm: c_int = 0x285;
pub const kHIDUsage_Csmr_ACSynchronize:    c_int = 0x286;
pub const kHIDUsage_Csmr_ACSendOrReceive:  c_int = 0x287;
pub const kHIDUsage_Csmr_ACSendTo: c_int = 0x288;
pub const kHIDUsage_Csmr_ACReply:  c_int = 0x289;
pub const kHIDUsage_Csmr_ACReplyAll:   c_int = 0x28A;
pub const kHIDUsage_Csmr_ACForwardMessage: c_int = 0x28B;
pub const kHIDUsage_Csmr_ACSend:   c_int = 0x28C;
pub const kHIDUsage_Csmr_ACAttachFile: c_int = 0x28D;
pub const kHIDUsage_Csmr_ACUpload: c_int = 0x28E;
pub const kHIDUsage_Csmr_ACDownload:   c_int = 0x28F;
pub const kHIDUsage_Csmr_ACSetBorders: c_int = 0x290;
pub const kHIDUsage_Csmr_ACInsertRow:  c_int = 0x291;
pub const kHIDUsage_Csmr_ACInsertColumn:   c_int = 0x292;
pub const kHIDUsage_Csmr_ACInsertFile: c_int = 0x293;
pub const kHIDUsage_Csmr_ACInsertPicture:  c_int = 0x294;
pub const kHIDUsage_Csmr_ACInsertObject:   c_int = 0x295;
pub const kHIDUsage_Csmr_ACInsertSymbol:   c_int = 0x296;
pub const kHIDUsage_Csmr_ACSaveAndClose:   c_int = 0x297;
pub const kHIDUsage_Csmr_ACRename: c_int = 0x298;
pub const kHIDUsage_Csmr_ACMerge:  c_int = 0x299;
pub const kHIDUsage_Csmr_ACSplit:  c_int = 0x29A;
pub const kHIDUsage_Csmr_ACDistributeH:    c_int = 0x29B;
pub const kHIDUsage_Csmr_ACDistributeV:    c_int = 0x29C;
pub const kHIDUsage_Csmr_ACKeyboardLayoutSelect:   c_int = 0x29D;
// 0x29E - 0xFFFF Reserved
pub const kHIDUsage_Csmr_Reserved: c_int = 0xFFFF;

// Digitizer Page (0x0D)
pub const kHIDUsage_Dig_Digitizer:                         c_int = 0x01;
pub const kHIDUsage_Dig_Pen:                               c_int = 0x02;
pub const kHIDUsage_Dig_LightPen:                          c_int = 0x03;
pub const kHIDUsage_Dig_TouchScreen:                       c_int = 0x04;
pub const kHIDUsage_Dig_TouchPad:                          c_int = 0x05;
pub const kHIDUsage_Dig_WhiteBoard:                        c_int = 0x06;
pub const kHIDUsage_Dig_CoordinateMeasuringMachine:        c_int = 0x07;
pub const kHIDUsage_Dig_3DDigitizer:                       c_int = 0x08;
pub const kHIDUsage_Dig_StereoPlotter:                     c_int = 0x09;
pub const kHIDUsage_Dig_ArticulatedArm:                    c_int = 0x0A;
pub const kHIDUsage_Dig_Armature:                          c_int = 0x0B;
pub const kHIDUsage_Dig_MultiplePointDigitizer:            c_int = 0x0C;
pub const kHIDUsage_Dig_FreeSpaceWand:                     c_int = 0x0D;
pub const kHIDUsage_Dig_DeviceConfiguration:               c_int = 0x0E;
// 0x0F - 0x1F Reserved
pub const kHIDUsage_Dig_Stylus:                            c_int = 0x20;
pub const kHIDUsage_Dig_Puck:                              c_int = 0x21;
pub const kHIDUsage_Dig_Finger:                            c_int = 0x22;
pub const kHIDUsage_Dig_DeviceSettings:                    c_int = 0x23;
pub const kHIDUsage_Dig_GestureCharacter:                  c_int = 0x24;
// 0x25 - 0x2F Reserved
pub const kHIDUsage_Dig_TipPressure:                       c_int = 0x30;
pub const kHIDUsage_Dig_BarrelPressure:                    c_int = 0x31;
pub const kHIDUsage_Dig_InRange:                           c_int = 0x32;
pub const kHIDUsage_Dig_Touch:                             c_int = 0x33;
pub const kHIDUsage_Dig_Untouch:                           c_int = 0x34;
pub const kHIDUsage_Dig_Tap:                               c_int = 0x35;
pub const kHIDUsage_Dig_Quality:                           c_int = 0x36;
pub const kHIDUsage_Dig_DataValid:                         c_int = 0x37;
pub const kHIDUsage_Dig_TransducerIndex:                   c_int = 0x38;
pub const kHIDUsage_Dig_TabletFunctionKeys:                c_int = 0x39;
pub const kHIDUsage_Dig_ProgramChangeKeys:                 c_int = 0x3A;
pub const kHIDUsage_Dig_BatteryStrength:                   c_int = 0x3B;
pub const kHIDUsage_Dig_Invert:                            c_int = 0x3C;
pub const kHIDUsage_Dig_XTilt:                             c_int = 0x3D;
pub const kHIDUsage_Dig_YTilt:                             c_int = 0x3E;
pub const kHIDUsage_Dig_Azimuth:                           c_int = 0x3F;
pub const kHIDUsage_Dig_Altitude:                          c_int = 0x40;
pub const kHIDUsage_Dig_Twist:                             c_int = 0x41;
pub const kHIDUsage_Dig_TipSwitch:                         c_int = 0x42;
pub const kHIDUsage_Dig_SecondaryTipSwitch:                c_int = 0x43;
pub const kHIDUsage_Dig_BarrelSwitch:                      c_int = 0x44;
pub const kHIDUsage_Dig_Eraser:                            c_int = 0x45;
pub const kHIDUsage_Dig_TabletPick:                        c_int = 0x46;
pub const kHIDUsage_Dig_TouchValid:                        c_int = 0x47;
pub const kHIDUsage_Dig_Width:                             c_int = 0x48;
pub const kHIDUsage_Dig_Height:                            c_int = 0x49;
// 0x4A - 0x50 Reserved
pub const kHIDUsage_Dig_ContactIdentifier:                 c_int = 0x51;
pub const kHIDUsage_Dig_DeviceMode:                        c_int = 0x52;
pub const kHIDUsage_Dig_DeviceIdentifier:                  c_int = 0x53;
pub const kHIDUsage_Dig_ContactCount:                      c_int = 0x54;
pub const kHIDUsage_Dig_ContactCountMaximum:               c_int = 0x55;

// 0x56 - 0x5F Reserved
pub const kHIDUsage_Dig_GestureCharacterEnable:            c_int = 0x60;
pub const kHIDUsage_Dig_GestureCharacterQuality:           c_int = 0x61;
pub const kHIDUsage_Dig_GestureCharacterDataLength:        c_int = 0x62;
pub const kHIDUsage_Dig_GestureCharacterData:              c_int = 0x63;
pub const kHIDUsage_Dig_GestureCharacterEncoding:          c_int = 0x64;
pub const kHIDUsage_Dig_GestureCharacterEncodingUTF8:      c_int = 0x65;
pub const kHIDUsage_Dig_GestureCharacterEncodingUTF16LE:   c_int = 0x66;
pub const kHIDUsage_Dig_GestureCharacterEncodingUTF16BE:   c_int = 0x67;
pub const kHIDUsage_Dig_GestureCharacterEncodingUTF32LE:   c_int = 0x68;
pub const kHIDUsage_Dig_GestureCharacterEncodingUTF32BE:   c_int = 0x69;

// 0x70 - 0xFFFF Reserved
pub const kHIDUsage_Dig_Reserved:                          c_int = 0xFFFF;

// Physical Interface Device Page (0x0F)
pub const kHIDUsage_PID_PhysicalInterfaceDevice: c_int = 0x01;
// 0x02 - 0x1F Reserved
pub const kHIDUsage_PID_Normal: c_int = 0x20;
pub const kHIDUsage_PID_SetEffectReport: c_int = 0x21;
pub const kHIDUsage_PID_EffectBlockIndex: c_int = 0x22;
pub const kHIDUsage_PID_ParamBlockOffset: c_int = 0x23;
pub const kHIDUsage_PID_ROM_Flag: c_int = 0x24;
pub const kHIDUsage_PID_EffectType: c_int = 0x25;
pub const kHIDUsage_PID_ET_ConstantForce: c_int = 0x26;
pub const kHIDUsage_PID_ET_Ramp: c_int = 0x27;
pub const kHIDUsage_PID_ET_CustomForceData: c_int = 0x28;

// 0x29 - 0x2F Reserved
pub const kHIDUsage_PID_ET_Square: c_int = 0x30;
pub const kHIDUsage_PID_ET_Sine: c_int = 0x31;
pub const kHIDUsage_PID_ET_Triangle: c_int = 0x32;
pub const kHIDUsage_PID_ET_SawtoothUp: c_int = 0x33;
pub const kHIDUsage_PID_ET_SawtoothDown: c_int = 0x34;
// 0x35 - 0x3F Reserved
pub const kHIDUsage_PID_ET_Spring: c_int = 0x40;
pub const kHIDUsage_PID_ET_Damper: c_int = 0x41;
pub const kHIDUsage_PID_ET_Inertia: c_int = 0x42;
pub const kHIDUsage_PID_ET_Friction: c_int = 0x43;
// 0x44 - 0x4F Reserved
pub const kHIDUsage_PID_Duration: c_int = 0x50;
pub const kHIDUsage_PID_SamplePeriod: c_int = 0x51;
pub const kHIDUsage_PID_Gain: c_int = 0x52;
pub const kHIDUsage_PID_TriggerButton: c_int = 0x53;
pub const kHIDUsage_PID_TriggerRepeatInterval: c_int = 0x54;
pub const kHIDUsage_PID_AxesEnable: c_int = 0x55;
pub const kHIDUsage_PID_DirectionEnable: c_int = 0x56;
pub const kHIDUsage_PID_Direction: c_int = 0x57;
pub const kHIDUsage_PID_TypeSpecificBlockOffset: c_int = 0x58;
pub const kHIDUsage_PID_BlockType: c_int = 0x59;
pub const kHIDUsage_PID_SetEnvelopeReport: c_int = 0x5A;
pub const kHIDUsage_PID_AttackLevel: c_int = 0x5B;
pub const kHIDUsage_PID_AttackTime: c_int = 0x5C;
pub const kHIDUsage_PID_FadeLevel: c_int = 0x5D;
pub const kHIDUsage_PID_FadeTime: c_int = 0x5E;
pub const kHIDUsage_PID_SetConditionReport: c_int = 0x5F;

pub const kHIDUsage_PID_CP_Offset: c_int = 0x60;
pub const kHIDUsage_PID_PositiveCoefficient: c_int = 0x61;
pub const kHIDUsage_PID_NegativeCoefficient: c_int = 0x62;
pub const kHIDUsage_PID_PositiveSaturation: c_int = 0x63;
pub const kHIDUsage_PID_NegativeSaturation: c_int = 0x64;
pub const kHIDUsage_PID_DeadBand: c_int = 0x65;
pub const kHIDUsage_PID_DownloadForceSample: c_int = 0x66;
pub const kHIDUsage_PID_IsochCustomForceEnable: c_int = 0x67;
pub const kHIDUsage_PID_CustomForceDataReport: c_int = 0x68;
pub const kHIDUsage_PID_CustomForceData: c_int = 0x69;
pub const kHIDUsage_PID_CustomForceVendorDefinedData: c_int = 0x6A;
pub const kHIDUsage_PID_SetCustomForceReport: c_int = 0x6B;
pub const kHIDUsage_PID_CustomForceDataOffset: c_int = 0x6C;
pub const kHIDUsage_PID_SampleCount: c_int = 0x6D;
pub const kHIDUsage_PID_SetPeriodicReport: c_int = 0x6E;
pub const kHIDUsage_PID_Offset: c_int = 0x6F;

pub const kHIDUsage_PID_Magnitude: c_int = 0x70;
pub const kHIDUsage_PID_Phase: c_int = 0x71;
pub const kHIDUsage_PID_Period: c_int = 0x72;
pub const kHIDUsage_PID_SetConstantForceReport: c_int = 0x73;
pub const kHIDUsage_PID_SetRampForceReport: c_int = 0x74;
pub const kHIDUsage_PID_RampStart: c_int = 0x75;
pub const kHIDUsage_PID_RampEnd: c_int = 0x76;
pub const kHIDUsage_PID_EffectOperationReport: c_int = 0x77;
pub const kHIDUsage_PID_EffectOperation: c_int = 0x78;
pub const kHIDUsage_PID_OpEffectStart: c_int = 0x79;
pub const kHIDUsage_PID_OpEffectStartSolo: c_int = 0x7A;
pub const kHIDUsage_PID_OpEffectStop: c_int = 0x7B;
pub const kHIDUsage_PID_LoopCount: c_int = 0x7C;
pub const kHIDUsage_PID_DeviceGainReport: c_int = 0x7D;
pub const kHIDUsage_PID_DeviceGain: c_int = 0x7E;
pub const kHIDUsage_PID_PoolReport: c_int = 0x7F;

pub const kHIDUsage_PID_RAM_PoolSize: c_int = 0x80;
pub const kHIDUsage_PID_ROM_PoolSize: c_int = 0x81;
pub const kHIDUsage_PID_ROM_EffectBlockCount: c_int = 0x82;
pub const kHIDUsage_PID_SimultaneousEffectsMax: c_int = 0x83;
pub const kHIDUsage_PID_PoolAlignment: c_int = 0x84;
pub const kHIDUsage_PID_PoolMoveReport: c_int = 0x85;
pub const kHIDUsage_PID_MoveSource: c_int = 0x86;
pub const kHIDUsage_PID_MoveDestination: c_int = 0x87;
pub const kHIDUsage_PID_MoveLength: c_int = 0x88;
pub const kHIDUsage_PID_BlockLoadReport: c_int = 0x89;
// 0x8A Reserved
pub const kHIDUsage_PID_BlockLoadStatus: c_int = 0x8B;
pub const kHIDUsage_PID_BlockLoadSuccess: c_int = 0x8C;
pub const kHIDUsage_PID_BlockLoadFull: c_int = 0x8D;
pub const kHIDUsage_PID_BlockLoadError: c_int = 0x8E;
pub const kHIDUsage_PID_BlockHandle: c_int = 0x8F;

pub const kHIDUsage_PID_BlockFreeReport: c_int = 0x90;
pub const kHIDUsage_PID_TypeSpecificBlockHandle: c_int = 0x91;
pub const kHIDUsage_PID_StateReport: c_int = 0x92;
// 0x93 Reserved
pub const kHIDUsage_PID_EffectPlaying: c_int = 0x94;
pub const kHIDUsage_PID_DeviceControlReport: c_int = 0x95;
pub const kHIDUsage_PID_DeviceControl: c_int = 0x96;
pub const kHIDUsage_PID_DC_EnableActuators: c_int = 0x97;
pub const kHIDUsage_PID_DC_DisableActuators: c_int = 0x98;
pub const kHIDUsage_PID_DC_StopAllEffects: c_int = 0x99;
pub const kHIDUsage_PID_DC_DeviceReset: c_int = 0x9A;
pub const kHIDUsage_PID_DC_DevicePause: c_int = 0x9B;
pub const kHIDUsage_PID_DC_DeviceContinue: c_int = 0x9C;
// 0x9d - 0x9E Reserved
pub const kHIDUsage_PID_DevicePaused: c_int = 0x9F;

pub const kHIDUsage_PID_ActuatorsEnabled: c_int = 0xA0;
// 0xA1 - 0xA3 Reserved
pub const kHIDUsage_PID_SafetySwitch: c_int = 0xA4;
pub const kHIDUsage_PID_ActuatorOverrideSwitch: c_int = 0xA5;
pub const kHIDUsage_PID_ActuatorPower: c_int = 0xA6;
pub const kHIDUsage_PID_StartDelay: c_int = 0xA7;
pub const kHIDUsage_PID_ParameterBlockSize: c_int = 0xA8;
pub const kHIDUsage_PID_DeviceManagedPool: c_int = 0xA9;
pub const kHIDUsage_PID_SharedParameterBlocks: c_int = 0xAA;
pub const kHIDUsage_PID_CreateNewEffectReport: c_int = 0xAB;
pub const kHIDUsage_PID_RAM_PoolAvailable: c_int = 0xAC;

// 0xAD - 0xFFFF Reserved
pub const kHIDUsage_PID_Reserved: c_int = 0xFFFF;

// AlphanumericDisplay Page (0x14)
pub const kHIDUsage_AD_AlphanumericDisplay:    c_int = 0x01;
// 0x02 - 0x1F Reserved
pub const kHIDUsage_AD_DisplayAttributesReport:    c_int = 0x20;
pub const kHIDUsage_AD_ASCIICharacterSet:    c_int = 0x21;
pub const kHIDUsage_AD_DataReadBack:    c_int = 0x22;
pub const kHIDUsage_AD_FontReadBack:    c_int = 0x23;
pub const kHIDUsage_AD_DisplayControlReport:    c_int = 0x24;
pub const kHIDUsage_AD_ClearDisplay:    c_int = 0x25;
pub const kHIDUsage_AD_DisplayEnable:    c_int = 0x26;
pub const kHIDUsage_AD_ScreenSaverDelay:    c_int = 0x27;
pub const kHIDUsage_AD_ScreenSaverEnable:    c_int = 0x28;
pub const kHIDUsage_AD_VerticalScroll:    c_int = 0x29;
pub const kHIDUsage_AD_HorizontalScroll:    c_int = 0x2A;
pub const kHIDUsage_AD_CharacterReport:    c_int = 0x2B;
pub const kHIDUsage_AD_DisplayData:    c_int = 0x2C;
pub const kHIDUsage_AD_DisplayStatus:    c_int = 0x2D;
pub const kHIDUsage_AD_StatNotReady:    c_int = 0x2E;
pub const kHIDUsage_AD_StatReady:    c_int = 0x2F;
pub const kHIDUsage_AD_ErrNotaloadablecharacter:    c_int = 0x30;
pub const kHIDUsage_AD_ErrFontdatacannotberead:    c_int = 0x31;
pub const kHIDUsage_AD_CursorPositionReport:    c_int = 0x32;
pub const kHIDUsage_AD_Row:    c_int = 0x33;
pub const kHIDUsage_AD_Column:    c_int = 0x34;
pub const kHIDUsage_AD_Rows:    c_int = 0x35;
pub const kHIDUsage_AD_Columns:    c_int = 0x36;
pub const kHIDUsage_AD_CursorPixelPositioning:    c_int = 0x37;
pub const kHIDUsage_AD_CursorMode:    c_int = 0x38;
pub const kHIDUsage_AD_CursorEnable:    c_int = 0x39;
pub const kHIDUsage_AD_CursorBlink:    c_int = 0x3A;
pub const kHIDUsage_AD_FontReport:    c_int = 0x3B;
pub const kHIDUsage_AD_FontData:    c_int = 0x3C;
pub const kHIDUsage_AD_CharacterWidth:    c_int = 0x3D;
pub const kHIDUsage_AD_CharacterHeight:    c_int = 0x3E;
pub const kHIDUsage_AD_CharacterSpacingHorizontal:    c_int = 0x3F;
pub const kHIDUsage_AD_CharacterSpacingVertical:    c_int = 0x40;
pub const kHIDUsage_AD_UnicodeCharacterSet:    c_int = 0x41;
// 0x42 - 0xFFFF Reserved
pub const kHIDUsage_AD_Reserved: c_int = 0xFFFF;

// Sensor Page (0x14)
pub const kHIDUsage_Snsr_Undefined:                            c_int = 0x00;
pub const kHIDUsage_Snsr_Sensor:                               c_int = 0x01;
// 0x02 - 0x0F Reserved
pub const kHIDUsage_Snsr_Biometric:                            c_int = 0x10;
pub const kHIDUsage_Snsr_Biometric_HumanPresence:              c_int = 0x11;
pub const kHIDUsage_Snsr_Biometric_HumanProximity:             c_int = 0x12;
pub const kHIDUsage_Snsr_Biometric_HumanTouch:                 c_int = 0x13;
// 0x14 - 0x1F Reserved
pub const kHIDUsage_Snsr_Electrical:                           c_int = 0x20;
pub const kHIDUsage_Snsr_Electrical_Capacitance:               c_int = 0x21;
pub const kHIDUsage_Snsr_Electrical_Current:                   c_int = 0x22;
pub const kHIDUsage_Snsr_Electrical_Power:                     c_int = 0x23;
pub const kHIDUsage_Snsr_Electrical_Inductance:                c_int = 0x24;
pub const kHIDUsage_Snsr_Electrical_Resistance:                c_int = 0x25;
pub const kHIDUsage_Snsr_Electrical_Voltage:                   c_int = 0x26;
pub const kHIDUsage_Snsr_Electrical_Potentiometer:             c_int = 0x27;
pub const kHIDUsage_Snsr_Electrical_Frequency:                 c_int = 0x28;
pub const kHIDUsage_Snsr_Electrical_Period:                    c_int = 0x29;
// 0x2A - 0x2F Reserved
pub const kHIDUsage_Snsr_Environmental:                        c_int = 0x30;
pub const kHIDUsage_Snsr_Environmental_AtmosphericPressure:    c_int = 0x31;
pub const kHIDUsage_Snsr_Environmental_Humidity:               c_int = 0x32;
pub const kHIDUsage_Snsr_Environmental_Temperature:            c_int = 0x33;
pub const kHIDUsage_Snsr_Environmental_WindDirection:          c_int = 0x34;
pub const kHIDUsage_Snsr_Environmental_WindSpeed:              c_int = 0x35;
// 0x36 - 0x3F Reserved
pub const kHIDUsage_Snsr_Light:                                c_int = 0x40;
pub const kHIDUsage_Snsr_Light_AmbientLight:                   c_int = 0x41;
pub const kHIDUsage_Snsr_Light_ConsumerInfrared:               c_int = 0x42;
// 0x43 - 0x4F Reserved
pub const kHIDUsage_Snsr_Location:                             c_int = 0x50;
pub const kHIDUsage_Snsr_Location_Broadcast:                   c_int = 0x51;
pub const kHIDUsage_Snsr_Location_DeadReckoning:               c_int = 0x52;
pub const kHIDUsage_Snsr_Location_GPS:                         c_int = 0x53;
pub const kHIDUsage_Snsr_Location_Lookup:                      c_int = 0x54;
pub const kHIDUsage_Snsr_Location_Other:                       c_int = 0x55;
pub const kHIDUsage_Snsr_Location_Static:                      c_int = 0x56;
pub const kHIDUsage_Snsr_Location_Triangulation:               c_int = 0x57;
// 0x58 - 0x5F Reserved
pub const kHIDUsage_Snsr_Mechanical:                           c_int = 0x60;
pub const kHIDUsage_Snsr_Mechanical_BooleanSwitch:             c_int = 0x61;
pub const kHIDUsage_Snsr_Mechanical_BooleanSwitchArray:        c_int = 0x62;
pub const kHIDUsage_Snsr_Mechanical_MultivalueSwitch:          c_int = 0x63;
pub const kHIDUsage_Snsr_Mechanical_Force:                     c_int = 0x64;
pub const kHIDUsage_Snsr_Mechanical_Pressure:                  c_int = 0x65;
pub const kHIDUsage_Snsr_Mechanical_Strain:                    c_int = 0x66;
pub const kHIDUsage_Snsr_Mechanical_Weight:                    c_int = 0x67;
pub const kHIDUsage_Snsr_Mechanical_HapticVibrator:            c_int = 0x68;
pub const kHIDUsage_Snsr_Mechanical_HallEffectSwitch:          c_int = 0x69;
// 0x6A - 0x6F Reserved
pub const kHIDUsage_Snsr_Motion:                               c_int = 0x70;
pub const kHIDUsage_Snsr_Motion_Accelerometer1D:               c_int = 0x71;
pub const kHIDUsage_Snsr_Motion_Accelerometer2D:               c_int = 0x72;
pub const kHIDUsage_Snsr_Motion_Accelerometer3D:               c_int = 0x73;
pub const kHIDUsage_Snsr_Motion_Gyrometer1D:                   c_int = 0x74;
pub const kHIDUsage_Snsr_Motion_Gyrometer2D:                   c_int = 0x75;
pub const kHIDUsage_Snsr_Motion_Gyrometer3D:                   c_int = 0x76;
pub const kHIDUsage_Snsr_Motion_MotionDetector:                c_int = 0x77;
pub const kHIDUsage_Snsr_Motion_Speedometer:                   c_int = 0x78;
pub const kHIDUsage_Snsr_Motion_Accelerometer:                 c_int = 0x79;
pub const kHIDUsage_Snsr_Motion_Gyrometer:                     c_int = 0x7A;
// 0x7B - 0x7F Reserved
pub const kHIDUsage_Snsr_Orientation:                          c_int = 0x80;
pub const kHIDUsage_Snsr_Orientation_Compass1D:                c_int = 0x81;
pub const kHIDUsage_Snsr_Orientation_Compass2D:                c_int = 0x82;
pub const kHIDUsage_Snsr_Orientation_Compass3D:                c_int = 0x83;
pub const kHIDUsage_Snsr_Orientation_Inclinometer1D:           c_int = 0x84;
pub const kHIDUsage_Snsr_Orientation_Inclinometer2D:           c_int = 0x85;
pub const kHIDUsage_Snsr_Orientation_Inclinometer3D:           c_int = 0x86;
pub const kHIDUsage_Snsr_Orientation_Distance1D:               c_int = 0x87;
pub const kHIDUsage_Snsr_Orientation_Distance2D:               c_int = 0x88;
pub const kHIDUsage_Snsr_Orientation_Distance3D:               c_int = 0x89;
pub const kHIDUsage_Snsr_Orientation_DeviceOrientation:        c_int = 0x8A;
pub const kHIDUsage_Snsr_Orientation_CompassD:                 c_int = 0x8B;
pub const kHIDUsage_Snsr_Orientation_InclinometerD:            c_int = 0x8C;
pub const kHIDUsage_Snsr_Orientation_DistanceD:                c_int = 0x8D;
// 0x8E - 0x8F Reserved
pub const kHIDUsage_Snsr_Scanner:                              c_int = 0x90;
pub const kHIDUsage_Snsr_Scanner_Barcode:                      c_int = 0x91;
pub const kHIDUsage_Snsr_Scanner_RFID:                         c_int = 0x92;
pub const kHIDUsage_Snsr_Scanner_NFC:                          c_int = 0x93;
// 0x94 - 0x9F Reserved
pub const kHIDUsage_Snsr_Time:                                 c_int = 0xA0;
pub const kHIDUsage_Snsr_Time_AlarmTimer:                      c_int = 0xA1;
pub const kHIDUsage_Snsr_Time_RealTimeClock:                   c_int = 0xA2;
// 0xA3 - 0xDF Reserved
pub const kHIDUsage_Snsr_Other:                                c_int = 0xE0;
pub const kHIDUsage_Snsr_Other_Custom:                         c_int = 0xE1;
pub const kHIDUsage_Snsr_Other_Generic:                        c_int = 0xE2;
pub const kHIDUsage_Snsr_Other_GenericEnumerator:              c_int = 0xE3;
// 0xE4 - 0xEF Reserved
// 0xF0 - 0xFF Vendor Reserved

// Common Sensor Type Data Fields
pub const kHIDUsage_Snsr_Modifier_None:                                c_int = 0x0;
pub const kHIDUsage_Snsr_Modifier_ChangeSensitivityAbsolute:           c_int = 0x1;
pub const kHIDUsage_Snsr_Modifier_Max:                                 c_int = 0x2;
pub const kHIDUsage_Snsr_Modifier_Min:                                 c_int = 0x3;
pub const kHIDUsage_Snsr_Modifier_Accuracy:                            c_int = 0x4;
pub const kHIDUsage_Snsr_Modifier_Resolution:                          c_int = 0x5;
pub const kHIDUsage_Snsr_Modifier_ThresholdHigh:                       c_int = 0x6;
pub const kHIDUsage_Snsr_Modifier_ThresholdLow:                        c_int = 0x7;
pub const kHIDUsage_Snsr_Modifier_CalibrationOffset:                   c_int = 0x8;
pub const kHIDUsage_Snsr_Modifier_CalibrationMultiplier:               c_int = 0x9;
pub const kHIDUsage_Snsr_Modifier_ReportInterval:                      c_int = 0xA;
pub const kHIDUsage_Snsr_Modifier_FrequencyMax:                        c_int = 0xB;
pub const kHIDUsage_Snsr_Modifier_PeriodMax:                           c_int = 0xC;
pub const kHIDUsage_Snsr_Modifier_ChangeSensitivityPercentRange:       c_int = 0xD;
pub const kHIDUsage_Snsr_Modifier_ChangeSensitivityPercentRelative:    c_int = 0xE;
pub const kHIDUsage_Snsr_Modifier_VendorDefined:                       c_int = 0xF;

// Event Usages
pub const kHIDUsage_Snsr_Event:                                        c_int = 0x0200;
pub const kHIDUsage_Snsr_Event_SensorState:                            c_int = 0x0201;
pub const kHIDUsage_Snsr_Event_SensorEvent:                            c_int = 0x0202;
// 0x0203 - 0x02FF Event Reserved

pub const kHIDUsage_Snsr_Event_SensorState_Undefined:                  c_int = 0x0800;
pub const kHIDUsage_Snsr_Event_SensorState_Ready:                      c_int = 0x0801;
pub const kHIDUsage_Snsr_Event_SensorState_NotAvailable:               c_int = 0x0802;
pub const kHIDUsage_Snsr_Event_SensorState_NoData:                     c_int = 0x0803;
pub const kHIDUsage_Snsr_Event_SensorState_Initializing:               c_int = 0x0804;
pub const kHIDUsage_Snsr_Event_SensorState_AccessDenied:               c_int = 0x0805;
pub const kHIDUsage_Snsr_Event_SensorState_Error:                      c_int = 0x0806;
// 0x0807 - 0x080F Reserved

pub const kHIDUsage_Snsr_Event_SensorEvent_Unknown:                    c_int = 0x0810;
pub const kHIDUsage_Snsr_Event_SensorEvent_StateChanged:               c_int = 0x0811;
pub const kHIDUsage_Snsr_Event_SensorEvent_PropertyChanged:            c_int = 0x0812;
pub const kHIDUsage_Snsr_Event_SensorEvent_DataUpdated:                c_int = 0x0813;
pub const kHIDUsage_Snsr_Event_SensorEvent_PollResponse:               c_int = 0x0814;
pub const kHIDUsage_Snsr_Event_SensorEvent_ChangeSensitivity:          c_int = 0x0815;
pub const kHIDUsage_Snsr_Event_SensorEvent_RangeMaxReached:            c_int = 0x0816;
pub const kHIDUsage_Snsr_Event_SensorEvent_RangeMinReached:            c_int = 0x0817;
pub const kHIDUsage_Snsr_Event_SensorEvent_HighThresholdCrossUp:       c_int = 0x0818;
pub const kHIDUsage_Snsr_Event_SensorEvent_HighThresholdCrossDown:     c_int = 0x0819;
pub const kHIDUsage_Snsr_Event_SensorEvent_LowThresholdCrossUp:        c_int = 0x081A;
pub const kHIDUsage_Snsr_Event_SensorEvent_LowThresholdCrossDown:      c_int = 0x081B;
pub const kHIDUsage_Snsr_Event_SensorEvent_ZeroThresholdCrossUp:       c_int = 0x081C;
pub const kHIDUsage_Snsr_Event_SensorEvent_ZeroThresholdCrossDown:     c_int = 0x081D;
pub const kHIDUsage_Snsr_Event_SensorEvent_PeriodExceeded:             c_int = 0x081E;
pub const kHIDUsage_Snsr_Event_SensorEvent_FrequencyExceeded:          c_int = 0x081F;
pub const kHIDUsage_Snsr_Event_SensorEvent_ComplexTrigger:             c_int = 0x0820;
// 0x0821 - 0x082F Reserved

// Property Usages
pub const kHIDUsage_Snsr_Property:                                     c_int = 0x0300;
pub const kHIDUsage_Snsr_Property_FriendlyName:                        c_int = 0x0301;
pub const kHIDUsage_Snsr_Property_PersistentUniqueID:                  c_int = 0x0302;
pub const kHIDUsage_Snsr_Property_SensorStatus:                        c_int = 0x0303;
pub const kHIDUsage_Snsr_Property_MinimumReportInterval:               c_int = 0x0304;
pub const kHIDUsage_Snsr_Property_Manufacturer:                        c_int = 0x0305;
pub const kHIDUsage_Snsr_Property_Model:                               c_int = 0x0306;
pub const kHIDUsage_Snsr_Property_SerialNumber:                        c_int = 0x0307;
pub const kHIDUsage_Snsr_Property_Description:                         c_int = 0x0308;
pub const kHIDUsage_Snsr_Property_ConnectionType:                      c_int = 0x0309;
pub const kHIDUsage_Snsr_Property_DevicePath:                          c_int = 0x030A;
pub const kHIDUsage_Snsr_Property_HardwareRevision:                    c_int = 0x030B;
pub const kHIDUsage_Snsr_Property_FirmwareVersion:                     c_int = 0x030C;
pub const kHIDUsage_Snsr_Property_ReleaseData:                         c_int = 0x030D;
pub const kHIDUsage_Snsr_Property_ReportInterval:                      c_int = 0x030E;
pub const kHIDUsage_Snsr_Property_ChangeSensitivityAbsolute:           c_int = 0x030F;
pub const kHIDUsage_Snsr_Property_ChangeSensitivityPercentRange:       c_int = 0x0310;
pub const kHIDUsage_Snsr_Property_ChangeSensitivityPercentRelative:    c_int = 0x0311;
pub const kHIDUsage_Snsr_Property_Accuracy:                            c_int = 0x0312;
pub const kHIDUsage_Snsr_Property_Resolution:                          c_int = 0x0313;
pub const kHIDUsage_Snsr_Property_Maximum:                             c_int = 0x0314;
pub const kHIDUsage_Snsr_Property_Minimum:                             c_int = 0x0315;
pub const kHIDUsage_Snsr_Property_ReportingState:                      c_int = 0x0316;
pub const kHIDUsage_Snsr_Property_SamplingRate:                        c_int = 0x0317;
pub const kHIDUsage_Snsr_Property_ResponseCurve:                       c_int = 0x0318;
pub const kHIDUsage_Snsr_Property_PowerState:                          c_int = 0x0319;
// 0x031A - 0x03FF Reserved

pub const kHIDUsage_Snsr_Property_ConnectionType_Integrated:           c_int = 0x0830;
pub const kHIDUsage_Snsr_Property_ConnectionType_Attached:             c_int = 0x0831;
pub const kHIDUsage_Snsr_Property_ConnectionType_External:             c_int = 0x0832;
// 0x0833 - 0x083F Reserved
pub const kHIDUsage_Snsr_Property_ReportingState_NoEvents:             c_int = 0x0840;
pub const kHIDUsage_Snsr_Property_ReportingState_AllEvents:            c_int = 0x0841;
pub const kHIDUsage_Snsr_Property_ReportingState_ThresholdEvents:      c_int = 0x0842;
pub const kHIDUsage_Snsr_Property_ReportingState_WakeNoEvents:         c_int = 0x0843;
pub const kHIDUsage_Snsr_Property_ReportingState_WakeAllEvents:        c_int = 0x0844;
pub const kHIDUsage_Snsr_Property_ReportingState_WakeThresholdEvents:  c_int = 0x0845;
// 0x0846 - 0x084F Reserved
pub const kHIDUsage_Snsr_Property_PowerState_Undefined:                c_int = 0x0850;
pub const kHIDUsage_Snsr_Property_PowerState_D0_FullPower:             c_int = 0x0851;
pub const kHIDUsage_Snsr_Property_PowerState_D1_LowPower:              c_int = 0x0852;
pub const kHIDUsage_Snsr_Property_PowerState_D2_Standby:               c_int = 0x0853;
pub const kHIDUsage_Snsr_Property_PowerState_D3_Sleep:                 c_int = 0x0854;
pub const kHIDUsage_Snsr_Property_PowerState_D4_PowerOff:              c_int = 0x0855;
// 0x0855 - 0x085F Reserved

// Power Device Page (0x84)
pub const kHIDUsage_PD_Undefined: c_int = 0x00;
pub const kHIDUsage_PD_iName: c_int = 0x01;
pub const kHIDUsage_PD_PresentStatus: c_int = 0x02;
pub const kHIDUsage_PD_ChangedStatus: c_int = 0x03;
pub const kHIDUsage_PD_UPS: c_int = 0x04;
pub const kHIDUsage_PD_PowerSupply: c_int = 0x05;
// Reserved 0x06 - 0x0F
pub const kHIDUsage_PD_BatterySystem: c_int = 0x10;
pub const kHIDUsage_PD_BatterySystemID: c_int = 0x11;
pub const kHIDUsage_PD_Battery: c_int = 0x12;
pub const kHIDUsage_PD_BatteryID: c_int = 0x13;
pub const kHIDUsage_PD_Charger: c_int = 0x14;
pub const kHIDUsage_PD_ChargerID: c_int = 0x15;
pub const kHIDUsage_PD_PowerConverter: c_int = 0x16;
pub const kHIDUsage_PD_PowerConverterID: c_int = 0x17;
pub const kHIDUsage_PD_OutletSystem: c_int = 0x18;
pub const kHIDUsage_PD_OutletSystemID: c_int = 0x19;
pub const kHIDUsage_PD_Input: c_int = 0x1A;
pub const kHIDUsage_PD_InputID: c_int = 0x1B;
pub const kHIDUsage_PD_Output: c_int = 0x1C;
pub const kHIDUsage_PD_OutputID: c_int = 0x1D;
pub const kHIDUsage_PD_Flow: c_int = 0x1E;
pub const kHIDUsage_PD_FlowID: c_int = 0x1F;
pub const kHIDUsage_PD_Outlet: c_int = 0x20;
pub const kHIDUsage_PD_OutletID: c_int = 0x21;
pub const kHIDUsage_PD_Gang: c_int = 0x22;
pub const kHIDUsage_PD_GangID: c_int = 0x23;
pub const kHIDUsage_PD_PowerSummary: c_int = 0x24;
pub const kHIDUsage_PD_PowerSummaryID: c_int = 0x25;
// Reserved 0x26 - 0x2F
pub const kHIDUsage_PD_Voltage: c_int = 0x30;
pub const kHIDUsage_PD_Current: c_int = 0x31;
pub const kHIDUsage_PD_Frequency: c_int = 0x32;
pub const kHIDUsage_PD_ApparentPower: c_int = 0x33;
pub const kHIDUsage_PD_ActivePower: c_int = 0x34;
pub const kHIDUsage_PD_PercentLoad: c_int = 0x35;
pub const kHIDUsage_PD_Temperature: c_int = 0x36;
pub const kHIDUsage_PD_Humidity: c_int = 0x37;
pub const kHIDUsage_PD_BadCount: c_int = 0x38;
// Reserved 0x39 - 0x3F
pub const kHIDUsage_PD_ConfigVoltage: c_int = 0x40;
pub const kHIDUsage_PD_ConfigCurrent: c_int = 0x41;
pub const kHIDUsage_PD_ConfigFrequency: c_int = 0x42;
pub const kHIDUsage_PD_ConfigApparentPower: c_int = 0x43;
pub const kHIDUsage_PD_ConfigActivePower: c_int = 0x44;
pub const kHIDUsage_PD_ConfigPercentLoad: c_int = 0x45;
pub const kHIDUsage_PD_ConfigTemperature: c_int = 0x46;
pub const kHIDUsage_PD_ConfigHumidity: c_int = 0x47;
// Reserved 0x48 - 0x4F
pub const kHIDUsage_PD_SwitchOnControl: c_int = 0x50;
pub const kHIDUsage_PD_SwitchOffControl: c_int = 0x51;
pub const kHIDUsage_PD_ToggleControl: c_int = 0x52;
pub const kHIDUsage_PD_LowVoltageTransfer: c_int = 0x53;
pub const kHIDUsage_PD_HighVoltageTransfer: c_int = 0x54;
pub const kHIDUsage_PD_DelayBeforeReboot: c_int = 0x55;
pub const kHIDUsage_PD_DelayBeforeStartup: c_int = 0x56;
pub const kHIDUsage_PD_DelayBeforeShutdown: c_int = 0x57;
pub const kHIDUsage_PD_Test: c_int = 0x58;
pub const kHIDUsage_PD_ModuleReset: c_int = 0x59;
pub const kHIDUsage_PD_AudibleAlarmControl: c_int = 0x5A;
// Reserved 0x5B - 0x5F
pub const kHIDUsage_PD_Present: c_int = 0x60;
pub const kHIDUsage_PD_Good: c_int = 0x61;
pub const kHIDUsage_PD_InternalFailure: c_int = 0x62;
pub const kHIDUsage_PD_VoltageOutOfRange: c_int = 0x63;
pub const kHIDUsage_PD_FrequencyOutOfRange: c_int = 0x64;
pub const kHIDUsage_PD_Overload: c_int = 0x65;
pub const kHIDUsage_PD_OverCharged: c_int = 0x66;
pub const kHIDUsage_PD_OverTemperature: c_int = 0x67;
pub const kHIDUsage_PD_ShutdownRequested: c_int = 0x68;
pub const kHIDUsage_PD_ShutdownImminent: c_int = 0x69;
// Reserved 0x6A
pub const kHIDUsage_PD_SwitchOnOff: c_int = 0x6B;
pub const kHIDUsage_PD_Switchable: c_int = 0x6C;
pub const kHIDUsage_PD_Used: c_int = 0x6D;
pub const kHIDUsage_PD_Boost: c_int = 0x6E;
pub const kHIDUsage_PD_Buck: c_int = 0x6F;
pub const kHIDUsage_PD_Initialized: c_int = 0x70;
pub const kHIDUsage_PD_Tested: c_int = 0x71;
pub const kHIDUsage_PD_AwaitingPower: c_int = 0x72;
pub const kHIDUsage_PD_CommunicationLost: c_int = 0x73;
// Reserved 0x74 - 0xFC
pub const kHIDUsage_PD_iManufacturer: c_int = 0xFD;
pub const kHIDUsage_PD_iProduct: c_int = 0xFE;
pub const kHIDUsage_PD_iserialNumber: c_int = 0xFF;

// Battery System Page (x85)
pub const kHIDUsage_BS_Undefined: c_int = 0x00;
pub const kHIDUsage_BS_SMBBatteryMode: c_int = 0x01;
pub const kHIDUsage_BS_SMBBatteryStatus: c_int = 0x02;
pub const kHIDUsage_BS_SMBAlarmWarning: c_int = 0x03;
pub const kHIDUsage_BS_SMBChargerMode: c_int = 0x04;
pub const kHIDUsage_BS_SMBChargerStatus: c_int = 0x05;
pub const kHIDUsage_BS_SMBChargerSpecInfo: c_int = 0x06;
pub const kHIDUsage_BS_SMBSelectorState: c_int = 0x07;
pub const kHIDUsage_BS_SMBSelectorPresets: c_int = 0x08;
pub const kHIDUsage_BS_SMBSelectorInfo: c_int = 0x09;
// Reserved 0x0A - 0x0F
pub const kHIDUsage_BS_OptionalMfgFunction1: c_int = 0x10;
pub const kHIDUsage_BS_OptionalMfgFunction2: c_int = 0x11;
pub const kHIDUsage_BS_OptionalMfgFunction3: c_int = 0x12;
pub const kHIDUsage_BS_OptionalMfgFunction4: c_int = 0x13;
pub const kHIDUsage_BS_OptionalMfgFunction5: c_int = 0x14;
pub const kHIDUsage_BS_ConnectionToSMBus: c_int = 0x15;
pub const kHIDUsage_BS_OutputConnection: c_int = 0x16;
pub const kHIDUsage_BS_ChargerConnection: c_int = 0x17;
pub const kHIDUsage_BS_BatteryInsertion: c_int = 0x18;
pub const kHIDUsage_BS_Usenext: c_int = 0x19;
pub const kHIDUsage_BS_OKToUse: c_int = 0x1A;
pub const kHIDUsage_BS_BatterySupported: c_int = 0x1B;
pub const kHIDUsage_BS_SelectorRevision: c_int = 0x1C;
pub const kHIDUsage_BS_ChargingIndicator: c_int = 0x1D;
// Reserved 0x1E - 0x27
pub const kHIDUsage_BS_ManufacturerAccess: c_int = 0x28;
pub const kHIDUsage_BS_RemainingCapacityLimit: c_int = 0x29;
pub const kHIDUsage_BS_RemainingTimeLimit: c_int = 0x2A;
pub const kHIDUsage_BS_AtRate: c_int = 0x2B;
pub const kHIDUsage_BS_CapacityMode: c_int = 0x2C;
pub const kHIDUsage_BS_BroadcastToCharger: c_int = 0x2D;
pub const kHIDUsage_BS_PrimaryBattery: c_int = 0x2E;
pub const kHIDUsage_BS_ChargeController: c_int = 0x2F;
// Reserved 0x30 - 0x3F
pub const kHIDUsage_BS_TerminateCharge: c_int = 0x40;
pub const kHIDUsage_BS_TerminateDischarge: c_int = 0x41;
pub const kHIDUsage_BS_BelowRemainingCapacityLimit: c_int = 0x42;
pub const kHIDUsage_BS_RemainingTimeLimitExpired: c_int = 0x43;
pub const kHIDUsage_BS_Charging: c_int = 0x44;
pub const kHIDUsage_BS_Discharging: c_int = 0x45;
pub const kHIDUsage_BS_FullyCharged: c_int = 0x46;
pub const kHIDUsage_BS_FullyDischarged: c_int = 0x47;
pub const kHIDUsage_BS_ConditioningFlag: c_int = 0x48;
pub const kHIDUsage_BS_AtRateOK: c_int = 0x49;
pub const kHIDUsage_BS_SMBErrorCode: c_int = 0x4A;
pub const kHIDUsage_BS_NeedReplacement: c_int = 0x4B;
// Reserved 0x4C - 0x5F
pub const kHIDUsage_BS_AtRateTimeToFull: c_int = 0x60;
pub const kHIDUsage_BS_AtRateTimeToEmpty: c_int = 0x61;
pub const kHIDUsage_BS_AverageCurrent: c_int = 0x62;
pub const kHIDUsage_BS_Maxerror: c_int = 0x63;
pub const kHIDUsage_BS_RelativeStateOfCharge: c_int = 0x64;
pub const kHIDUsage_BS_AbsoluteStateOfCharge: c_int = 0x65;
pub const kHIDUsage_BS_RemainingCapacity: c_int = 0x66;
pub const kHIDUsage_BS_FullChargeCapacity: c_int = 0x67;
pub const kHIDUsage_BS_RunTimeToEmpty: c_int = 0x68;
pub const kHIDUsage_BS_AverageTimeToEmpty: c_int = 0x69;
pub const kHIDUsage_BS_AverageTimeToFull: c_int = 0x6A;
pub const kHIDUsage_BS_CycleCount: c_int = 0x6B;
// Reserved 0x6C - 0x7F
pub const kHIDUsage_BS_BattPackModelLevel: c_int = 0x80;
pub const kHIDUsage_BS_InternalChargeController: c_int = 0x81;
pub const kHIDUsage_BS_PrimaryBatterySupport: c_int = 0x82;
pub const kHIDUsage_BS_DesignCapacity: c_int = 0x83;
pub const kHIDUsage_BS_SpecificationInfo: c_int = 0x84;
pub const kHIDUsage_BS_ManufacturerDate: c_int = 0x85;
pub const kHIDUsage_BS_SerialNumber: c_int = 0x86;
pub const kHIDUsage_BS_iManufacturerName: c_int = 0x87;
pub const kHIDUsage_BS_iDevicename: c_int = 0x88;
pub const kHIDUsage_BS_iDeviceChemistry: c_int = 0x89;
pub const kHIDUsage_BS_ManufacturerData: c_int = 0x8A;
pub const kHIDUsage_BS_Rechargable: c_int = 0x8B;
pub const kHIDUsage_BS_WarningCapacityLimit: c_int = 0x8C;
pub const kHIDUsage_BS_CapacityGranularity1: c_int = 0x8D;
pub const kHIDUsage_BS_CapacityGranularity2: c_int = 0x8E;
pub const kHIDUsage_BS_iOEMInformation: c_int = 0x8F;
// Reserved 0x90 - 0xBF
pub const kHIDUsage_BS_InhibitCharge: c_int = 0xC0;
pub const kHIDUsage_BS_EnablePolling: c_int = 0xC1;
pub const kHIDUsage_BS_ResetToZero: c_int = 0xC2;
// Reserved 0xC3 - 0xCF
pub const kHIDUsage_BS_ACPresent: c_int = 0xD0;
pub const kHIDUsage_BS_BatteryPresent: c_int = 0xD1;
pub const kHIDUsage_BS_PowerFail: c_int = 0xD2;
pub const kHIDUsage_BS_AlarmInhibited: c_int = 0xD3;
pub const kHIDUsage_BS_ThermistorUnderRange: c_int = 0xD4;
pub const kHIDUsage_BS_ThermistorHot: c_int = 0xD5;
pub const kHIDUsage_BS_ThermistorCold: c_int = 0xD6;
pub const kHIDUsage_BS_ThermistorOverRange: c_int = 0xD7;
pub const kHIDUsage_BS_VoltageOutOfRange: c_int = 0xD8;
pub const kHIDUsage_BS_CurrentOutOfRange: c_int = 0xD9;
pub const kHIDUsage_BS_CurrentNotRegulated: c_int = 0xDA;
pub const kHIDUsage_BS_VoltageNotRegulated: c_int = 0xDB;
pub const kHIDUsage_BS_MasterMode: c_int = 0xDC;
// Reserved 0xDD - 0xEF
pub const kHIDUsage_BS_ChargerSelectorSupport: c_int = 0xF0;
pub const kHIDUsage_BS_ChargerSpec: c_int = 0xF1;
pub const kHIDUsage_BS_Level2: c_int = 0xF2;
pub const kHIDUsage_BS_Level3: c_int = 0xF3;
// Reserved 0xF2 - 0xFF

// Bar Code Scanner Page (0x8C)
pub const kHIDUsage_BCS_Undefined: c_int = 0x00;
pub const kHIDUsage_BCS_BadgeReader: c_int = 0x01;
pub const kHIDUsage_BCS_BarCodeScanner: c_int = 0x02;
pub const kHIDUsage_BCS_DumbBarCodeScanner: c_int = 0x03;
pub const kHIDUsage_BCS_CordlessScannerBase: c_int = 0x04;
pub const kHIDUsage_BCS_BarCodeScannerCradle: c_int = 0x05;
// Reserved 0x06 - 0x0F
pub const kHIDUsage_BCS_AttributeReport: c_int = 0x10;
pub const kHIDUsage_BCS_SettingsReport: c_int = 0x11;
pub const kHIDUsage_BCS_ScannedDataReport: c_int = 0x12;
pub const kHIDUsage_BCS_RawScannedDataReport: c_int = 0x13;
pub const kHIDUsage_BCS_TriggerReport: c_int = 0x14;
pub const kHIDUsage_BCS_StatusReport: c_int = 0x15;
pub const kHIDUsage_BCS_UPC_EANControlReport: c_int = 0x16;
pub const kHIDUsage_BCS_EAN2_3LabelControlReport: c_int = 0x17;
pub const kHIDUsage_BCS_Code39ControlReport: c_int = 0x18;
pub const kHIDUsage_BCS_Interleaved2of5ControlReport: c_int = 0x19;
pub const kHIDUsage_BCS_Standard2of5ControlReport: c_int = 0x1A;
pub const kHIDUsage_BCS_MSIPlesseyControlReport: c_int = 0x1B;
pub const kHIDUsage_BCS_CodabarControlReport: c_int = 0x1C;
pub const kHIDUsage_BCS_Code128ControlReport: c_int = 0x1D;
pub const kHIDUsage_BCS_Misc1DControlReport: c_int = 0x1E;
pub const kHIDUsage_BCS_2DControlReport: c_int = 0x1F;
// Reserved 0x20 - 0x2F
pub const kHIDUsage_BCS_Aiming_PointerMide: c_int = 0x30;
pub const kHIDUsage_BCS_BarCodePresentSensor: c_int = 0x31;
pub const kHIDUsage_BCS_Class1ALaser: c_int = 0x32;
pub const kHIDUsage_BCS_Class2Laser: c_int = 0x33;
pub const kHIDUsage_BCS_HeaterPresent: c_int = 0x34;
pub const kHIDUsage_BCS_ContactScanner: c_int = 0x35;
pub const kHIDUsage_BCS_ElectronicArticleSurveillanceNotification: c_int = 0x36;
pub const kHIDUsage_BCS_ConstantElectronicArticleSurveillance: c_int = 0x37;
pub const kHIDUsage_BCS_ErrorIndication: c_int = 0x38;
pub const kHIDUsage_BCS_FixedBeeper: c_int = 0x39;
pub const kHIDUsage_BCS_GoodDecodeIndication: c_int = 0x3A;
pub const kHIDUsage_BCS_HandsFreeScanning: c_int = 0x3B;
pub const kHIDUsage_BCS_IntrinsicallySafe: c_int = 0x3C;
pub const kHIDUsage_BCS_KlasseEinsLaser: c_int = 0x3D;
pub const kHIDUsage_BCS_LongRangeScanner: c_int = 0x3E;
pub const kHIDUsage_BCS_MirrorSpeedControl: c_int = 0x3F;
pub const kHIDUsage_BCS_NotOnFileIndication: c_int = 0x40;
pub const kHIDUsage_BCS_ProgrammableBeeper: c_int = 0x41;
pub const kHIDUsage_BCS_Triggerless: c_int = 0x42;
pub const kHIDUsage_BCS_Wand: c_int = 0x43;
pub const kHIDUsage_BCS_WaterResistant: c_int = 0x44;
pub const kHIDUsage_BCS_MultiRangeScanner: c_int = 0x45;
pub const kHIDUsage_BCS_ProximitySensor: c_int = 0x46;
// Reserved 0x47 - 0x4C
pub const kHIDUsage_BCS_FragmentDecoding: c_int = 0x4D;
pub const kHIDUsage_BCS_ScannerReadConfidence: c_int = 0x4E;
pub const kHIDUsage_BCS_DataPrefix: c_int = 0x4F;
pub const kHIDUsage_BCS_PrefixAIMI: c_int = 0x50;
pub const kHIDUsage_BCS_PrefixNone: c_int = 0x51;
pub const kHIDUsage_BCS_PrefixProprietary: c_int = 0x52;
// Reserved 0x53 - 0x54
pub const kHIDUsage_BCS_ActiveTime: c_int = 0x55;
pub const kHIDUsage_BCS_AimingLaserPattern: c_int = 0x56;
pub const kHIDUsage_BCS_BarCodePresent: c_int = 0x57;
pub const kHIDUsage_BCS_BeeperState: c_int = 0x58;
pub const kHIDUsage_BCS_LaserOnTime: c_int = 0x59;
pub const kHIDUsage_BCS_LaserState: c_int = 0x5A;
pub const kHIDUsage_BCS_LockoutTime: c_int = 0x5B;
pub const kHIDUsage_BCS_MotorState: c_int = 0x5C;
pub const kHIDUsage_BCS_MotorTimeout: c_int = 0x5D;
pub const kHIDUsage_BCS_PowerOnResetScanner: c_int = 0x5E;
pub const kHIDUsage_BCS_PreventReadOfBarcodes: c_int = 0x5F;
pub const kHIDUsage_BCS_InitiateBarcodeRead: c_int = 0x60;
pub const kHIDUsage_BCS_TriggerState: c_int = 0x61;
pub const kHIDUsage_BCS_TriggerMode: c_int = 0x62;
pub const kHIDUsage_BCS_TriggerModeBlinkingLaserOn: c_int = 0x63;
pub const kHIDUsage_BCS_TriggerModeContinuousLaserOn: c_int = 0x64;
pub const kHIDUsage_BCS_TriggerModeLaserOnWhilePulled: c_int = 0x65;
pub const kHIDUsage_BCS_TriggerModeLaserStaysOnAfterTriggerRelease: c_int = 0x66;
// Reserved 0x67 - 0x6C
pub const kHIDUsage_BCS_CommitParametersToNVM: c_int = 0x6D;
pub const kHIDUsage_BCS_ParameterScanning: c_int = 0x6E;
pub const kHIDUsage_BCS_ParametersChanged: c_int = 0x6F;
pub const kHIDUsage_BCS_SetParameterDefaultValues: c_int = 0x70;
// Reserved 0x71 - 0x74
pub const kHIDUsage_BCS_ScannerInCradle: c_int = 0x75;
pub const kHIDUsage_BCS_ScannerInRange: c_int = 0x76;
// Reserved 0x77 - 0x79
pub const kHIDUsage_BCS_AimDuration: c_int = 0x7A;
pub const kHIDUsage_BCS_GoodReadLampDuration: c_int = 0x7B;
pub const kHIDUsage_BCS_GoodReadLampIntensity: c_int = 0x7C;
pub const kHIDUsage_BCS_GoodReadLED: c_int = 0x7D;
pub const kHIDUsage_BCS_GoodReadToneFrequency: c_int = 0x7E;
pub const kHIDUsage_BCS_GoodReadToneLength: c_int = 0x7F;
pub const kHIDUsage_BCS_GoodReadToneVolume: c_int = 0x80;
// Reserved 0x81
pub const kHIDUsage_BCS_NoReadMessage: c_int = 0x82;
pub const kHIDUsage_BCS_NotOnFileVolume: c_int = 0x83;
pub const kHIDUsage_BCS_PowerupBeep: c_int = 0x84;
pub const kHIDUsage_BCS_SoundErrorBeep: c_int = 0x85;
pub const kHIDUsage_BCS_SoundGoodReadBeep: c_int = 0x86;
pub const kHIDUsage_BCS_SoundNotOnFileBeep: c_int = 0x87;
pub const kHIDUsage_BCS_GoodReadWhenToWrite: c_int = 0x88;
pub const kHIDUsage_BCS_GRWTIAfterDecode: c_int = 0x89;
pub const kHIDUsage_BCS_GRWTIBeep_LampAfterTransmit: c_int = 0x8A;
pub const kHIDUsage_BCS_GRWTINoBeep_LampUseAtAll: c_int = 0x8B;
// Reserved 0x8C - 0x90
pub const kHIDUsage_BCS_BooklandEAN: c_int = 0x91;
pub const kHIDUsage_BCS_ConvertEAN8To13Type: c_int = 0x92;
pub const kHIDUsage_BCS_ConvertUPCAToEAN_13: c_int = 0x93;
pub const kHIDUsage_BCS_ConvertUPC_EToA: c_int = 0x94;
pub const kHIDUsage_BCS_EAN_13: c_int = 0x95;
pub const kHIDUsage_BCS_EAN_8: c_int = 0x96;
pub const kHIDUsage_BCS_EAN_99_128_Mandatory: c_int = 0x97;
pub const kHIDUsage_BCS_EAN_99_P5_128_Optional: c_int = 0x98;
// Reserved 0x99
pub const kHIDUsage_BCS_UPC_EAN: c_int = 0x9A;
pub const kHIDUsage_BCS_UPC_EANCouponCode: c_int = 0x9B;
pub const kHIDUsage_BCS_UPC_EANPeriodicals: c_int = 0x9C;
pub const kHIDUsage_BCS_UPC_A: c_int = 0x9D;
pub const kHIDUsage_BCS_UPC_AWith128Mandatory: c_int = 0x9E;
pub const kHIDUsage_BCS_UPC_AWith128Optical: c_int = 0x9F;
pub const kHIDUsage_BCS_UPC_AWithP5Optional: c_int = 0xA0;
pub const kHIDUsage_BCS_UPC_E: c_int = 0xA1;
pub const kHIDUsage_BCS_UPC_E1: c_int = 0xA2;
// Reserved 0xA3 - 0xA8
pub const kHIDUsage_BCS_Periodical: c_int = 0xA9;
pub const kHIDUsage_BCS_PeriodicalAutoDiscriminatePlus2: c_int = 0xAA;
pub const kHIDUsage_BCS_PeriodicalOnlyDecodeWithPlus2: c_int = 0xAB;
pub const kHIDUsage_BCS_PeriodicalIgnorePlus2: c_int = 0xAC;
pub const kHIDUsage_BCS_PeriodicalAutoDiscriminatePlus5: c_int = 0xAD;
pub const kHIDUsage_BCS_PeriodicalOnlyDecodeWithPlus5: c_int = 0xAE;
pub const kHIDUsage_BCS_PeriodicalIgnorePlus5: c_int = 0xAF;
pub const kHIDUsage_BCS_Check: c_int = 0xB0;
pub const kHIDUsage_BCS_CheckDisablePrice: c_int = 0xB1;
pub const kHIDUsage_BCS_CheckEnable4DigitPrice: c_int = 0xB2;
pub const kHIDUsage_BCS_CheckEnable5DigitPrice: c_int = 0xB3;
pub const kHIDUsage_BCS_CheckEnableEuropean4DigitPrice: c_int = 0xB4;
pub const kHIDUsage_BCS_CheckEnableEuropean5DigitPrice: c_int = 0xB5;
// Reserved 0xB6
pub const kHIDUsage_BCS_EANTwoLabel: c_int = 0xB7;
pub const kHIDUsage_BCS_EANThreeLabel: c_int = 0xB8;
pub const kHIDUsage_BCS_EAN8FlagDigit1: c_int = 0xB9;
pub const kHIDUsage_BCS_EAN8FlagDigit2: c_int = 0xBA;
pub const kHIDUsage_BCS_EAN8FlagDigit3: c_int = 0xBB;
pub const kHIDUsage_BCS_EAN13FlagDigit1: c_int = 0xBC;
pub const kHIDUsage_BCS_EAN13FlagDigit2: c_int = 0xBD;
pub const kHIDUsage_BCS_EAN13FlagDigit3: c_int = 0xBE;
pub const kHIDUsage_BCS_AddEAN2_3LabelDefinition: c_int = 0xBF;
pub const kHIDUsage_BCS_ClearAllEAN2_3LabelDefinitions: c_int = 0xC0;
// Reserved 0xC1 - 0xC2
pub const kHIDUsage_BCS_Codabar: c_int = 0xC3;
pub const kHIDUsage_BCS_Code128: c_int = 0xC4;
// Reserved 0xC5 - 0xC6
pub const kHIDUsage_BCS_Code39: c_int = 0xC7;
pub const kHIDUsage_BCS_Code93: c_int = 0xC8;
pub const kHIDUsage_BCS_FullASCIIConversion: c_int = 0xC9;
pub const kHIDUsage_BCS_Interleaved2of5: c_int = 0xCA;
pub const kHIDUsage_BCS_ItalianPharmacyCode: c_int = 0xCB;
pub const kHIDUsage_BCS_MSI_Plessey: c_int = 0xCC;
pub const kHIDUsage_BCS_Standard2of5IATA: c_int = 0xCD;
pub const kHIDUsage_BCS_Standard2of5: c_int = 0xCE;
// Reserved 0xCF - 0xD2
pub const kHIDUsage_BCS_TransmitStart_Stop: c_int = 0xD3;
pub const kHIDUsage_BCS_TriOptic: c_int = 0xD4;
pub const kHIDUsage_BCS_UCC_EAN_128: c_int = 0xD5;
pub const kHIDUsage_BCS_CheckDigit: c_int = 0xD6;
pub const kHIDUsage_BCS_CheckDigitDisable: c_int = 0xD7;
pub const kHIDUsage_BCS_CheckDigitEnableInterleaved2of5OPCC: c_int = 0xD8;
pub const kHIDUsage_BCS_CheckDigitEnableInterleaved2of5USS: c_int = 0xD9;
pub const kHIDUsage_BCS_CheckDigitEnableStandard2of5OPCC: c_int = 0xD8;
pub const kHIDUsage_BCS_CheckDigitEnableStandard2of5USS: c_int = 0xD9;
pub const kHIDUsage_BCS_CheckDigitEnableOneMSIPlessey: c_int = 0xDC;
pub const kHIDUsage_BCS_CheckDigitEnableTwoMSIPlessey: c_int = 0xDD;
pub const kHIDUsage_BCS_CheckDigitCodabarEnable: c_int = 0xDE;
pub const kHIDUsage_BCS_CheckDigitCode99Enable: c_int = 0xDF;
// Reserved 0xE0 - 0xEF
pub const kHIDUsage_BCS_TransmitCheckDigit: c_int = 0xF0;
pub const kHIDUsage_BCS_DisableCheckDigitTransmit: c_int = 0xF1;
pub const kHIDUsage_BCS_EnableCheckDigitTransmit: c_int = 0xF2;
// Reserved 0xF3 - 0xFA
pub const kHIDUsage_BCS_SymbologyIdentifier1: c_int = 0xFB;
pub const kHIDUsage_BCS_SymbologyIdentifier2: c_int = 0xFC;
pub const kHIDUsage_BCS_SymbologyIdentifier3: c_int = 0xFD;
pub const kHIDUsage_BCS_DecodedData: c_int = 0xFE;
pub const kHIDUsage_BCS_DecodeDataContinued: c_int = 0xFF;
pub const kHIDUsage_BCS_BarSpaceData: c_int = 0x100;
pub const kHIDUsage_BCS_ScannerDataAccuracy: c_int = 0x101;
pub const kHIDUsage_BCS_RawDataPolarity: c_int = 0x102;
pub const kHIDUsage_BCS_PolarityInvertedBarCode: c_int = 0x103;
pub const kHIDUsage_BCS_PolarityNormalBarCode: c_int = 0x103;
// Reserved 0x105
pub const kHIDUsage_BCS_MinimumLengthToDecode: c_int = 0x106;
pub const kHIDUsage_BCS_MaximumLengthToDecode: c_int = 0x107;
pub const kHIDUsage_BCS_FirstDiscreteLengthToDecode: c_int = 0x108;
pub const kHIDUsage_BCS_SecondDiscreteLengthToDecode: c_int = 0x109;
pub const kHIDUsage_BCS_DataLengthMethod: c_int = 0x10A;
pub const kHIDUsage_BCS_DLMethodReadAny: c_int = 0x10B;
pub const kHIDUsage_BCS_DLMethodCheckInRange: c_int = 0x10C;
pub const kHIDUsage_BCS_DLMethodCheckForDiscrete: c_int = 0x10D;
// Reserved 0x10E - 0x10F
pub const kHIDUsage_BCS_AztecCode: c_int = 0x110;
pub const kHIDUsage_BCS_BC412: c_int = 0x111;
pub const kHIDUsage_BCS_ChannelCode: c_int = 0x112;
pub const kHIDUsage_BCS_Code16: c_int = 0x113;
pub const kHIDUsage_BCS_Code32: c_int = 0x114;
pub const kHIDUsage_BCS_Code49: c_int = 0x115;
pub const kHIDUsage_BCS_CodeOne: c_int = 0x116;
pub const kHIDUsage_BCS_Colorcode: c_int = 0x117;
pub const kHIDUsage_BCS_DataMatrix: c_int = 0x118;
pub const kHIDUsage_BCS_MaxiCode: c_int = 0x119;
pub const kHIDUsage_BCS_MicroPDF: c_int = 0x11A;
pub const kHIDUsage_BCS_PDF_417: c_int = 0x11B;
pub const kHIDUsage_BCS_PosiCode: c_int = 0x11C;
pub const kHIDUsage_BCS_QRCode: c_int = 0x11D;
pub const kHIDUsage_BCS_SuperCode: c_int = 0x11E;
pub const kHIDUsage_BCS_UltraCode: c_int = 0x11F;
pub const kHIDUsage_BCS_USB_5_SlugCode: c_int = 0x120;
pub const kHIDUsage_BCS_VeriCode: c_int = 0x121;
// Reserved 0x122 - 0xFFFF

// Weighing Devices Page (0x8D)
pub const kHIDUsage_WD_Undefined: c_int = 0x00;
pub const kHIDUsage_WD_WeighingDevice: c_int = 0x01;
// Reserved 0x02 - 0x1F
pub const kHIDUsage_WD_ScaleScaleDevice: c_int = 0x20;
pub const kHIDUsage_WD_ScaleScaleClassIMetricCL: c_int = 0x21;
pub const kHIDUsage_WD_ScaleScaleClassIMetric: c_int = 0x22;
pub const kHIDUsage_WD_ScaleScaleClassIIMetric: c_int = 0x23;
pub const kHIDUsage_WD_ScaleScaleClassIIIMetric: c_int = 0x24;
pub const kHIDUsage_WD_ScaleScaleClassIIILMetric: c_int = 0x25;
pub const kHIDUsage_WD_ScaleScaleClassIVMetric: c_int = 0x26;
pub const kHIDUsage_WD_ScaleScaleClassIIIEnglish: c_int = 0x27;
pub const kHIDUsage_WD_ScaleScaleClassIIILEnglish: c_int = 0x28;
pub const kHIDUsage_WD_ScaleScaleClassIVEnglish: c_int = 0x29;
pub const kHIDUsage_WD_ScaleScaleClassGeneric: c_int = 0x2A;
// Reserved 0x2B - 0x2F
pub const kHIDUsage_WD_ScaleAtrributeReport: c_int = 0x30;
pub const kHIDUsage_WD_ScaleControlReport: c_int = 0x31;
pub const kHIDUsage_WD_ScaleDataReport: c_int = 0x32;
pub const kHIDUsage_WD_ScaleStatusReport: c_int = 0x33;
pub const kHIDUsage_WD_ScaleWeightLimitReport: c_int = 0x34;
pub const kHIDUsage_WD_ScaleStatisticsReport: c_int = 0x35;
// Reserved 0x36 - 0x3F
pub const kHIDUsage_WD_DataWeight: c_int = 0x40;
pub const kHIDUsage_WD_DataScaling: c_int = 0x41;
// Reserved 0x42 - 0x4F
pub const kHIDUsage_WD_WeightUnit: c_int = 0x50;
pub const kHIDUsage_WD_WeightUnitMilligram: c_int = 0x51;
pub const kHIDUsage_WD_WeightUnitGram: c_int = 0x52;
pub const kHIDUsage_WD_WeightUnitKilogram: c_int = 0x53;
pub const kHIDUsage_WD_WeightUnitCarats: c_int = 0x54;
pub const kHIDUsage_WD_WeightUnitTaels: c_int = 0x55;
pub const kHIDUsage_WD_WeightUnitGrains: c_int = 0x56;
pub const kHIDUsage_WD_WeightUnitPennyweights: c_int = 0x57;
pub const kHIDUsage_WD_WeightUnitMetricTon: c_int = 0x58;
pub const kHIDUsage_WD_WeightUnitAvoirTon: c_int = 0x59;
pub const kHIDUsage_WD_WeightUnitTroyOunce: c_int = 0x5A;
pub const kHIDUsage_WD_WeightUnitOunce: c_int = 0x5B;
pub const kHIDUsage_WD_WeightUnitPound: c_int = 0x5C;
// Reserved 0x5D - 0x5F
pub const kHIDUsage_WD_CalibrationCount: c_int = 0x60;
pub const kHIDUsage_WD_RezeroCount: c_int = 0x61;
// Reserved 0x62 - 0x6F
pub const kHIDUsage_WD_ScaleStatus: c_int = 0x70;
pub const kHIDUsage_WD_ScaleStatusFault: c_int = 0x71;
pub const kHIDUsage_WD_ScaleStatusStableAtZero: c_int = 0x72;
pub const kHIDUsage_WD_ScaleStatusInMotion: c_int = 0x73;
pub const kHIDUsage_WD_ScaleStatusWeightStable: c_int = 0x74;
pub const kHIDUsage_WD_ScaleStatusUnderZero: c_int = 0x75;
pub const kHIDUsage_WD_ScaleStatusOverWeightLimit: c_int = 0x76;
pub const kHIDUsage_WD_ScaleStatusRequiresCalibration: c_int = 0x77;
pub const kHIDUsage_WD_ScaleStatusRequiresRezeroing: c_int = 0x78;
// Reserved 0x79 - 0x7F
pub const kHIDUsage_WD_ZeroScale: c_int = 0x80;
pub const kHIDUsage_WD_EnforcedZeroReturn: c_int = 0x81;
// Reserved 0x82 - 0xFFFF

// Magnetic Stripe Reader Page (0x8E)
pub const kHIDUsage_MSR_Undefined: c_int = 0x00;
pub const kHIDUsage_MSR_DeviceReadOnly: c_int = 0x01;
// Reserved 0x02 - 0x10
pub const kHIDUsage_MSR_Track1Length: c_int = 0x11;
pub const kHIDUsage_MSR_Track2Length: c_int = 0x12;
pub const kHIDUsage_MSR_Track3Length: c_int = 0x13;
pub const kHIDUsage_MSR_TrackJISLength: c_int = 0x14;
// Reserved 0x15 - 0x1F
pub const kHIDUsage_MSR_TrackData: c_int = 0x20;
pub const kHIDUsage_MSR_Track1Data: c_int = 0x21;
pub const kHIDUsage_MSR_Track2Data: c_int = 0x22;
pub const kHIDUsage_MSR_Track3Data: c_int = 0x23;
pub const kHIDUsage_MSR_TrackJISData: c_int = 0x24;
// Reserved 0x25 - 0xFFFF
