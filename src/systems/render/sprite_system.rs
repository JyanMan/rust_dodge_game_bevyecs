use bevy_ecs::prelude::*;

use crate::core::renderer::*;
use crate::components::*;

pub fn sprite_system_draw(query: Query<(&Transform, &Sprite), Without<HealthBarTag>>, mut renderer: NonSendMut<Renderer<'static>>) {

    for (trans, sprite) in &query {
        if !sprite.visible {
            continue;
        }

        renderer.draw_to_cam(sprite, trans.global, sprite.scale);
    }
}

pub fn text_system_draw(
    mut query: Query<(&mut TextObject, &Transform)>,
    mut renderer: NonSendMut<Renderer<'static>>
) {
    // let mut query = world.query::<(&mut TextObject, &Transform)>();
    for (mut text, trans) in &mut query {
        text.set_pos(trans.global);
        renderer.render_text(&mut text);
    }
}

pub fn health_bar_system_draw(
    mut query: Query<(&Transform, &Sprite), (Without<HealthBarFillTag>, With<HealthBarTag>)>,
    mut query_without: Query<(&Transform, &Sprite), (With<HealthBarFillTag>, With<HealthBarTag>)>,
    mut renderer: NonSendMut<Renderer<'static>>
) {
    for (trans, sprite) in &mut query {
        if !sprite.visible {
            continue;
        }

        renderer.draw(sprite, trans.global, sprite.scale);
    }
    for (trans, sprite) in &mut query_without {
        if !sprite.visible {
            continue;
        }

        renderer.draw(sprite, trans.global, sprite.scale);
    }
}

