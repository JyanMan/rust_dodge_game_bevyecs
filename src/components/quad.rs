use bevy_ecs::prelude::*;
use std::vec::Vec;

use crate::math_helper::*;

#[derive(Component, Default)]
pub struct CellPos(pub Vec<Point>);
