use bevy_ecs::prelude::*;
use bevy_app::prelude::*;

use crate::components::*;
use crate::resources::TagRegistry;

use crate::sys;

pub struct TagsRegistry;

impl Plugin for TagsRegistry {
    fn build(&self, app: &mut App) {
        app.init_resource::<TagRegistry>();
        app.add_systems(PostUpdate, (
            sys::entity::tag::handle::<PlayerTag>,
            sys::entity::tag::handle::<ZombieTag>,
            sys::entity::tag::handle::<AllyWeaponTag>,
            sys::entity::tag::handle::<AllyTag>,
            sys::entity::tag::handle::<EnemyWeaponTag>,
            sys::entity::tag::handle::<EnemyTag>,
        ));
    }
}

