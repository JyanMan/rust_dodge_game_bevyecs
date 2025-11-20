use bevy_ecs::prelude::*;

use crate::core::renderer::*;
use crate::components::*;

pub fn sprite_system_draw(world: &mut World, renderer: &mut Renderer) {
    let mut query = world.query_filtered::<(&Transform, &Sprite), Without<HealthBarTag>>();

    for (trans, sprite) in query.iter(world) {
        if !sprite.visible {
            continue;
        }

        renderer.draw_to_cam(sprite, trans.global, sprite.scale);
    }
}

pub fn text_system_draw(world: &mut World, renderer: &mut Renderer) {
    let mut query = world.query::<&mut TextObject>();
    for mut text in query.iter_mut(world) {
        renderer.render_text(&mut text);
    }
}

pub fn health_bar_system_draw(world: &mut World, renderer: &mut Renderer) {
    let mut query = world.query_filtered::<(&Transform, &Sprite), (Without<HealthBarFillTag>, With<HealthBarTag>)>();

    for (trans, sprite) in query.iter(world) {
        if !sprite.visible {
            continue;
        }

        renderer.draw(sprite, trans.global, sprite.scale);
    }
    let mut query = world.query_filtered::<(&Transform, &Sprite), (With<HealthBarFillTag>, With<HealthBarTag>)>();

    for (trans, sprite) in query.iter(world) {
        if !sprite.visible {
            continue;
        }

        renderer.draw(sprite, trans.global, sprite.scale);
    }
}

