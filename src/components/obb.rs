use bevy_ecs::prelude::*;
use std::vec::*;
use sdl2::rect::*;

use crate::components::*;
use crate::core::Renderer;

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
