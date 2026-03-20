use bevy_ecs::prelude::*;
use crate::components::StateMachine;
use crate::resources::DeltaTime;

pub fn update(
    mut query: Query<&mut StateMachine>,
    delta_time: Res<DeltaTime>
) {
    for mut state_m in &mut query {
        state_m.update_next_state_timer(delta_time.0);
    }
}
