use crate::ecs::ecs::*;
use super::*;

pub fn register_entity_components(ecs: &mut ECS) {
    //PLAYER
    ecs.register_component::<PlayerData>();
    //ZOMBIE
    ecs.register_component::<ZombieTag>();
}
