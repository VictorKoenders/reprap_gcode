use super::{ArcMovement, Movement, Result};
use parser::Parser;

pub enum Go {
    /// G0: Rapid linear Move
    RapidLinearMove(Movement),

    /// G1: linear Move
    LinearMove(Movement),

    /// G2: Clockwise Arc
    ClockwiseArc(ArcMovement),

    /// G3: Counter clockwise Arc
    CounterClockwiseArc(ArcMovement),
}

impl Go {
    pub fn parse(input: &mut Parser) -> Result<Go> {}
}
