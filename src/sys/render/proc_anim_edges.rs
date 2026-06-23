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
use crate::config::*;

pub fn proc_anim_edges(
    // world: &mut World,

    mut draw_list: ResMut<DrawList>,
    // proc_anim: ResMut<ProcAnim>,
    query: Query<(Entity, &Transform)>,
    polygons: Query<&PolygonId>,
    mut trans_list: Local<SparseSet<Entity, Transform>>
) {
    // TODO

    // let mut state: SystemState<(
    //     Query<(Entity, &Transform)>,
    //     Query<&PolygonId>,
    //     NonSendMut<Renderer>
    // )> = SystemState::new(world);

    // let (query, polygons, mut renderer) = state.get_mut(world);


    // let mut query = world.query::<(Entity, &Transform)>();
    // let mut polygons= world.query::<&PolygonId>();

    trans_list.clear();
    for (e, trans) in &query {
        trans_list.insert(e, *trans);
    }

    let mut vertices = Vec::new();

    for polygon in &polygons {
        if polygon.list.len() < 2 {
            continue;
        }
        if let Some(first) = trans_list.get(polygon.list[0]) {
            let a = first.pos;
            vertices.push( [a.x, a.y], );
        }

        for i in 1..polygon.list.len() {
            let prev = polygon.list[i-1]; 
            let next = polygon.list[i]; 
            if let Some(prev) = trans_list.get(prev)
            && let Some(next) = trans_list.get(next) {
                let b = next.pos;

                vertices.push( [b.x, b.y], );

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
        if points.len() % 3 == 0 && !points.is_empty() {

            draw_list.draw(Draw {
                cmd: DrawCommand::Geometry(GeometryParams::new(
                    points,
                    TextureId::BloodParticle
                )),
                kind: DrawKind::Both
            }, 2);
        }
    }
}
