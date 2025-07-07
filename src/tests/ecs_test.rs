use crate::ecs::ecs::*;
use crate::components::position::*;
use crate::components::velocity::*;
use crate::components::sprite::*;
use crate::components::animation_player::*;
use crate::components::area::*;

#[test]
pub fn query_test() {

    let mut ecs = ECS::new();

    // Register all component types
    ecs.register_component::<Position>();
    ecs.register_component::<Velocity>();
    ecs.register_component::<Sprite>();
    ecs.register_component::<AnimationPlayer>();
    ecs.register_component::<Area>();

    // Spawn entity with Position and Velocity
    ecs.spawn::<(Position, Velocity)>((Position::zero(), Velocity::zero()));

    // Spawn entity with Position and AnimationPlayer
    ecs.spawn::<(Position, AnimationPlayer)>((Position::zero(), AnimationPlayer::new(3)));

    // Mutate Position.x of entities with Position + Sprite
    for (pos, _sprite) in ecs.query_comp::<(&mut Position, &Sprite)>() {
        pos.x = 20.0;
    }

    // Mutate Position.x of entities with Position + AnimationPlayer
    for (pos, _anim_p) in ecs.query_comp::<(&mut Position, &AnimationPlayer)>() {
        pos.x = 12314.0;
    }

    // mutate all position.y
    for pos in ecs.query_comp::<&mut Position>() {
        pos.y = 55.0;
    }

    // Assert mutation worked for AnimationPlayer group
    for (pos, _) in ecs.query_comp::<(&Position, &AnimationPlayer)>() {
        dbg!("pos: {}, {}", pos.x, pos.y);
        assert_eq!(pos.x, 12314.0);
        assert_eq!(pos.y, 55.0);
    }


    // create new entity
    ecs.spawn::<(Position, AnimationPlayer, Area)>((Position::zero(), AnimationPlayer::new(1), Area::default()));
    // assert new entity has no edited value
    for (pos, _, _) in ecs.query_comp::<(&Position, &AnimationPlayer, &Area)>() {
        dbg!("pos: {}, {}", pos.x, pos.y);
        assert_eq!(pos.x, 0.0);
        assert_eq!(pos.y, 0.0);
    }
}

#[test]
fn bulk_spawn_test() {
    let mut ecs = ECS::new();
    ecs.register_component::<Position>();
    ecs.register_component::<Velocity>();
    ecs.register_component::<Sprite>();
    ecs.register_component::<AnimationPlayer>();
    ecs.register_component::<Area>();

    for _ in 0..300 {
        ecs.spawn::<(Position, Velocity, AnimationPlayer)>
            ((Position::zero(), Velocity::zero(), AnimationPlayer::new(1)));
    }

    for _ in 0..300 {
        ecs.spawn::<(Position, Velocity, Area)>((Position::zero(), Velocity::zero(), Area::default()));
    }

    for (pos, _, _) in ecs.query_comp::<(&mut Position, &Velocity, &Area)>() {
        pos.x = 500.0;
    }

    for (pos, _, _) in ecs.query_comp::<(&mut Position, &Velocity, &AnimationPlayer)>() {
        assert_eq!(pos.x, 0.0)
    }
    
}
