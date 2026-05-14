use bevy_ecs::prelude::*;

use crate::core::renderer::*;
use crate::components::*;

pub fn sprites_draw(query: Query<(&Transform, &Sprite), (Without<HealthBarTag>, Without<Dodge>)>, mut renderer: NonSendMut<Renderer>) {

    for (trans, sprite) in &query {
        if !sprite.visible {
            continue;
        }

        renderer.draw_to_cam(sprite, trans.pos, sprite.scale);
    }
}

pub fn texts_draw(
    mut query: Query<(&mut TextObject, &Transform)>,
    mut renderer: NonSendMut<Renderer>
) {
    // let mut query = world.query::<(&mut TextObject, &Transform)>();
    for (mut text, trans) in &mut query {
        text.set_pos(trans.pos);
        renderer.render_text(&mut text);
    }
}

pub fn dodge_stamina_draw(
    mut query: Query<(&Dodge, &mut Transform, &mut Sprite)>,
    mut renderer: NonSendMut<Renderer>
) {
    for (dodge, mut trans, mut sprite) in &mut query {
        // println!("wtf??");
        renderer.draw(&sprite, trans.pos, sprite.scale);
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

        renderer.draw(sprite, trans.pos, sprite.scale);
    }
    for (trans, sprite) in &mut query_without {
        if !sprite.visible {
            continue;
        }

        renderer.draw(sprite, trans.pos, sprite.scale);
    }
}

