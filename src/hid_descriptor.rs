//! HID Report Descriptor Analysis and Raw Bytes
//!
//! This module provides detailed analysis of the HID report descriptor
//! used by the button box, including the raw byte representation.

/// Raw HID report descriptor bytes for a 2-button gamepad
///
/// This is the compiled form of our HID descriptor that gets sent to the host.
/// Understanding this helps with debugging and compatibility issues.
pub const HID_REPORT_DESCRIPTOR: &[u8] = &[
    // Usage Page (Generic Desktop)
    0x05, 0x01, // Usage (Gamepad)
    0x09, 0x05, // Collection (Application)
    0xA1, 0x01, // Usage (Pointer)
    0x09, 0x01, // Collection (Physical)
    0xA1, 0x00, // Usage Page (Button)
    0x05, 0x09, // Usage Minimum (Button 1)
    0x19, 0x01, // Usage Maximum (Button 2)
    0x29, 0x02, // Logical Minimum (0)
    0x15, 0x00, // Logical Maximum (1)
    0x25, 0x01, // Report Count (2)
    0x95, 0x02, // Report Size (1 bit)
    0x75, 0x01, // Input (Data, Variable, Absolute)
    0x81, 0x02, // Report Count (6) - Padding bits
    0x95, 0x06, // Report Size (1 bit)
    0x75, 0x01, // Input (Constant, Variable, Absolute)
    0x81, 0x01, // End Collection (Physical)
    0xC0, // End Collection (Application)
    0xC0,
];

/// Structure representing a single HID report from the button box
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, packed)]
pub struct ButtonBoxHidReport {
    /// Button states packed into a single byte
    /// - Bit 0: Button 1 (1 = pressed, 0 = released)
    /// - Bit 1: Button 2 (1 = pressed, 0 = released)
    /// - Bits 2-7: Padding (always 0)
    pub buttons: u8,
}

impl ButtonBoxHidReport {
    /// Create a new report with no buttons pressed
    pub const fn new() -> Self {
        Self { buttons: 0 }
    }

    /// Set the state of button 1
    pub fn set_button1(&mut self, pressed: bool) {
        if pressed {
            self.buttons |= 0x01;
        } else {
            self.buttons &= !0x01;
        }
    }

    /// Set the state of button 2
    pub fn set_button2(&mut self, pressed: bool) {
        if pressed {
            self.buttons |= 0x02;
        } else {
            self.buttons &= !0x02;
        }
    }

    /// Get the state of button 1
    pub fn button1_pressed(&self) -> bool {
        (self.buttons & 0x01) != 0
    }

    /// Get the state of button 2
    pub fn button2_pressed(&self) -> bool {
        (self.buttons & 0x02) != 0
    }

    /// Convert to byte array for transmission
    pub fn as_bytes(&self) -> [u8; 1] {
        [self.buttons]
    }

    /// Create from raw bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= 1 {
            Some(Self {
                buttons: bytes[0] & 0x03, // Mask to only use first 2 bits
            })
        } else {
            None
        }
    }
}

impl Default for ButtonBoxHidReport {
    fn default() -> Self {
        Self::new()
    }
}

/// HID Descriptor field descriptions for documentation purposes
pub mod descriptor_fields {
    //! Human-readable descriptions of HID descriptor fields

    /// HID descriptor field explanations
    pub const FIELD_DESCRIPTIONS: &[(&str, &str)] = &[
        ("0x05, 0x01", "Usage Page (Generic Desktop)"),
        ("0x09, 0x05", "Usage (Gamepad)"),
        ("0xA1, 0x01", "Collection (Application)"),
        ("0x09, 0x01", "Usage (Pointer)"),
        ("0xA1, 0x00", "Collection (Physical)"),
        ("0x05, 0x09", "Usage Page (Button)"),
        ("0x19, 0x01", "Usage Minimum (Button 1)"),
        ("0x29, 0x02", "Usage Maximum (Button 2)"),
        ("0x15, 0x00", "Logical Minimum (0)"),
        ("0x25, 0x01", "Logical Maximum (1)"),
        ("0x95, 0x02", "Report Count (2 buttons)"),
        ("0x75, 0x01", "Report Size (1 bit each)"),
        ("0x81, 0x02", "Input (Data, Variable, Absolute)"),
        ("0x95, 0x06", "Report Count (6 padding bits)"),
        ("0x75, 0x01", "Report Size (1 bit each)"),
        ("0x81, 0x01", "Input (Constant, Variable, Absolute)"),
        ("0xC0", "End Collection (Physical)"),
        ("0xC0", "End Collection (Application)"),
    ];

    /// Report structure explanation
    pub const REPORT_STRUCTURE: &str = r#"
Report Structure (1 byte total):
================================
Byte 0:
  Bit 0: Button 1 state (1 = pressed, 0 = released)
  Bit 1: Button 2 state (1 = pressed, 0 = released)
  Bits 2-7: Padding (constant 0)

Examples:
- No buttons pressed: 0x00
- Button 1 pressed: 0x01
- Button 2 pressed: 0x02
- Both buttons pressed: 0x03
"#;

    /// USB device information
    pub const USB_DEVICE_INFO: &str = r#"
USB Device Information:
======================
Vendor ID (VID): 0x16C0 (Van Ooijen Technische Informatica)
Product ID (PID): 0x27DD (Generic HID device)
Manufacturer: "Button Box Co"
Product: "2-Button Box"
Serial Number: "001"
Device Class: Interface-specific (0x00)
"#;
}

/// Constants for button bit positions
pub mod button_bits {
    pub const BUTTON1_BIT: u8 = 0x01;
    pub const BUTTON2_BIT: u8 = 0x02;
    pub const BUTTON_MASK: u8 = 0x03;
    pub const PADDING_MASK: u8 = 0xFC;
}

/// Helper functions for button state manipulation
pub mod button_helpers {
    use super::button_bits::*;

    /// Extract button states from raw byte
    pub fn extract_buttons(raw_byte: u8) -> (bool, bool) {
        let button1 = (raw_byte & BUTTON1_BIT) != 0;
        let button2 = (raw_byte & BUTTON2_BIT) != 0;
        (button1, button2)
    }

    /// Create button byte from individual button states
    pub fn create_button_byte(button1: bool, button2: bool) -> u8 {
        let mut byte = 0u8;
        if button1 {
            byte |= BUTTON1_BIT;
        }
        if button2 {
            byte |= BUTTON2_BIT;
        }
        byte
    }

    /// Validate that a byte only has valid button bits set
    pub fn is_valid_button_byte(byte: u8) -> bool {
        (byte & PADDING_MASK) == 0
    }
}
