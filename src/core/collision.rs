use std::cmp::*;
use std::f64;
use crate::components::area::*;
use crate::components::velocity::*;
use crate::components::position::*;

const INFINITY: f32 = f64::INFINITY as f32;
const EPSILON: f32 = 0.001;

pub fn swept_abb(moving: Area, vel: Velocity, target: Area, normal_out: &mut Position ) -> f32 {
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

    if entry_time > exit_time || entry_time < 0.0 || entry_time > 1.0 {
        return 1.0;
        // no collision
    }
    if x_entry > y_entry {
        normal_out.x = if vel.x < 0.0 { 1.0 } else { -1.0 };
        normal_out.y = 0.0;
    } else {
        normal_out.x = 0.0;
        normal_out.y = if vel.y < 0.0 { 1.0 } else { -1.0 };
    }
    // if entry_time > exit_time || x_entry < 0.0 && y_entry < 0.0 || x_entry > 1.0 || y_entry > 1.0 {
    //     return 1.0 // no collision
    // }
    // choose greater axis
    // if x_entry > y_entry {
    //     normal_out.x = if x_inv_entry < 0.0 { 1.0 } else { -1.0 }; 
    //     normal_out.y = 0.0;
    // }
    // else {
    //     normal_out.x = 0.0;
    //     normal_out.y = if y_inv_entry < 0.0 { 1.0 } else { -1.0 }
    // }

    return entry_time
}

