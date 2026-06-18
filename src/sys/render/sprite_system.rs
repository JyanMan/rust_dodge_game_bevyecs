use bevy_ecs::prelude::*;
use std::f64::consts::PI;
use sdl2::render::*;
use bevy_ecs::system::*;

use crate::core::renderer::*;
use crate::components::*;

#[allow(clippy::type_complexity)]
pub fn sprites_draw(
    // query: Query<
    //     (&Transform, &Sprite, &LocalTransform),
    //     (Without<HealthBarTag>, Without<Dodge>)
    // >,
    world: &mut World,
    canvas: &mut WindowCanvas,
    // renderer: &mut Renderer
) {

    let mut state: SystemState<(
        Query<(&Transform, &Sprite, &LocalTransform),
        (Without<HealthBarTag>, Without<Dodge>)>, NonSendMut<Renderer>
    )> = SystemState::new(world);

    let (query, mut renderer) = state.get_mut(world);
    for (trans, sprite, local) in &query
    {
        if !sprite.visible {
            continue;
        }

        // let angle_deg_trans = local.rot * (180.0 / PI);
        // println!("angle_deg_trans: {}", angle_deg_trans);

        let trans_angle = local.rot as f64 * (180.0 / PI);

        let angle_deg = trans_angle + sprite.angle;

        // adjust sprite angle
        let angle_deg =  if trans.scale.x < 0.0 {
            -angle_deg
        }
        else {
            angle_deg
        };
        let new_scale = Vector2::new(
            sprite.scale.x * trans.scale.x,  
            sprite.scale.y * trans.scale.y,  
        );
        renderer.draw_to_cam(canvas, sprite, trans.pos, new_scale, angle_deg);
    }
}

// pub fn sprite_update_trans(
//     mut query: Query<(&mut Sprite, &LocalTransform, &Transform)>
// ) {
//     for (mut sprite, local, trans) in &mut query {
//         // let angle_deg = local.rot as f64 * (180.0 / PI);

//         // adjust sprite angle
//         // if trans.scale.x < 0.0 {
//         //     sprite.angle = -angle_deg;
//         //     sprite.flip_x = true;
//         // }
//         // else {
//         //     sprite.angle = angle_deg;
//         //     sprite.flip_x = false;
//         // }
//     }
// }

pub fn texts_draw(
    
    world: &mut World,
    canvas: &mut WindowCanvas,
    // renderer: &mut Renderer
    // mut query: Query<(&mut TextObject, &Transform)>,
    // mut renderer: NonSendMut<Renderer>
) {
    let mut state: SystemState<(
        Query<(&mut TextObject, &Transform)>,
        NonSendMut<Renderer>
    )> = SystemState::new(world);

    let (mut query, mut renderer) = state.get_mut(world);

    for (mut text, trans) in &mut query {
        text.set_pos(trans.pos);
        renderer.render_text(canvas, &mut text);
    }
}

pub fn dodge_stamina_draw(
    
    world: &mut World,
    canvas: &mut WindowCanvas,
    // mut query: Query<(&Dodge, &mut Transform, &mut Sprite)>,
    // mut renderer: NonSendMut<Renderer>
) {
    let mut state: SystemState<(
        Query<(&Dodge, &mut Transform, &mut Sprite)>,
        NonSendMut<Renderer>
    )> = SystemState::new(world);

    let (mut query, mut renderer) = state.get_mut(world);

    for (dodge, mut trans, mut sprite) in &mut query {
        // println!("wtf??");
        renderer.draw(canvas, &sprite, trans.pos, sprite.scale);
    }
   
}

#[allow(clippy::type_complexity)]
pub fn health_bar_draw(
    
    world: &mut World,
    canvas: &mut WindowCanvas,
    // renderer: &mut Renderer
    // mut query: Query<(&Transform, &Sprite), (Without<HealthBarFillTag>, With<HealthBarTag>)>,
    // mut query_without: Query<(&Transform, &Sprite), (With<HealthBarFillTag>, With<HealthBarTag>)>,
    // mut renderer: NonSendMut<Renderer>
) {
    // let mut query = world.query_filtered::<(&Transform, &Sprite), (Without<HealthBarFillTag>, With<HealthBarTag>)>();
    let mut state: SystemState<(
        Query<(&Transform, &Sprite), (Without<HealthBarFillTag>, With<HealthBarTag>)>,
        Query<(&Transform, &Sprite), (With<HealthBarFillTag>, With<HealthBarTag>)>,
        NonSendMut<Renderer>
    )> = SystemState::new(world);

    let (query, query_without, mut renderer) = state.get_mut(world);

    for (trans, sprite) in &query {
        if !sprite.visible {
            continue;
        }

        renderer.draw(canvas, sprite, trans.pos, sprite.scale);
    }
    // let mut query_without = world.query_filtered::<(&Transform, &Sprite), (With<HealthBarFillTag>, With<HealthBarTag>)>();
    for (trans, sprite) in &query_without {
        if !sprite.visible {
            continue;
        }

        renderer.draw(canvas, sprite, trans.pos, sprite.scale);
    }
}

