use crate::ecs::ecs::*;
use super::*;

pub fn register_all_components(ecs: &mut ECS) {
    register_entity_components(ecs);
    register_weapon_components(ecs);

    ecs.register_component::<Area>();
    ecs.register_component::<Animation>();
    ecs.register_component::<AnimationPlayer>();
    ecs.register_component::<Combat>();
    ecs.register_component::<Owner>();
    ecs.register_component::<Position>();
    ecs.register_component::<RigidBody>();
    ecs.register_component::<Sprite>();
    ecs.register_component::<Velocity>();
    ecs.register_component::<WalkerData>();
}
