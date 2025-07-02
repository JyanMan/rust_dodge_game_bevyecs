use crate::components::position::*;
use crate::components::velocity::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct RigidBody {
    pos: Position,
    vel: Velocity,
    grounded: bool
}
