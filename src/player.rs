use std::cmp::{max, min};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

use super::{Player, Position, State, TileType, xy_idx};

pub(crate) fn player_input(gs: &mut State, ctx: &mut Rltk, world_size: Position) {
    //player movement
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs, world_size),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs, world_size),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs, world_size),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs, world_size),
            _ => {}
        },
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World, world_size: Position) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();
    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx((pos.x + delta_x) as usize, (pos.y + delta_y) as usize);
        if map[destination_idx] != TileType::Wall {
            pos.x = min((world_size.x - 1) as i32, max(0, pos.x + delta_x));
            pos.y = min((world_size.y - 1) as i32, max(0, pos.y + delta_y));
        }
    }
}
