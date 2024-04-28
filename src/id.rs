//! The ID is composed of:
//! SEB: 3 bytes
//! PRJCT: 5 bytes project name
//! VV: 2 bytes for Volvox firmware Version
//! MAC-ADDRESS: 6 bytes of the MAC address of the ESP32
//! The alias of the unit: 16 bytes, i.e. LEFT_PINKY_TOF__
//! (SEBPATCH00ffeeddccbbaaLEFT_PINKY_TOF__)

// TODO: We use mostly alphanum and could use only lowercase to reduce the size of the ID
use crate::utils::mac_address_to_string;
use core::fmt::Write;
use heapless::String;

pub const VOLVOX_ID_LEN: usize = 32;
/// The friendly print has dash separators and is easier to read so needs a few more bytes
pub const VOLVOX_ID_FRIENDLY_PRINT_LEN: usize = 34;

/// Volvox firmware ID
pub struct VolvoxId {
    /// Always "SEB"
    pub prelude: [u8; 3],
    /// The project name
    pub project: [u8; 5],
    /// The Volvox version
    pub version: [u8; 2],
    /// The MAC address of the ESP32
    pub mac_address: [u8; 6],
    /// The alias of the unit
    pub alias: String<16>,
}

impl VolvoxId {
    /// Creates a new VolvoxId
    pub fn new(project: [u8; 5], version: [u8; 2], mac_address: [u8; 6]) -> Self {
        Self {
            prelude: *b"SEB",
            project,
            version,
            mac_address,
            alias: String::new(),
        }
    }

    /// Sets the alias of the unit
    pub fn set_alias(&mut self, alias: &str) {
        self.alias.clear();
        write!(&mut self.alias, "{}", alias).expect("Failed to set alias");
    }

    /// Converts the VolvoxId to a string
    pub fn to_string(&self) -> String<VOLVOX_ID_FRIENDLY_PRINT_LEN> {
        let mut volvox_id: String<VOLVOX_ID_FRIENDLY_PRINT_LEN> = String::new();
        write!(
            &mut volvox_id,
            "{}{}{}.{}.{}",
            core::str::from_utf8(&self.prelude).unwrap(),
            core::str::from_utf8(&self.project).unwrap(),
            core::str::from_utf8(&self.version).unwrap(),
            mac_address_to_string(self.mac_address),
            self.alias
        )
        .expect("Failed to convert VolvoxId to string");
        volvox_id
    }
}
