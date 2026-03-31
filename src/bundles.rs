use bevy_ecs::prelude::*;
use crate::components::*;

#[derive(Bundle)]
pub struct HitboxBundle {
    pub obb: OBB,
    pub e_over_obbs: EntityOverlappingOBBs,
    pub target_e_tags: TargetEntityTags,
    pub cell_pos: CellPos,
    pub e_tag_container: EntityTagContainer,
}
