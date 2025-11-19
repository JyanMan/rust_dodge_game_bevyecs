use bevy_ecs::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::core::*;

pub fn player_health_bar_spawn(world: &mut World, renderer: &mut Renderer) {
    let mut sprite_health= Sprite::new(&renderer.asset_m, TextureId::HealthBar);
    sprite_health.set_sprite_sheet(1, 2);
    sprite_health.frame = 0;
    world.spawn((
        sprite_health,
        Transform::new(0.0, 0.0),
        HealthBarTag::default(),
        HealthBarFillTag::default()
    ));
    let mut sprite_health_clear = Sprite::new(&renderer.asset_m, TextureId::HealthBar);
    sprite_health_clear.set_sprite_sheet(1, 2);
    sprite_health_clear.frame = 1;
    world.spawn((
        sprite_health_clear,
        Transform::new(0.0, 0.0),
        HealthBarTag::default()
    ));
}

pub fn player_health_bar_update(
    mut query: Query<(&HealthBarFillTag, &mut Sprite)>,
    health_clear_query: Query<(&HealthBarTag, &mut Sprite), Without<HealthBarFillTag>>,
    player_health_query: Query<&Health, With<PlayerTag>>,
) {

    let mut health: Option<&Health> = None;
    let mut back_sprite: Option<&Sprite> = None;

    for p_health in player_health_query.iter() {
        health = Some(p_health); 
    }

    for (_, sprite) in health_clear_query.iter() {
        back_sprite = Some(sprite);
    }

    let health_width = if let (Some(back_sprite), Some(health)) =
        (back_sprite, health) {
        // println!("width: {}, current: {}, max: {}", back_sprite.width, health.current, health.max);
        back_sprite.width * (health.current as f32 / health.max as f32)
    }
    else { 0.0 };
    for (_, mut sprite) in query.iter_mut() {
        sprite.width = health_width;
    }
}

pub fn health_knock_timer(
    mut query: Query<(&mut Health, &mut KnockbackTrigger, &mut Velocity)>,
    delta_time: Res<DeltaTime>
) {
    for (mut health, mut knock, mut vel) in &mut query {
        health.timer(delta_time.0);

        if knock.timer(delta_time.0) {
            // knock timer ended
            vel.vec = vel.vec * 0.1;
        }
        if knock.knocked { 
            vel.vec = knock.dir * knock.knocked_force as f32;
            knock.knocked_force = (knock.knocked_force as f32 * 0.5).round() as i32;
        }
    } 
}

pub fn health_update(
    mut query: Query<(Entity, &mut Health)>,
    mut commands: Commands
) {

    for (e, health) in &mut query {
        if health.current <= 0 {
            commands.entity(e).despawn();
        }
    }
}
