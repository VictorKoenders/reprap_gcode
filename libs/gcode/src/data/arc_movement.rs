/// Describes an arc movement struct. All fields are optional, but usually at least 1 field is required.
#[derive(Default)]
pub struct ArcMovement {
    /// The position to move to/by on the X axis
    pub x: Option<f32>,

    /// The position to move to/by on the Y axis
    pub y: Option<f32>,

    /// The point in X space from the current X position to maintain a constant distance from
    pub i: Option<f32>,

    /// The point in Y space from the current Y position to maintain a constant distance from
    pub j: Option<f32>,

    /// The amount to extrude between the starting point and ending point
    pub e: Option<f32>,

    /// The feedrate per minute of the move between the starting point and ending point (if supplied)
    pub f: Option<f32>,
}
