use bevy_ecs::prelude::*;
use crate::core::renderer::*;
use crate::components::entity::{ WalkerData, WalkerState };
use crate::components::*;
use crate::resources::asset_manager::*;
use crate::components::entity::*;
use super::player_animation_init;

#[derive(Bundle)]
struct PlayerBundle {
    sprite: Sprite,
    trans: Transform,
    vel: Velocity,
    area: Area,
    obb: OBB,
    player_d: PlayerData,
    tag: PlayerTag,
    input: PlayerInput,
    walker_d: WalkerData,
    anim_player: AnimationPlayer,
    grav_affected: GravityAffected,
    combat: Combat,
    e_over_obbs: EntityOverlappingOBBs,
    target_e_tags: TargetEntityTags,
    cell_pos: CellPos,
    e_tag_container: EntityTagContainer,
    health: Health,
    knock: KnockbackTrigger
}

pub fn player_spawn(world: &mut World, renderer: &mut Renderer) -> Entity {
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::Player);
    sprite.set_sprite_sheet(6, 6);

    let mut area = Area::new(
        10.0, -1000.0, 10.0, 20.0
    );
    area.offset = Vector2::new(0.0, 6.0);

    let player_e = world.spawn(PlayerBundle {
         sprite,
         trans: Transform::new(10.0, -1000.0),
         vel: Velocity::new(0.0, 0.0),
         area,
         obb: OBB::new(10.0, 20.0, Vector2::new(10.0, -1000.0), false),
         player_d: PlayerData::default(),
         tag: PlayerTag {},
         input: PlayerInput::default(),
         walker_d: WalkerData {
             run_speed: 120.0,
             accel: 30.0,
             jump_force: 250.0,
             grounded: false,
             state: WalkerState::default()
         },
         anim_player: AnimationPlayer::new(WalkerAnim::COUNT),
         grav_affected: GravityAffected(true),
         combat: Combat::new(2.0, 0.1),
         e_over_obbs: EntityOverlappingOBBs(Vec::new()),
         target_e_tags: TargetEntityTags(vec![EntityTag::EnemyWeapon]),
         cell_pos: CellPos(Vec::new()),
         e_tag_container: EntityTagContainer(EntityTag::Player),
         health: Health::new(100),
         knock: KnockbackTrigger::default(),
    }).id();

    let mut player_ref_mut = world.entity_mut(player_e);
    let mut anim_player = player_ref_mut.get_mut::<AnimationPlayer>().unwrap();
    player_animation_init(&mut anim_player, player_e);

    player_e
}
