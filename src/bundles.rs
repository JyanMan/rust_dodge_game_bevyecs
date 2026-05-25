use bevy_ecs::prelude::*;
use crate::components::*;
use crate::components::states::*;

#[derive(Bundle)]
pub struct HitboxBundle {
    pub obb: OBB,
    pub e_over_obbs: EntityOverlappingOBBs,
    pub target_e_tags: TargetEntityTags,
    pub cell_pos: CellPos,
    // pub e_tag_container: EntityTagContainer,
}

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub trans: Transform,
    pub vel: Velocity,
    pub area: Area,
    pub grav_affected: GravityAffected,
}

#[derive(Bundle)]
pub struct WeaponBundle {
    pub tag: WeaponTag,
    pub trans: Transform,
    pub hitbox: HitboxBundle,
    pub fns: WeaponFns,
    pub state: StateMachine<WeaponState>,
    pub data: WeaponConfig,
}
