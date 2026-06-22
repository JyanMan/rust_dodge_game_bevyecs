use bevy_ecs::prelude::*;
use bevy_app::prelude::*;

use crate::sys;
use crate::config::RENDER_DISTANCE;
use crate::components::Vector2;
use crate::resources::EntityQuadMap;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            EntityQuadMap::new(
                Vector2::new(0.0, 0.0), RENDER_DISTANCE
            )
        );
        app.add_systems(Startup, (
            sys::world::chunks::init,
        ));

        app.add_systems(FixedPostUpdate, (
            // sys::entity::hit_reaction::update,
            // sys::entity::status::inflictor::<DamageOverTime>,

            sys::world::chunks::generate,
            sys::world::entity_quad::generate,
            sys::world::camera::update,
        ));

        app.add_systems(Update, (
            sys::world::damage_counter::update,
            // TODO
            // sys::world::damage_counter::despawn_update,
        ));

        // app.add_systems(Render, (
        //     sys::world::chunks::draw.before(sys::render::sprites_draw),
        // ));
    }
}
