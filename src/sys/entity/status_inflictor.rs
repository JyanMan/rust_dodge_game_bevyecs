use bevy_ecs::prelude::*;
use crate::components::*;

pub fn update<S: Component + Clone>(
    mut query: Query<(&StatusInflictor<S>, &mut EntityOverlappingOBBs)>,
    mut commands: Commands
) {
    for (inflictor, mut e_over_obbs) in &mut query {
        for (e, _) in e_over_obbs.0.iter() {
            // println!("working overlap check");
            // inflictor.inflict(*e, &mut commands);
        }
    }
}
