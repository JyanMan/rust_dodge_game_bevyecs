use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use crate::sys;
use crate::plugins::sdl_plugin::Render;
use crate::resources::*;
use crate::components::*;

pub struct ProcAnimPlugin;

impl Plugin for ProcAnimPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<ProcAnim>();
        // app.add_systems(Render, (
        //     sys::render::proc_anim_edges.after(sys::render::sprites_draw),
        //     // sys::render::proc_anim_edges.after(sys::render::sprites_draw),
        // ));
        app.add_systems(FixedUpdate, (
            sys::physics::dist_constraints,
        ));
    }
}
