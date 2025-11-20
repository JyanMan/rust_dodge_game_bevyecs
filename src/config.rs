pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 600;
pub const TILE_SIZE: i32 = 8;
pub const CHUNK_SIZE: i32 = 16;
pub const CELL_SIZE: i32 = TILE_SIZE * CHUNK_SIZE;

pub const KNOCK_TIME: f32 = 0.3;
pub const IMMUNE_TIME: f32 = 0.1;

#[allow(dead_code)]
pub const HALF_WIDTH: i32 = SCREEN_WIDTH / 2;
#[allow(dead_code)]
pub const HALF_HEIGHT: i32 = SCREEN_HEIGHT / 2;
pub const HALF_WIDTH_F: f32 = SCREEN_WIDTH as f32 / 2.0;
pub const HALF_HEIGHT_F: f32 = SCREEN_HEIGHT as f32 / 2.0;

pub const MAX_ENTITIES: i32 = 5000;

pub const DAMAGE_COUNTER_TIME: f32 = 0.3;
