use bevy_ecs::prelude::*;

use crate::components::*;
use crate::resources::{DeltaTime, asset_manager::*};
use crate::core::*;

pub fn update_sprites(
    mut player_q: Query<&mut DodgeStamina>,
    mut dodge_q: Query<(&Dodge, &mut Sprite)>,
    mut commands: Commands,
    renderer: NonSend<Renderer>
) {
    for mut dodge_stam in &mut player_q {
        if dodge_stam.dodge_entities.is_empty() {
            for i in 0..dodge_stam.max_stack {
                let mut sprite = Sprite::new( &renderer.asset_m, TextureId::DodgeStamina);
                let trans =  Transform::new(
                    (i as f32) * (sprite.width * 0.35),
                    35.0
                );

                sprite.set_sprite_sheet(2, 1);
                let e = commands.spawn((
                    sprite,
                    trans,
                    Dodge(i)
                )).id();       
                dodge_stam.dodge_entities.push(e);
            }
        }

        for e in &dodge_stam.dodge_entities {
            if let Ok((dodge, mut sprite)) = dodge_q.get_mut(*e) {
                // if the dodge id is above the stack, then
                // it must be used dodges
                if dodge.0 >= dodge_stam.stack {
                    sprite.frame = 1;
                }
                else {
                    sprite.frame = 0;
                }
            }
        }
    }
}

pub fn timer(
    mut query: Query<&mut DodgeStamina>,
    dt: Res<DeltaTime>,
    mut commands: Commands,
    renderer: NonSend<Renderer>
) {
    for mut dodge_stam in &mut query {

        if dodge_stam.in_between_timer.tick(dt.0).just_finished() {
           dodge_stam.in_between_timer.pause(); 
        }
        if dodge_stam.stack >= dodge_stam.max_stack {
            dodge_stam.timer.reset();
            continue;
        }
        if dodge_stam.timer.tick(dt.0).just_finished() {
            dodge_stam.stack = dodge_stam.max_stack;
        }

    }
}
