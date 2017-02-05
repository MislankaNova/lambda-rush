// Commands that can be given to lambda

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Left, Front, Right
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Thrust {
    Accelerate, Hold, Slow
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Command {
    pub direction : Direction,
    pub thrust    : Thrust,
    pub z         : bool,
    pub x         : bool,
    pub shift     : bool
}
