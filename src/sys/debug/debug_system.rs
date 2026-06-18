use bevy_ecs::prelude::*;
use bevy_ecs::system::*;
use bevy_ecs::storage::SparseSet;
use sdl2::render::*;
use sdl2::pixels::Color;
use sdl2::rect::*;

use crate::core::renderer::*;
use crate::components::*;
use crate::resources::AreaManager;

pub fn draw_entity_areas(
    // query: Query<&Area>,
    world: &mut World,
    canvas: &mut WindowCanvas,
    renderer: &mut Renderer
) {

    for area in world.query::<&Area>().iter(world) {
        area.draw(canvas, renderer);
    }
}

pub fn render_all_obb(
    world: &mut World,
    canvas: &mut WindowCanvas,
    // renderer: &mut Renderer
) {
    let mut state: SystemState<(
        Query<&OBB>,
        NonSendMut<Renderer>
    )> = SystemState::new(world);

    let (query, mut renderer) = state.get_mut(world);


    for obb in &query {
        if obb.disabled {
            continue;
        }
        obb.draw(canvas, &mut renderer);
    }
}

pub fn draw_tile_areas(
    // mut area_m: AreaManager>,
    world: &mut World,
    canvas: &mut WindowCanvas,
    renderer: &mut Renderer
) {
    let mut area_m = world.get_resource_mut::<AreaManager>().unwrap();
    area_m.draw_tile_areas(canvas, renderer);
}

// fn draw_point(renderer: &mut Renderer, pos: Vector2) {
//     let cam_pos = renderer.camera.get_pos();
//     let cam_scale = renderer.camera.get_scale();
//     let a = (pos - cam_pos)
//         * cam_scale;
//     renderer.canvas.set_draw_color(Color::RGB(0, 0, 255));
//     renderer.canvas.fill_rect(Rect::new(
//         a.x.round() as i32 - 2, a.y.round() as i32 - 2,
//         5, 5
//     )).unwrap();
// }

// pub fn constraints(
//     mut set: ParamSet<(
//         Query<(&Constraints, Option<&Transform>)>,
//         Query<(Entity, &Transform)>
//     )>,
//     mut renderer: NonSendMut<Renderer>,
//     mut trans_list: Local<SparseSet<Entity, Transform>>
// ) {

//     trans_list.clear();
//     for (e, trans) in set.p1() {
//         trans_list.insert(e, *trans);
//     }
    
//     for (constraints, self_trans) in set.p0() {
//         if let Some(self_trans) = self_trans {
//             draw_point(&mut renderer, self_trans.pos);
//         }
//         for constraint in &constraints.0 {
//             if let Some(trans) = trans_list.get(constraint.target) {
//                 draw_point(&mut renderer, trans.pos);
//             }
//         }
//     }
// }
