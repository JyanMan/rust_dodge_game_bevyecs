use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
use sdl2::rect::*;
use sdl2::render::*;
use sdl2::pixels::Color;
use std::vec::Vec;
use i_triangle::float::triangulatable::Triangulatable;

use crate::components::*;
use crate::resources::*;
use crate::core::*;

pub fn proc_anim_edges(
    mut renderer: NonSendMut<Renderer>,
    // proc_anim: ResMut<ProcAnim>,
    query: Query<(Entity, &Transform)>,
    polygons: Query<&PolygonId>,
    mut trans_list: Local<SparseSet<Entity, Transform>>
) {

    trans_list.clear();
    for (e, trans) in &query {
        trans_list.insert(e, *trans);
    }

    let mut test = Vec::new();

    let cam_pos = renderer.camera.get_pos();
    let cam_scale = renderer.camera.get_scale();

    for polygon in &polygons {
        if polygon.list.len() < 2 {
            continue;
        }
        if let Some(first) = trans_list.get(polygon.list[0]) {
            let a = (first.pos - cam_pos)
                * cam_scale;
            test.push( [a.x, a.y], );
        }

        for i in 1..polygon.list.len() {
            let prev = polygon.list[i-1]; 
            let next = polygon.list[i]; 
            if let Some(prev) = trans_list.get(prev)
            && let Some(next) = trans_list.get(next) {
                let a = (prev.pos - cam_pos)
                    * cam_scale;
                let b = (next.pos - cam_pos)
                    * cam_scale;

                test.push( [b.x, b.y], );

                renderer.canvas.set_draw_color(Color::RGB(255, 255, 0));
                renderer.canvas.draw_line(
                    Point::new(
                        a.x.round() as i32, a.y.round() as i32
                    ),
                    Point::new(b.x.round() as i32, b.y.round() as i32),
                ).unwrap();

                renderer.canvas.set_draw_color(Color::RGB(255, 0, 0));
                renderer.canvas.draw_rect(
                    Rect::new(
                        a.x.round() as i32, a.y.round() as i32,
                        5, 5
                    )
                ).unwrap();
            }
        }
    }
    // for (_e, trans, constraint, anchor) in &query {
    //     // let constraint = if let Some(constraint) = constraint {
    //     //     constraint
    //     // } else {
    //     //     continue;
    //     // };
    //     if let Some(constraint) = constraint
    //     && let Some(other_e) = constraint.target()
    //     && let Some(second) = trans_list.get(other_e) {
    //         let cam_pos = renderer.camera.get_pos();
    //         let cam_scale = renderer.camera.get_scale();
    //         let a = (trans.pos - cam_pos)
    //             * cam_scale;
    //         let b = (second.pos - cam_pos)
    //             * cam_scale;

    //         test.push(
    //                 [a.x, a.y],
    //             // Vertex {
    //             //     position: FPoint::new(a.x, a.y),
    //             //     color: Color::RGB(0, 0, 255),
    //             //     tex_coord: FPoint::new(1.0, 1.0)
    //             // }
    //         );

    //         renderer.canvas.set_draw_color(Color::RGB(255, 255, 0));
    //         renderer.canvas.draw_line(
    //             Point::new(
    //                 a.x.round() as i32, a.y.round() as i32
    //             ),
    //             Point::new(b.x.round() as i32, b.y.round() as i32),
    //         ).unwrap();

    //         renderer.canvas.set_draw_color(Color::RGB(255, 0, 0));
    //         renderer.canvas.draw_rect(
    //             Rect::new(
    //                 a.x.round() as i32, a.y.round() as i32,
    //                 5, 5
    //             )
    //         ).unwrap();
    //     }
    //     else if anchor.is_some() {
    //         let cam_pos = renderer.camera.get_pos();
    //         let cam_scale = renderer.camera.get_scale();
    //         let a = (trans.pos - cam_pos)
    //             * cam_scale;
    //         test.push([a.x, a.y]);
            
    //     }
    // }
    if !test.is_empty() {
        let trian = test.triangulate().to_triangulation::<u16>();
        let mut vertices = Vec::new();
        for i in &trian.indices {
            let p = trian.points[*i as usize];
            vertices.push(Vertex {
                position: FPoint::new(p[0], p[1]),
                color: Color::RGB(255, 255, 0),
                tex_coord: FPoint::new(0.0, 0.0)
            })
        }
        // println!("points_len {}, indices len {}, vertices len: {}", trian.points.len(), trian.indices.len(), vertices.len());
        if vertices.len() % 3 == 0 && !vertices.is_empty() {
            renderer.render_geometry(&vertices, TextureId::Player, VertexIndices::Sequential).unwrap();
        }
       
    }
    // let texture = renderer.asset_m.get_texture(TextureId::Player); 
    // if !test.is_empty() {
    //     renderer.render_geometry(&test, TextureId::Player, VertexIndices::Sequential).unwrap();
    //     // renderer.canvas.draw_polygon()
    // }

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
