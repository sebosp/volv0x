//! The organelles provide different functionalities to the Volvox peers.
//! Some may provide light sensors, distance, sound, gas, etc.
//!

use bitflags::bitflags;

pub mod tof;
pub use tof::*;

// Organelles have different functions, be them:
// - Input:
//   - Button
//   - Distance
//   - Accelerometer
//   - Gyroscope
//   - Microphone
//   - Infra Red
//   - Gas
//   - Battery
//   - Temperature
//   - Humidity
//   - Pressure
//   - Light
// - Output
//   - LED
//   - Buzzer
//   - Display
//   - Sound
//   - Vibration

bitflags! {
    /// The Functions of the unit
    pub struct Functions: u32 {
        /// The unit has LED(s), could be NeoPixel or Simple LED
        const LED         = 0b0000000_0000000_0000000_0000001;
        /// The unit has a distance sensor, could be a ToF VL53L0X or ToF VL53L1X, etc.
        const DISTANCE    = 0b0000000_0000000_0000000_0000010;
        /// The unit has vibration device
        const VIBRATOR    = 0b0000000_0000000_0000000_0000100;
        /// The unit has a visible light sensor
        const LIGHT       = 0b0000000_0000000_0000000_0001000;
        /// The unit has a button that can be pressed to acknowledge a receipt required message
        const BUTTON      = 0b0000000_0000000_0000000_0010000;
        /// The unit has a buzzer
        const BUZZER      = 0b0000000_0000000_0000000_0100000;
        /// The unit has an accelerometer
        const ACCEL       = 0b0000000_0000000_0000000_1000000;
        /// The unit has a gyroscope
        const GYRO        = 0b0000000_0000000_0000001_0000000;
        /// The unit has a microphone
        const MICROPHONE  = 0b0000000_0000000_0000010_0000000;
        /// The unit has infra red transmitter
        const INFRARED_TX = 0b0000000_0000000_0000100_0000000;
        /// The unit has infra red receiver
        const INFRARED_RX = 0b0000000_0000000_0001000_0000000;
        /// The unit has a display
        const DISPLAY     = 0b0000000_0000000_0010000_0000000;
        /// The unit has battery
        const BATTERY     = 0b0000000_0000000_0100000_0000000;
        /// The unit has gas sensor, could be CO2, CO, etc.
        const GAS         = 0b0000000_0000000_1000000_0000000;
        /// The unit has temperature sensor
        const TEMPERATURE = 0b0000000_0000001_0000000_0000000;
        /// The unit has humidity sensor
        const HUMIDITY    = 0b0000000_0000010_0000000_0000000;
        /// The unit has pressure sensor
        const PRESSURE    = 0b0000000_0000100_0000000_0000000;
        /// The unit has sound sensor
        const SOUND       = 0b0000000_0001000_0000000_0000000;
    }
}
