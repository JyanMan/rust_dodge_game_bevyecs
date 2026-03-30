use bevy_ecs::prelude::*;
use crate::components::*;
use crate::components::states::*;
use crate::resources::DeltaTime;

pub fn timer( mut query: Query<&mut StateMachine<CombatState>>, delta_time: Res<DeltaTime> ) {
    for mut combat_state in &mut query {
        // if let CombatState::Knocked { mut timer } = combat_state.curr_state() {
        //     if timer.tick(delta_time.0).just_finished()  {
        //         combat_state.set_state(CombatState::KnockEnd);
        //     }
        //     combat_state.set_state(CombatState::Knocked{timer});
        // }
    }
}
