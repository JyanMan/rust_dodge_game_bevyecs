use bevy_ecs::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::core::*;
use crate::systems::*;

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

    let text_str = format!("Hp: {} / {}", 0, 0);
    let text_e = spawn_text(world, renderer, text_str.as_str(), 8, Vector2::new(0.0, 30.0));
    let mut text_ref = world.entity_mut(text_e);
    text_ref.insert(HealthBarTextTag::default());

}

pub fn player_health_bar_update(
    mut query: Query<(&HealthBarFillTag, &mut Sprite)>,
    health_clear_query: Query<(&HealthBarTag, &mut Sprite), Without<HealthBarFillTag>>,
    player_health_query: Query<&Health, With<PlayerTag>>,
    mut health_bar_text_query: Query<&mut TextObject, With<HealthBarTextTag>>,
) {

    let mut health: Option<&Health> = None;
    let mut back_sprite: Option<&Sprite> = None;

    for p_health in player_health_query.iter() {
        health = Some(p_health); 
        for mut text in &mut health_bar_text_query {
            let text_str = format!("Hp: {} / {}", p_health.current, p_health.max);
            text.set_content(text_str.as_str()); 
        }
    }

    for (_, sprite) in health_clear_query.iter() {
        back_sprite = Some(sprite);
    }

    let health_width = if let (Some(back_sprite), Some(health)) =
        (back_sprite, health) {
        back_sprite.width * (health.current as f32 / health.max as f32)
    }
    else { 0.0 };
    for (_, mut sprite) in query.iter_mut() {
        sprite.width = health_width;
    }
}

pub fn damage_counter_update(
    mut query: Query<(&mut DamageCounterTimer, &mut Transform)>,
    delta_time: Res<DeltaTime>,
) {
    for (mut timer, mut trans) in &mut query {
        if timer.0 < 0.0 {
            continue;
        }
        timer.timer(delta_time.0);
        // increase at a decreasing rate
        trans.global.y -= 0.1 / timer.0;
    }
}

pub fn damage_counter_despawn_update(
    world: &mut World,
    renderer: &mut Renderer
) {
   let mut query = world.query::<(Entity, &DamageCounterTimer, &TextObject)>(); 
   let mut temp_vec: Vec<Entity> = vec![];
   for (e, timer, text) in query.iter(world) {
       if timer.0 < 0.0 {
           renderer.delete_text(text);
           temp_vec.push(e);
       }
   }
   for e in temp_vec.iter() {
      world.despawn(*e);
   }
}

pub fn health_knock_timer(
    mut query: Query<(&mut Health, &mut KnockbackTrigger, &mut Velocity)>,
    delta_time: Res<DeltaTime>,
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
