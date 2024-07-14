//! The inventory module contains the known units and their organelles.
//!

pub const C6_VIBRATOR_01: [u8; 6] = [0x54, 0x32, 0x04, 0x03, 0x3a, 0x64];

pub const S3_DUAL_HAND_VIBR_LEFT: [u8; 6] = [0xf4, 0x12, 0xfa, 0x8d, 0xb8, 0x5c];

pub const C6_VIBRATOR_01_ORGANELLES: [&str; 2] = ["Vibrator", "Battery"];

/*
Chip type:         esp32c6 (revision v0.0)
Crystal frequency: 40 MHz
Flash size:        4MB
Features:          WiFi 6, BT 5
MAC address:       54:32:04:03:3a:64
*/
