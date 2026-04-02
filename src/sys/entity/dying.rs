use bevy_ecs::prelude::*;
use crate::components::Dying;

/// did not use Added<Dying> so that the despawn happens during a normal frame
/// otherwise the entity is despawned too early making the Dying component
/// to be read by other systems too late.
/// This crashes if a command is done on entity that is despawned
pub fn update(
    query: Query<(Entity, &Dying)>,
    mut commands: Commands
) {
    for (e, _) in query {
        commands.entity(e).despawn();
    }
}
