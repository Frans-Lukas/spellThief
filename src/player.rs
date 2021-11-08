use std::cmp::{max, min};

use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

use super::{
    CombatStats, GameLog, Item, KnownSpells, Map, Player, Position, RunState, State, TileType,
    Viewshed, WantsToMelee, WantsToPickupItem,
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

            VirtualKeyCode::Key1 => {
                if has_spell_in_slot(&mut gs.ecs, 1) {
                    let (range, spell) = get_spell_in_slot(&mut gs.ecs, 1);
                    return RunState::ShowTargeting {
                        range,
                        targetable: spell,
                    };
                }
            }

            VirtualKeyCode::G => get_item(&mut gs.ecs),
            VirtualKeyCode::Numpad5 => return RunState::PlayerTurn,
            VirtualKeyCode::Space => return RunState::PlayerTurn,
            VirtualKeyCode::I => return RunState::ShowInventory,
            VirtualKeyCode::D => return RunState::ShowDropItem,
            // Save and Quit
            VirtualKeyCode::Escape => return RunState::SaveGame,
            VirtualKeyCode::R => return RunState::ShowRemoveItem,
            // Level changes
            VirtualKeyCode::Period => {
                if try_next_level(&mut gs.ecs) {
                    return RunState::NextLevel;
                }
            }
            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}

fn has_spell_in_slot(ecs: &mut World, slot: i32) -> bool {
    let players = ecs.read_storage::<Player>();
    let known_spells = ecs.read_storage::<KnownSpells>();
    for (_player, known_spells) in (&players, &known_spells).join() {
        return known_spells.spells.len() >= slot as usize;
    }
    return false;
}

fn get_spell_in_slot(ecs: &mut World, slot: i32) -> (i32, Entity) {
    unimplemented!()
}

pub fn try_next_level(ecs: &mut World) -> bool {
    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let player_idx = map.xy_idxi32(player_pos.x, player_pos.y);
    if map.tiles[player_idx] == TileType::DownStairs {
        true
    } else {
        let mut gamelog = ecs.fetch_mut::<GameLog>();
        gamelog
            .entries
            .push("There is no way down from here.".to_string());
        false
    }
}

fn get_item(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();

    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        None => gamelog
            .entries
            .push("There is nothing here to pick up.".to_string()),
        Some(item) => {
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            pickup
                .insert(
                    *player_entity,
                    WantsToPickupItem {
                        collected_by: *player_entity,
                        item,
                    },
                )
                .expect("Unable to insert want to pickup");
        }
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World, world_size: Position) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();

    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();

    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _player, pos, viewshed) in
        (&entities, &mut players, &mut positions, &mut viewsheds).join()
    {
        let destination_idx = map.xy_idx((pos.x + delta_x) as usize, (pos.y + delta_y) as usize);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Add target failed");
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
