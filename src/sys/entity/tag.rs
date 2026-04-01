use bevy_ecs::prelude::*;
use crate::components::*;
use crate::resources::*;

pub fn handle<Tag: Component>(
    query: Query<Entity, Added<Tag>>,
    mut removed: RemovedComponents<Tag>, 
    mut tag_reg: ResMut<TagRegistry>,
) {
    for e in &query {
        tag_reg.entity_insert::<Tag>(e);
    }
    removed.read().for_each(|e| {
        tag_reg.entity_remove::<Tag>(e);
    });
}
