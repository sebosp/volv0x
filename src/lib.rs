#![no_std]
use core::fmt::Write;
use heapless::String;
use smart_leds::colors::*;

pub mod organelle;
pub use organelle::*;

/// The version of the Volvox protocol
pub const VOLVOX_VERSION: u32 = 0x00;

/// The ID is composed of:
/// SEB: 3 letters.
/// PRJCT: 5 letters project name
/// VV: 2 letters for Volvox Message Version
/// MAC-ADDRESS: 6 bytes of the MAC address of the ESP32
/// (SEB-PATCH-VV.MAC-ADDRESS) MAC address of the ESP32 and the last 4 bytes of the ESP32 chip ID
pub const VOLVOX_ID_LEN: usize = 10;

/// Volvox firmware ID
pub struct VolvoxId {
    /// Always "SEB"
    pub prelude: [u8; 3],
    /// The project name
    pub project: [u8; 5],
    /// The Volvox version
    pub version: [u8; 2],
}

impl VolvoxId {
    /// Creates a new VolvoxId
    pub fn new(project: [u8; 5], version: [u8; 2]) -> Self {
        Self {
            prelude: *b"SEB",
            project,
            version,
        }
    }

    /// Converts the VolvoxId to a string
    pub fn to_string(&self) -> String<VOLVOX_ID_LEN> {
        let mut volvox_id: String<VOLVOX_ID_LEN> = String::new();
        write!(
            &mut volvox_id,
            "{}{}{}",
            core::str::from_utf8(&self.prelude).unwrap(),
            core::str::from_utf8(&self.project).unwrap(),
            core::str::from_utf8(&self.version).unwrap(),
        )
        .expect("Failed to convert VolvoxId to string");
        volvox_id
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VolvoxMsg {
    /// A unit is registering itself to peers.
    Register {
        /// The ID of the unit
        id: [u8; VOLVOX_ID_LEN],
        /// The capabilities of the unit
        capabilities: u32,
    },
    /// A request to get
    Request([u8; VOLVOX_ID_LEN], u64),
    /// A unit is unregistering itself to peers.
    Response([u8; VOLVOX_ID_LEN], u64),
}
