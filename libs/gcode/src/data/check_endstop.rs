/// Flag to check if an endstop was hit. Default is S0.
pub enum CheckEndstop {
    /// ignore end stops
    Ignore = 0,

    /// check for end stops
    Check = 1,

    /// In RepRapFirmware, using the S1 or S2 parameter on a delta printer causes the XYZ parameters to refer to the individual tower motor positions instead of the head position, and to enable endstop detection as well if the parameter is S1
    CheckIndividualMotorPosition = 2,
}

impl Default for CheckEndstop {
    fn default() -> Self {
        CheckEndstop::Ignore
    }
}
