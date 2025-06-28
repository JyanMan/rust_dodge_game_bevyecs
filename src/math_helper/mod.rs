use crate::config::*;
use crate::components::position::*;
use std::ops;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn lerp_pos(a: &Position, b: &Position, t: f32) -> Position {
    Position {
        x: lerp(a.x, b.x, t),
        y: lerp(a.y, b.y, t),
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32, pub y: i32
}


impl ops::Mul<f32> for Point {
    type Output = Point;

    fn mul(self, scale: f32) -> Point {
        Point {
            x: self.x * (scale as i32),
            y: self.y * (scale as i32)
        }
    }
}

#[inline(always)]
pub fn world_to_chunk(world_pos: &Position) -> Point {
    return Point {
        x: (world_pos.x / ((CHUNK_SIZE * TILE_SIZE) as f32)).floor() as i32,
        y: (world_pos.y / ((CHUNK_SIZE * TILE_SIZE) as f32)).floor() as i32
    };
}

#[inline(always)]
pub fn chunk_to_world(chunk_pos: &Point) -> Position {
    return Position {
        x: (chunk_pos.x * CHUNK_SIZE * TILE_SIZE) as f32,
        y: (chunk_pos.y * CHUNK_SIZE * TILE_SIZE) as f32,
    };
}

#[inline(always)]
pub fn world_to_tile(world_pos: &Position) -> Point {
    return Point {
        x: (world_pos.x / ((TILE_SIZE) as f32)).floor() as i32,
        y: (world_pos.y / ((TILE_SIZE) as f32)).floor() as i32
    };
}

#[inline(always)]
pub fn tile_to_world(tile_pos: Point) -> Position {
    return Position {
        x: (tile_pos.x * TILE_SIZE) as f32,
        y: (tile_pos.y * TILE_SIZE) as f32,
    };
}

pub fn pos_to_point(vec: Position) -> Point {
    return Point { x: vec.x as i32, y: vec.y as i32 };
}
