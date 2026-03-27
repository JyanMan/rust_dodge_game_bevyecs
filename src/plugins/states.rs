use bevy_ecs::prelude::*;
use bevy_app::prelude::*;

use crate::sys;

pub struct States;

impl Plugin for States {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            sys::state_machine::update,
        ));
        app.add_systems(PostUpdate, (
            sys::entity::zombie::state_handler,
            sys::entity::player::state_handler,
            sys::entity::player::stat_update,
            sys::weapon::anim_state_update,
            sys::anim::walker::anim_state_handler,
        ));
    }
}

