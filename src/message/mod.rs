//! The Volvox messaging system.

use crate::id::VOLVOX_ID_LEN;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VolvoxMsg {
    /// A unit is registering itself to peers.
    Register {
        /// The ID of the unit
        id: [u8; VOLVOX_ID_LEN],
        /// The volvox firmware version
        version: u32,
        /// The capabilities of the unit
        capabilities: u32,
    },
    /// A request to generate a reaction to an event
    Request {
        /// The source of the request
        src: [u8; VOLVOX_ID_LEN],
        /// The destination of the request
        dst: [u8; VOLVOX_ID_LEN],
        /// The request ID
        id: u64,
        /// The request data
        data: RequestData,
    },
    /// A unit is unregistering itself to peers.
    Response([u8; VOLVOX_ID_LEN], u64),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RequestData {
    /// A request to get the distance from a ToF sensor
    Distance,
    /// A request to blink an LED
    Blink,
    /// A request to vibrate
    Vibrate,
    /// A request to activate the buzzer
    Buzzer,
    /// A request to get the light level
    Light,
    /// A request to get the button state
    Button,
    /// A request to get the accelerometer data
    Accel,
    /// A request to get the gyroscope data
    Gyro,
    /// A request to get the microphone data
    Microphone,
    /// A request to get the infra red transmitter data
    InfraredTx,
    /// A request to get the infra red receiver data
    InfraredRx,
    /// A request to get the display data
    Display,
    /// A request to get the battery data
    Battery,
    /// A request to get the gas sensor data
    Gas,
    /// A request to get the temperature sensor data
    Temperature,
}
