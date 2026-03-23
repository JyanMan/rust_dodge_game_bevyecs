use bevy_ecs::prelude::*;
use crate::components::StateMachine;
use crate::resources::DeltaTime;
use crate::components::states::*;

pub fn update(
    mut movement_q: Query<&mut StateMachine<MovementState>>,
    mut combat_q: Query<&mut StateMachine<CombatState>>,
    delta_time: Res<DeltaTime>
) {
    for mut state_m in &mut movement_q {
        state_m.update_next_state_timer(delta_time.0);
    }
    for mut state_m in &mut combat_q {
        state_m.update_next_state_timer(delta_time.0);
    }
}
