use specs::prelude::*;

use common::*;
use simple_map::SimpleMapBuilder;

use super::{Map, Position, Rect, spawner, TileType};

mod simple_map;
mod common;

trait MapBuilder {
    fn build(new_depth: i32) -> (Map, Position);
    fn spawn(map: &Map, ecs: &mut World, new_depth: i32);
}

pub fn spawn(map: &Map, ecs: &mut World, new_depth: i32) {
    SimpleMapBuilder::spawn(map, ecs, new_depth);
}

pub fn build_random_map(new_depth: i32) -> (Map, Position) {
    SimpleMapBuilder::build(new_depth)
}
