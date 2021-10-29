use std::cmp::{max, min};

use rltk::{console, Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

use super::{
    CombatStats, Map, Player, Position, RunState, State, TileType, Viewshed, WantsToMelee,
};

pub(crate) fn player_input(gs: &mut State, ctx: &mut Rltk, world_size: Position) -> RunState {
    //player movement
    match ctx.key {
        None => return RunState::AwaitingInput,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(-1, 0, &mut gs.ecs, world_size)
            }

            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(1, 0, &mut gs.ecs, world_size)
            }

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(0, -1, &mut gs.ecs, world_size)
            }

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(0, 1, &mut gs.ecs, world_size)
            }

            // Diagonals
            VirtualKeyCode::Numpad9 | VirtualKeyCode::Y => {
                try_move_player(1, -1, &mut gs.ecs, world_size)
            }

            VirtualKeyCode::Numpad7 | VirtualKeyCode::U => {
                try_move_player(-1, -1, &mut gs.ecs, world_size)
            }

            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => {
                try_move_player(1, 1, &mut gs.ecs, world_size)
            }

            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => {
                try_move_player(-1, 1, &mut gs.ecs, world_size)
            }

            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World, world_size: Position) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();

    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();

    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _player, pos, viewshed) in (&entities, &mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx((pos.x + delta_x) as usize, (pos.y + delta_y) as usize);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee.insert(entity, WantsToMelee{ target: *potential_target }).expect("Add target failed");
                return;
            }
        }
        if !map.blocked[destination_idx] {
            pos.x = min(world_size.x - 1, max(0, pos.x + delta_x));
            pos.y = min(world_size.y - 1, max(0, pos.y + delta_y));

            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}
