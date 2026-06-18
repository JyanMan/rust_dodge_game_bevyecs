use std::any::TypeId;
use bevy_ecs::prelude::*;
use crate::core::renderer::*;
use crate::components::entity::{ WalkerData, WalkerState };
use crate::components::*;
use crate::resources::asset_manager::*;
use crate::components::states::*;
use crate::components::entity::*;
use crate::bundles::*;

#[derive(Bundle)]
struct PlayerBundle {
    sprite: Sprite,
    physics: PhysicsBundle,
    player_d: PlayerData,
    dodge_stam: DodgeStamina,
    p_tag: PlayerTag,
    ally_tag: AllyTag,
    input: PlayerInput,
    walker_d: WalkerData,
    anim_player: AnimationPlayer,
    combat: Combat,
    hitbox: HitboxBundle,
    health: Health,
    movement_state: StateMachine<MovementState>,
    combat_state: StateMachine<CombatState>,
}


pub fn spawn(world: &mut World) -> Entity {
    let renderer = world.get_non_send_resource::<Renderer>().unwrap();
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::Player);
    sprite.set_sprite_sheet(6, 6);

    let mut area = Area::new(
        10.0, -1000.0, 10.0, 20.0
    );
    area.offset = Vector2::new(0.0, 6.0);

    let player_e = world.spawn(PlayerBundle {
         sprite,
         physics: PhysicsBundle {
             area,
             grav_affected: GravityAffected(true),
             trans: Transform::new(10.0, -1000.0),
             vel: Velocity::new(0.0, 0.0),
         },
         player_d: PlayerData::default(),
         dodge_stam: DodgeStamina::new(2),
         p_tag: PlayerTag {},
         ally_tag: AllyTag,
         input: PlayerInput::default(),
         walker_d: WalkerData {
             run_speed: 120.0,
             accel: 30.0,
             jump_force: 250.0,
             grounded: false,
             state: WalkerState::default()
         },
         anim_player: AnimationPlayer::new(WalkerAnim::COUNT),
         combat: Combat::new(2.0, 0.0),
         hitbox: HitboxBundle {
             obb: OBB::new(10.0, 20.0, Vector2::new(10.0, -1000.0), false)
                 .with_offset(Vector2::new(0.0, 3.0)),
             e_over_obbs: EntityOverlappingOBBs::default(),
             target_e_tags: TargetEntityTags(vec![TypeId::of::<EnemyWeaponTag>()]),
             cell_pos: CellPos(Vec::new()),
             // e_tag_container: EntityTagContainer(EntityTag::Player),
         },
         health: Health::new(100),
         // knock: KnockbackTrigger::default(),
         // state_machine: StateMachine::new(State::Idle),
         movement_state: super::states::movement_state(),
         combat_state: super::states::combat_state(),
         // walker_anim: super::states::walker_state(),
    }).id();

    spawn_cape(world, player_e);


    let mut player_ref_mut = world.entity_mut(player_e);
    let mut anim_player = player_ref_mut.get_mut::<AnimationPlayer>().unwrap();
    // player_animation_init(&mut anim_player, player_e);
    super::anim_init(&mut anim_player, player_e);

    player_e
}

fn spawn_cape(world: &mut World, player_e: Entity) {


    let e0 = world.spawn((
        AttachedTo(player_e),
        Anchor(player_e),
        Transform::zero(),
        LocalTransform::new(-2.0, 2.0),
    )).id();

    let e1_restrict = world.spawn((
        AttachedTo(player_e),
        Transform::zero(),
        LocalTransform::new(-7.5, 7.0)
    )).id();
    let e1 = world.spawn((
        Constraints(vec![
            DistanceConstraint {
                target: e0,
                distance: 3.5,
                stiffness: 80.0,
                target_offset: Vector2::new(0.0, -1.5)
            },
            DistanceConstraint {
                target: e1_restrict, 
                distance: 5.5,
                stiffness: 80.0,
                target_offset: Vector2::new(-2.0, 0.0)
            }
        ]),
        Transform::zero(),
        GravityAffected(true),
        Velocity::new(0.0, 0.0),
        InducedVelocity::default()
    )).id();

    let e2_restrict = world.spawn((
        AttachedTo(player_e),
        Transform::zero(),
        LocalTransform::new(-7.0, 10.0)
    )).id();
    let e2 = world.spawn((
        Constraints(vec![
            DistanceConstraint {
                target: e1,
                distance: 3.5,
                stiffness: 80.0,
                target_offset: Vector2::new(0.0, -1.5)
            },
            DistanceConstraint {
                target: e2_restrict,
                distance: 5.5,
                stiffness: 80.0,
                target_offset: Vector2::new(0.0, -1.5)
            },
        ]),
        Transform::new(5.0, 20.0),
        GravityAffected(true),
        Velocity::new(0.0, 0.0),
        InducedVelocity::default()
    )).id();

    let b0 = world.spawn((
        AttachedTo(player_e),
        Anchor(player_e),
        Transform::zero(),
        LocalTransform::new(2.0, 2.0),
    )).id();

    let b1_restrict = world.spawn((
        AttachedTo(player_e),
        Transform::zero(),
        LocalTransform::new(7.5, 7.0)
    )).id();
    let b1 = world.spawn((
        Constraints(vec![
            DistanceConstraint {
                target: b0,
                distance: 3.5,
                stiffness: 80.0,
                target_offset: Vector2::new(0.0, -1.5)
            },
            DistanceConstraint {
                target: b1_restrict,
                distance: 5.5,
                stiffness: 80.0,
                target_offset: Vector2::new(2.0, 0.0)
            }
        ]),
        Transform::zero(),
        GravityAffected(true),
        Velocity::new(0.0, 0.0),
        InducedVelocity::default()
    )).id();

    let b2_restrict = world.spawn((
        AttachedTo(player_e),
        Transform::zero(),
        LocalTransform::new(7.0, 10.0)
    )).id();
    let b2 = world.spawn((
        Constraints(vec![
            DistanceConstraint {
                target: b1,
                distance: 3.5,
                stiffness: 80.0,
                target_offset: Vector2::new(0.0, -1.5)
            },
            DistanceConstraint {
                target: b2_restrict,
                distance: 5.5,
                stiffness: 80.0,
                target_offset: Vector2::new(0.0, -1.5)
            },
        ]),
        Transform::new(5.0, 20.0),
        GravityAffected(true),
        Velocity::new(0.0, 0.0),
        InducedVelocity::default()
    )).id();

    let end_restrict = world.spawn((
        AttachedTo(player_e),
        Transform::zero(),
        LocalTransform::new(0.0, 10.0)
    )).id();
    let end_connector = world.spawn((
        Constraints(vec![
            DistanceConstraint {
                target: b2,
                distance: 2.0,
                stiffness: 80.0,
                target_offset: Vector2::new(2.0, -1.5)
            },
            DistanceConstraint {
                target: e2,
                distance: 2.0,
                stiffness: 80.0,
                target_offset: Vector2::new(-2.0, -1.5)
            },
            DistanceConstraint {
                target: end_restrict,
                distance: 5.0,
                stiffness: 80.0,
                target_offset: Vector2::new(0.0, 2.5)
            },
        ]),
        Transform::new(20.0, -5.0),
        GravityAffected(true),
        Velocity::new(0.0, 0.0),
        InducedVelocity::default()
    )).id();

    let anchor_connector = world.spawn((
        Constraints(vec![
            DistanceConstraint {
                target: b0,
                distance: 2.0,
                stiffness: 80.0,
                target_offset: Vector2::new(2.0, 0.0)
            },
            DistanceConstraint {
                target: e0,
                distance: 2.0,
                stiffness: 80.0,
                target_offset: Vector2::new(-2.0, 0.0)
            },
        ]),
        Transform::new(20.0, -5.0),
        GravityAffected(true),
        Velocity::new(0.0, 0.0),
        InducedVelocity::default()
    )).id();

    let polygon = world.spawn(
        PolygonId {
            list: vec![
                e0, e1, e2,
                end_connector,
                b2, b1, b0,
                anchor_connector,
            ]
        }
    ).id();
}
