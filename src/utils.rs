//! Utility functions for the project
//!

use core::fmt::Write;
use heapless::String;

/// Converts a MAC address to a string
pub fn mac_address_to_string(mac_address: [u8; 6]) -> String<18> {
    let mut mac_str: String<18> = String::new();
    write!(
        &mut mac_str,
        "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
        mac_address[0],
        mac_address[1],
        mac_address[2],
        mac_address[3],
        mac_address[4],
        mac_address[5],
    )
    .expect("Failed to transform mac address to string");
    mac_str
}
