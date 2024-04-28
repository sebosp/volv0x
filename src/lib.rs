#![no_std]

pub mod id;
pub mod inventory;
pub mod message;
pub mod organelle;
pub mod utils;

/// The version of the Volvox firmware, used for message compatibility
pub const VOLVOX_VERSION: u32 = 0x00;
