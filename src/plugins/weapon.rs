use bevy_ecs::prelude::*;
use bevy_app::prelude::*;

use crate::sys;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (
            sys::weapon::attack_timer_and_signal_update,
        ));
        app.add_systems(Update, (
            sys::weapon::lost_owner,
            sys::weapon::newly_owned,
        ));
        app.add_systems(PostUpdate, (
            sys::weapon::anim_state_update,
        ));
    }
}
            
