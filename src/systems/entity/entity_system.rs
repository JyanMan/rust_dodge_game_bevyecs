use bevy_ecs::prelude::*;
use crate::components::*;


pub fn entity_knocked_reaction(
    mut query: Query<(&KnockbackTrigger, &mut Combat)>,
) {
    for (knock, mut combat) in &mut query {
        if knock.knocked {
            combat.stun()
        }
    }
}
