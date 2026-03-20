use bevy_ecs::prelude::*;

use crate::core::renderer::*;
use crate::components::*;

pub fn sprites_draw(query: Query<(&Transform, &Sprite), Without<HealthBarTag>>, mut renderer: NonSendMut<Renderer>) {

    for (trans, sprite) in &query {
        if !sprite.visible {
            continue;
        }

        renderer.draw_to_cam(sprite, trans.global, sprite.scale);
    }
}

pub fn texts_draw(
    mut query: Query<(&mut TextObject, &Transform)>,
    mut renderer: NonSendMut<Renderer>
) {
    // let mut query = world.query::<(&mut TextObject, &Transform)>();
    for (mut text, trans) in &mut query {
        text.set_pos(trans.global);
        renderer.render_text(&mut text);
    }
}

pub fn health_bar_draw(
    mut query: Query<(&Transform, &Sprite), (Without<HealthBarFillTag>, With<HealthBarTag>)>,
    mut query_without: Query<(&Transform, &Sprite), (With<HealthBarFillTag>, With<HealthBarTag>)>,
    mut renderer: NonSendMut<Renderer>
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

