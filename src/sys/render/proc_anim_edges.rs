use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
use sdl2::rect::*;
use sdl2::pixels::Color;

use crate::components::*;
use crate::resources::*;
use crate::core::*;

pub fn proc_anim_edges(
    mut renderer: NonSendMut<Renderer>,
    // proc_anim: ResMut<ProcAnim>,
    query: Query<(Entity, &Transform, Option<&DistanceConstraint>)>,
    mut trans_list: Local<SparseSet<Entity, Transform>>
) {

    trans_list.clear();
    for (e, trans, _) in &query {
        trans_list.insert(e, *trans);
    }

    for (_e, trans, constraint) in &query {
        let constraint = if let Some(constraint) = constraint {
            constraint
        } else {
            continue;
        };
        if let Some(other_e) = constraint.target
        && let Some(second) = trans_list.get(other_e) {
            let cam_pos = renderer.camera.get_pos();
            let cam_scale = renderer.camera.get_scale();
                let a = (trans.pos - cam_pos)
                    * cam_scale;
                let b = (second.pos - cam_pos)
                    * cam_scale;
            renderer.canvas.set_draw_color(Color::RGB(255, 0, 0));
            renderer.canvas.draw_line(
                Point::new(
                    a.x.round() as i32, a.y.round() as i32
                ),
                Point::new(b.x.round() as i32, b.y.round() as i32),
            ).unwrap();
        }
    }

    // for (e, list) in proc_anim.connections.iter() {
    //     let mut prev_e = e;
    //     for other_e in list {
    //         // println!("e and other_e: {}, {}", e, other_e);
    //         if let Some(first) = trans_list.get(*prev_e)
    //         && let Some(second) = trans_list.get(*other_e) {
    //             // println!("{:?}, {:?}", first.pos, second.pos);
    //             let cam_pos = renderer.camera.get_pos();
    //             let cam_scale = renderer.camera.get_scale();
    //                 let a = (first.pos - cam_pos)
    //                     * cam_scale;
    //                 let b = (second.pos - cam_pos)
    //                     * cam_scale;
    //             renderer.canvas.set_draw_color(Color::RGB(255, 0, 0));
    //             renderer.canvas.draw_line(
    //                 Point::new(
    //                     a.x.round() as i32, a.y.round() as i32
    //                 ),
    //                 Point::new(b.x.round() as i32, b.y.round() as i32),
    //             ).unwrap();
    //             prev_e = other_e;
    //         }
    //     }
    // }
    
}
