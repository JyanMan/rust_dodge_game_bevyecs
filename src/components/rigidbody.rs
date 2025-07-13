use crate::components::Transform;
use crate::components::velocity::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct RigidBody {
    transform: Transform,
    vel: Velocity,
    grounded: bool
}
