/// Describes a movement struct. All fields are optional, but usually at least 1 field is required.
#[derive(Default)]
pub struct Movement {
    /// The position to move to/by on the X axis
    pub x: Option<f32>,

    /// The position to move to/by on the Y axis
    pub y: Option<f32>,

    /// The position to move to/by on the Z axis
    pub z: Option<f32>,

    /// The amount to extrude between the starting point and ending point
    pub e: Option<f32>,

    /// The feedrate per minute of the move between the starting point and ending point (if supplied)
    pub f: Option<f32>,

    /// Flag to check if an endstop was hit. Default is S0.
    pub s: u8,
}
