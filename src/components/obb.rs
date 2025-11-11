use bevy_ecs::prelude::*;
use std::vec::*;
use sdl2::rect::*;

use crate::components::*;
use crate::core::Renderer;

const PI: f32 = 3.141592;

/* WARNING: changing pub atts require that you call compute_vertices to apply */
#[derive(Component)]
pub struct OBB {
    pub center: Vector2,
    pub offset: Vector2,
    vertices: Vec<Vector2>,
    pub rotation: f32,
    width: f32,
    height: f32,
    half_extents: Vector2,
    disabled: bool
}

impl OBB {
    pub fn new(width: f32, height: f32, center: Vector2) -> Self {
        let mut obb = Self {
            center,
            offset: Vector2::zero(),
            vertices: vec!(Vector2::zero(); 4),
            rotation: 0.0,
            width,
            height,
            half_extents: Vector2::zero(),
            disabled: false,
        };
        obb.compute_vertices();

        obb
    }

    pub fn compute_vertices(&mut self) {
        let c = self.rotation.cos();
        let s = self.rotation.sin();

        let axes = Vec::from([
            Vector2::new(c, s), 
            Vector2::new(-s, c)
        ]);

        self.half_extents = Vector2::new(self.width / 2.0, self.height / 2.0);
        
        let extents = Vec::from([
            axes[0] * self.half_extents.x,
            axes[1] * self.half_extents.y
        ]);

        self.vertices[0] = self.center - (extents[0] + extents[1]);
        self.vertices[1] = self.center + (extents[0] - extents[1]);
        self.vertices[2] = self.center + (extents[0] + extents[1]);
        self.vertices[3] = self.center - (extents[0] - extents[1]);
    }

    pub fn overlapping(&self, other: OBB) -> bool {
        for i in 0..4 {
            let delta = self.vertices[(i+1) % 4] - self.vertices[i];
            let axis = Vector2::new(-delta.y, delta.x).normalize();
            
            let mut min_a: f32 = 0.0;
            let mut max_a: f32 = 0.0;
            Self::project_to_axis(axis, &self.vertices, 4, &mut min_a, &mut max_a);

            let mut min_b: f32 = 0.0;
            let mut max_b: f32 = 0.0;
            Self::project_to_axis(axis, &other.vertices, 4, &mut min_b, &mut max_b);

            if min_a > max_b || min_b > max_a { return false }
        }

        for i in 0..4 {
            let delta = other.vertices[(i+1) % 4] - other.vertices[i];
            let axis = Vector2::new(-delta.y, delta.x).normalize();
            
            let mut min_a: f32 = 0.0;
            let mut max_a: f32 = 0.0;
            Self::project_to_axis(axis, &self.vertices, 4, &mut min_a, &mut max_a);

            let mut min_b: f32 = 0.0;
            let mut max_b: f32 = 0.0;
            Self::project_to_axis(axis, &other.vertices, 4, &mut min_b, &mut max_b);

            if min_a > max_b || min_b > max_a { return false }
        }

        return true;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.compute_vertices();
    }

    fn project_to_axis(axis: Vector2, vertices: &Vec<Vector2>, num_vertices: i32, min: &mut f32, max: &mut f32) {
        *max = axis.dot(vertices[0]);
        *min = *max;

        for i in 0..num_vertices {
            let projection = axis.dot(vertices[i as usize]);

            if projection > *max { *max = projection; }
            if projection < *min { *min = projection; }
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        let cam_pos = renderer.camera.get_pos();
        let cam_scale = renderer.camera.get_scale();
        for i in 0..4 {
            let a = (self.vertices[(i+1) % 4] - cam_pos)
                * cam_scale;
            let b = (self.vertices[i] - cam_pos)
                * cam_scale;
            let _ = renderer.canvas.draw_line(
                Point::new(a.x.round() as i32, a.y.round() as i32), 
                Point::new(b.x.round() as i32, b.y.round() as i32)
            );
        }
    }

    pub fn rotate_around(&mut self, center: Vector2) {
        let ninety_deg = PI / 2.0;
        let mut rot = self.rotation;
        let mut temp_offset = self.offset;

        if self.rotation > ninety_deg || self.rotation < -ninety_deg {
            rot -= PI;
            temp_offset.x = -temp_offset.x;
        }

        let new_offset = temp_offset.rotate_around(center, rot);
        self.offset = new_offset;
    }

// for (int i = 0; i < it->count; i++) {

//     ecs_entity_t parent_e = ecs_get_target(it->world, it->entities[i], EcsChildOf, 0);

//     if (parent_e) {
//         if (!ecs_is_alive(it->world, parent_e)) {
//             continue;
//         }
//         // follow owner pos
//         const Position *owner_pos = ecs_get(it->world, parent_e, Position);
//         pos[i].global = vector2_sum(owner_pos->global, pos[i].local);

//         float ninety_deg = PI / 2.0f;
//         float rotation = obb[i].rotation;
//         Vector2 temp_offset = obb[i].offset;

//         // if rotated towards left, mirror the heck out of it
//         if (obb[i].rotation > ninety_deg || obb[i].rotation < -ninety_deg) {
//             rotation -= (float)PI;
//             temp_offset.x = -temp_offset.x;
//         }
//         // rotate around its center based on offset
//         Vector2 new_offset = v2_rotate_around(temp_offset, (Vector2){0}, rotation);
//         obb[i].center = vector2_sum(pos[i].global, new_offset);
//     }
//     else {
//         obb[i].center = vector2_sum(pos[i].global, obb->offset);
//     }
//     compute_obb_vertices(&obb[i]);
//     // rotate around it's parent if it has one
// }
}

// void area_manager_draw_obb(AreaManager* am, SDL_Renderer* renderer, OBB* area, const Camera *camera) 
// {
//     float cam_scale = camera->scale;
//     float cam_x = camera->pos.x;
//     float cam_y = camera->pos.y;
//     // pos.x -= sm->camera->position.x * cam_scale;
//     // pos.y -= sm->camera->position.y * cam_scale;
//     SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);
//     for (int i = 0; i < 4; i++) {
//         Vector2 a = vector2_sub(area->vertices[(i+1) % 4], camera->pos);
//         a = vector2_scale(a, cam_scale);
// 
//         Vector2 b = vector2_sub(area->vertices[i], camera->pos);
//         b = vector2_scale(b, cam_scale);
// 
//         SDL_RenderDrawLine(renderer, f_roundtoint(a.x), f_roundtoint(a.y), f_roundtoint(b.x), f_roundtoint(b.y));
//     }
// }



// bool obb_colliding_with_rect(OBB a, Float_Rect b)
// {
//     OBB rect_to_obb = {
//         .vertices = { 
//             (Vector2) { b.x, b.y},
//             (Vector2) { b.x, b.y + b.h },
//             (Vector2) { b.x + b.w, b.y + b.h },
//             (Vector2) { b.x + b.w, b.y},
//         },
//         .rotation = 0,
//         .width = b.w,
//         .height = b.h,
//         .center = (Vector2){b.x + b.w / 2, b.y + b.h / 2}
//     };
//     return obb_colliding(a, rect_to_obb);
// }
// 
