use super::{Go, Machine};

/// A single gcode line
pub enum Instruction {
    /// An Gxxx instruction, translated into Go
    Go(Go),

    /// An Mxxx instruction, translated as Machine
    Machine(Machine),
}
