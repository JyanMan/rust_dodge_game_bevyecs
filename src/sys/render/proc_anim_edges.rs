use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
use bevy_ecs::system::*;
use sdl2::rect::*;
use sdl2::render::*;
use sdl2::pixels::Color;
use std::vec::Vec;
use i_triangle::float::triangulatable::Triangulatable;

use crate::components::*;
use crate::resources::*;
use crate::core::*;

pub fn proc_anim_edges(
    world: &mut World,
    canvas: &mut WindowCanvas,
    // renderer: &mut Renderer,
    // proc_anim: ResMut<ProcAnim>,
    // query: Query<(Entity, &Transform)>,
    // polygons: Query<&PolygonId>,
    trans_list: &mut SparseSet<Entity, Transform>
) {

    let mut state: SystemState<(
        Query<(Entity, &Transform)>,
        Query<&PolygonId>,
        NonSendMut<Renderer>
    )> = SystemState::new(world);

    let (query, polygons, mut renderer) = state.get_mut(world);


    // let mut query = world.query::<(Entity, &Transform)>();
    // let mut polygons= world.query::<&PolygonId>();

    trans_list.clear();
    for (e, trans) in &query {
        trans_list.insert(e, *trans);
    }

    let mut vertices = Vec::new();

    let cam_pos = renderer.camera.get_pos();
    let cam_scale = renderer.camera.get_scale();

    for polygon in &polygons {
        if polygon.list.len() < 2 {
            continue;
        }
        if let Some(first) = trans_list.get(polygon.list[0]) {
            let a = (first.pos - cam_pos)
                * cam_scale;
            vertices.push( [a.x, a.y], );
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

                vertices.push( [b.x, b.y], );

                // renderer.canvas.set_draw_color(Color::RGB(255, 255, 0));
                // renderer.canvas.draw_line(
                //     Point::new(
                //         a.x.round() as i32, a.y.round() as i32
                //     ),
                //     Point::new(b.x.round() as i32, b.y.round() as i32),
                // ).unwrap();

                // renderer.canvas.set_draw_color(Color::RGB(255, 0, 0));
                // renderer.canvas.draw_rect(
                //     Rect::new(
                //         a.x.round() as i32, a.y.round() as i32,
                //         5, 5
                //     )
                // ).unwrap();
            }
        }
    }
    if !vertices.is_empty() {
        let trian = vertices.triangulate().to_triangulation::<u16>();
        let mut points = Vec::new();
        for i in &trian.indices {
            let p = trian.points[*i as usize];
            points.push(Vertex {
                position: FPoint::new(p[0], p[1]),
                color: Color::RGB(255, 255, 0),
                tex_coord: FPoint::new(0.0, 0.0)
            })
        }
        // println!("points_len {}, indices len {}, vertices len: {}", trian.points.len(), trian.indices.len(), vertices.len());
        if points.len() % 3 == 0 && !points.is_empty() {
            renderer.render_geometry(canvas, &points, TextureId::BloodParticle, VertexIndices::Sequential).unwrap();
        }
    }
}
