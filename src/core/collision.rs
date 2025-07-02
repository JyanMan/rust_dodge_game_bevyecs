use std::f64;
use crate::components::area::*;
use crate::components::velocity::*;
use crate::components::position::*;
use crate::config::*;
use crate::managers::area_manager::*;
use crate::math_helper::*;

const INFINITY: f32 = f64::INFINITY as f32;
const EPSILON: f32 = 0.001;

pub fn aabb_is_colliding(a: &Area, b: &Area) -> bool {
    return 
        a.x + a.w > b.x &&
        a.x < b.x + b.w &&
        a.y + a.h > b.y &&
        a.y < b.y + b.h
    ;
}

pub fn collision_overlap(a: &Area, b: &Area) -> Position
{
     // Calculation of centers of rectangles
    let center1 = Position::new( a.x + a.w / 2.0, a.y + a.h / 2.0 );
    let center2 = Position::new( b.x + b.w / 2.0, b.y + b.h / 2.0 );

    // Calculation of the distance vector between the centers of the rectangles
    let delta = center1 - center2;

    // Calculation of half-widths and half-heights of rectangles
    let hs1 = Position::new(a.w *0.5, a.h *0.5 );
    let hs2 = Position::new( b.w *0.5, b.h *0.5 );

    // Calculation of the minimum distance at which the two rectangles can be separated
    let min_dist_x = hs1.x + hs2.x - (delta.x).abs();
    let min_dist_y = hs1.y + hs2.y - (delta.y).abs();

    let mut adjusted_pos = Position::zero();
    // Adjusted object position based on minimum distance
    if min_dist_x < min_dist_y {
        adjusted_pos.x += min_dist_x.copysign(delta.x);
    } else {
        adjusted_pos.y += min_dist_y.copysign(delta.y);
    }
    return adjusted_pos;
}


pub fn swept_aabb(moving: &Area, vel: &Velocity, target: &Area, normal_out: &mut Position ) -> f32 {
    //Broad-phase check: if there's no overlap in the axis perpendicular to movement, skip
    // check for vertical overlap
    if vel.x != 0.0 && (moving.y + moving.h <= target.y || moving.y >= target.y + target.h) {
        return 1.0
    }
    // check for horizontal overlap
    if vel.y != 0.0 && ((moving.x + moving.w <= target.x || moving.x >= target.x + target.w)) {
        return 1.0
    }

    // calculate inverse entry and exit times
    let mut x_inv_entry: f32 = 0.0;
    let mut y_inv_entry: f32 = 0.0;
    let mut x_inv_exit: f32 = 0.0;
    let mut y_inv_exit: f32 = 0.0;

    // calc horizontal distance of entry and exit
    if vel.x > 0.0 {
        x_inv_entry = target.x - (moving.x + moving.w);
        x_inv_exit = (target.x + target.w) - moving.x;
    }
    else if vel.x < 0.0 {
        x_inv_entry = (target.x + target.w) - moving.x;
        x_inv_exit = target.x - (moving.x + moving.w);
    }

    // calc vertical distance of entry and exit
    if vel.y > 0.0 {
        y_inv_entry = target.y - (moving.y + moving.h);
        y_inv_exit = (target.y + target.h) - moving.y;
    } 
    else if vel.y < 0.0 {
        y_inv_entry = (target.y + target.h) - moving.y;
        y_inv_exit = target.y - (moving.y + moving.h);
    }

    // calc horizonal time of entry and exit
    let x_entry: f32 = if vel.x.abs() <= EPSILON {
        -INFINITY  
    } else {
        x_inv_entry / vel.x
    };

    let x_exit = if vel.x.abs() <= EPSILON {
        INFINITY  
    } else {
        x_inv_exit / vel.x
    };

    // calc vertical time of entry and exit
    let y_entry = if vel.y.abs() <= EPSILON {
        -INFINITY  
    } else {
        y_inv_entry / vel.y
    };
    let y_exit = if vel.y.abs() <= EPSILON {
        INFINITY
    } else {
        y_inv_exit / vel.y
    };

    // get later collision time to ensure both axis overlap
    let entry_time = x_entry.max(y_entry);
    // get the closer time, no overlap on one axis means no collision
    let exit_time = x_exit.min(y_exit);

    if entry_time > exit_time || x_entry < 0.0 && y_entry < 0.0 || x_entry > 1.0 || y_entry > 1.0 {
        return 1.0;
        // no collision
    }
    if x_entry > y_entry {
        normal_out.x = if x_inv_entry < 0.0 { 1.0 } else { -1.0 };
        normal_out.y = 0.0;
    } else {
        normal_out.x = 0.0;
        normal_out.y = if y_inv_entry < 0.0 { 1.0 } else { -1.0 };
    }

    return entry_time
}

fn aabb_resolve(
    area: &mut Area, 
    pos: &mut Position,
    curr_vel: &mut Velocity,
    axis_motion: &Velocity, 
    motion: &Velocity, 
    grounded: &mut bool, 
    area_m: &mut AreaManager
) {

    let mut earliest_ct: f32 = 1.0;
    let mut hit_area: Option<&Area> = None;
    let mut earliest_normal = Position::zero();

    let start = Position::new(area.x + area.offset.x, area.y + area.offset.y);
    let end = Position::new(
        start.x + motion.x, start.y + motion.y
        );
    //let end = start + motion;

    let buffer = TILE_SIZE as f32;
    let swept = Area::new(
        start.x.min(end.x) - buffer,
        start.y.min(end.y) - buffer,
        area.w + (end.x - start.x).abs() + buffer * 2.0,
        area.h + (end.y - start.y).abs() + buffer * 2.0
    );

    let min_tile = world_to_tile(&Position::new(swept.x, swept.y));
    let max_tile = world_to_tile(&Position::new(swept.x + swept.w, swept.y + swept.h));

    for y in min_tile.y..max_tile.y {
        for x in min_tile.x..max_tile.x {
            let tile_pos = Point::new(x, y);

            let tile_area = if let Some(t_area) = area_m.get_tile_area(&tile_pos) {
                t_area
            } else {
                continue; 
            };

            // if aabb_is_colliding(&area, &tile_area) {
            //     // entity update bounds comes first because the bounds is not the same as entity position
            //     // requires an update, otherwise sprite is ahead, you resolve delayed bounds
            //     // entity_update_bounds(e);

            //     let adjusted_pos = collision_overlap(&area, &tile_area);
            //     pos.x += adjusted_pos.x;
            //     pos.y += adjusted_pos.y;

            //     if adjusted_pos.y < 0.0 {
            //     }
            //     *grounded = true;

            //     if adjusted_pos.y.abs() >= EPSILON {
            //         curr_vel.y = 0.0;
            //     }
            //     if adjusted_pos.x.abs() >= EPSILON  {
            //         curr_vel.x = 0.0;
            //     }
            //     area.update_pos(pos.x, pos.y);
            //     continue;
            // }
            // continue;

            let mut normal = Position::zero();
            let collision_time = swept_aabb(area, axis_motion, tile_area, &mut normal);
            if collision_time == 1.0 {
                // no collision
                if aabb_is_colliding(&area, &tile_area) {
                    // entity update bounds comes first because the bounds is not the same as entity position
                    // requires an update, otherwise sprite is ahead, you resolve delayed bounds
                    // entity_update_bounds(e);
                    area.update_pos(pos.x, pos.y);

                    let adjusted_pos = collision_overlap(&area, &tile_area);
                    pos.x += adjusted_pos.x;
                    pos.y += adjusted_pos.y;

                    if adjusted_pos.y < 0.0 {
                        *grounded = true;
                    }

                    if adjusted_pos.y.abs() >= EPSILON {
                        curr_vel.y = 0.0;
                    }
                    if adjusted_pos.x.abs() >= EPSILON  {
                        curr_vel.x = 0.0;
                    }
                }
                continue;
            }

            if collision_time < earliest_ct {
                earliest_ct = collision_time;
                earliest_normal = normal;
                hit_area = Some(tile_area);
            }
        }
    }


    // adjust based on earliest future collision
    if let Some(_area) = hit_area {
        if earliest_ct >= 1.0 {
            return;
        }
        *pos = Position::new(
            pos.x + axis_motion.x * earliest_ct,
            pos.y + axis_motion.y * earliest_ct,
        );
        // pos.x += axis_motion.x * earliest_ct;
        // pos.y += axis_motion.y * earliest_ct;
        if earliest_normal.y < 0.0 {
            *grounded = true;
        }         // entity_update_bounds(e);
        let _remaining_time = 1.0 - earliest_ct;
        // slide 
        if axis_motion.x != 0.0 && earliest_normal.x != 0.0 {
            curr_vel.x = 0.0;
        }
        if axis_motion.y != 0.0 && earliest_normal.y != 0.0 {
            curr_vel.y = 0.0;
        }
    }
}

pub fn area_colliding_to_tile(
    entity_area: &mut Area,
    entity_pos: &mut Position,
    vel: &mut Velocity, grounded: &mut bool, area_m: &mut AreaManager, time_step: f32
) {
    *grounded = false;
    let mut e_motion = vel.clone() * time_step;

    let mut y_motion = Velocity::new(0.0, e_motion.y);
    aabb_resolve(entity_area, entity_pos, vel, &mut y_motion, &mut e_motion, grounded, area_m);

    let mut dummy_grounded = false;
    let mut x_motion = Velocity::new(e_motion.x, 0.0);
    aabb_resolve(entity_area, entity_pos, vel, &mut x_motion, &mut e_motion, &mut dummy_grounded, area_m);
}

//             // area->debug = true;
//             Vector2 normal;
//             float collision_time = swept_aabb(e->area, vel, *area, &normal);
// 
//             if (collision_time == 1.0f) {
//                 // swept_aabb has edge cases, static collision resolution helps solve its issues
//                 if (abb_is_colliding(&e->area, area)) {
// 
//                     // entity update bounds comes first because the bounds is not the same as entity position
//                     // requires an update, otherwise sprite is ahead, you resolve delayed bounds
//                     entity_update_bounds(e);
// 
//                     Vector2 adjusted_pos = collision_overlap(&e->area, area);
//                     e->pos.x += adjusted_pos.x;
//                     e->pos.y += adjusted_pos.y;
//                     if (fabsf(adjusted_pos.y) >= EPSILON) e->velocity.y = 0.0f;
//                     if (fabsf(adjusted_pos.x) >= EPSILON) e->velocity.x = 0.0f;
//                 }
//                 continue;
//             }
//             
//             // set only the closest area to be collided with to adjust player pos
//             // closest is the one with shortest collising time
//             if (collision_time < earliest_ct) {
//                 earliest_ct = collision_time;
//                 earliest_normal = normal;
//                 hit_area = area;
//             }
//         }
//     }
// 
//     // adjust based on earliest future collision
//     if (hit_area != NULL && earliest_ct < 1.0f) {
//         e->pos.x += vel.x * earliest_ct;
//         e->pos.y += vel.y * earliest_ct;
//         if (earliest_normal.y < 0) *grounded = true;
//         // entity_update_bounds(e);
//         float remaining_time = 1.0f - earliest_ct;
//         // slide 
//         if (vel.x != 0 && earliest_normal.x != 0) {
//             e->velocity.x = 0;
//         }
//         if (vel.y != 0 && earliest_normal.y != 0) e->velocity.y = 0;
//     }
// }

