//! Time Of Flight sensing organelle

/// The sensed range of the sensor, some of the values are only available for the VL53L1X, which
/// senses max 4 meters. The sensor VL53L0X senses max 2 meters.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SensedRange {
    /// An object is Extremely near to the sensor
    /// Less than 200 mm
    ExtremelyNear,
    /// An object is Very near to the sensor
    /// Less than 600 mm
    VeryNear,
    /// An object is near from the sensor
    /// Less than 1000 mm
    Near,
    /// An object is near from the sensor
    /// Less than 2000 mm
    Far,
    /// An object is very far from the sensor
    /// Less than 3000 mm. Only available for the VL53L1X
    VeryFar,
    /// An object is very far from the sensor
    /// Less than 3800 mm. Only available for the VL53L1X
    ExtremelyFar,
    /// Nothing sensed.
    None,
    /// Sensor is off, not connected or I2C driver error
    Off,
}

impl SensedRange {
    /// Converts a distance in mm to a `SensedRange`
    pub fn from_distance(distance: u16) -> Self {
        match distance {
            0..=200 => SensedRange::ExtremelyNear,
            201..=600 => SensedRange::VeryNear,
            601..=1000 => SensedRange::Near,
            1001..=2000 => SensedRange::Far,
            2001..=3000 => SensedRange::VeryFar,
            3001..=3800 => SensedRange::ExtremelyFar,
            1001..=8190 => SensedRange::None,
            _ => SensedRange::Off,
        }
    }

    /// Returns the next time in ms to display the sensed range
    pub fn next_blink_loop(&self) -> u8 {
        match self {
            SensedRange::ExtremelyNear => 2,
            SensedRange::VeryNear => 3,
            SensedRange::Near => 4,
            SensedRange::Far => 5,
            SensedRange::VeryFar => 6,
            SensedRange::ExtremelyFar => 7,
            SensedRange::None => 8,
            SensedRange::Off => 32,
        }
    }

    /* An example implementation that transforms the SensedRange into a color
     * use smart_leds::colors::*;
     * pub fn into_color(&self) -> [smart_leds::RGB<u8>; 1] {
            match self {
                SensedRange::ExtremelyNear => [DARK_RED],
                SensedRange::VeryNear => [ORANGE_RED],
                SensedRange::Near => [DARK_ORCHID],
                SensedRange::Far => [GOLD],
                SensedRange::VeryFar => [GREEN],
                SensedRange::ExtremelyFar => [LIGHT_GREEN],
                SensedRange::None => [BLACK],
                SensedRange::Off => [BLUE],
            }
    } */
}
