// A force that can act on an agent

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Force {
    Damage(i32)
}
