use bevy_ecs::prelude::*;
use bevy_app::prelude::*;

use crate::sys;
use crate::components::DamageOverTime;
use crate::plugins::sdl_plugin::Input;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, 
            sys::entity::health::update,
        );
        app.add_systems(Update, (
            sys::entity::dying::update,
            sys::entity::dodge_stamina::update_sprites,
            sys::entity::health::player::health_bar_update,
        ));

        app.add_systems(FixedPostUpdate, (
            sys::entity::hit_reaction::update,
            sys::entity::status::inflictor::<DamageOverTime>,
        ));

        app.add_systems(Input, (
            sys::entity::player::input_update,
        ));
    }
}
