use bevy_ecs::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::core::renderer::*;
use rand::*;

pub fn spawn(commands: &mut Commands, start_pos: Vector2, damage: i32) {
    let mut rng = rand::thread_rng(); 
    let text = format!("{}", damage);
    let x_rand = rng.gen_range(-1..1) as f32;
    let y_rand = rng.gen_range((-1)..0) as f32;
    let offset = Vector2::new(x_rand, y_rand).normalize() * 10.0;
    let pos = start_pos + offset;
    commands.spawn((
        TextObject::new(text.as_str(), 4, pos, true),
        DamageCounterTimer::new(),
        Transform::new(pos.x, pos.y)
    ));
}

pub fn update(
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

pub fn despawn_update(
    query: Query<(Entity, &DamageCounterTimer, &TextObject)>,
    mut renderer: NonSendMut<Renderer>,
    mut commands: Commands
) {
   let mut temp_vec: Vec<Entity> = vec![];
   for (e, timer, text) in &query {
       if timer.0 < 0.0 {
           renderer.delete_text(text);
           temp_vec.push(e);
       }
   }
   for e in temp_vec.iter() {
      commands.entity(*e).despawn();
   }
}
