pub struct EntityState {
    grounded: bool,
    knocked: bool,
}

#[derive(Clone)]
pub struct WalkerData {
    run_speed: f32,
    accel: f32,
    jump_force: f32,
    pub grounded: bool,
}
