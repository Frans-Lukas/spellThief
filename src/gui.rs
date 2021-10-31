use std::cmp::max;

use rltk::{Point, RGB, Rltk, VirtualKeyCode};
use specs::{Entity, Join, World, WorldExt};

use super::{CombatStats, InBackpack, MagicStats, Map, Player, Position, State, Viewshed};
use super::{B_GUI_SIZE, GameLog, HEIGHT, Name, R_GUI_SIZE, WIDTH, WINDOW_WIDTH};

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    draw_bottom_ui(ecs, ctx);
    draw_right_ui(ecs, ctx);
    draw_cursor(ecs, ctx);
}

fn draw_cursor(ecs: &World, ctx: &mut Rltk) {
    // Draw mouse cursor
    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));
    draw_tooltips(ecs, ctx);
}

fn draw_right_ui(ecs: &World, ctx: &mut Rltk) {
    draw_right_border(ctx);
    draw_player_stats(ecs, ctx);
}

fn draw_bottom_ui(ecs: &World, ctx: &mut Rltk) {
    draw_bottom_border(ctx);
    // draw_healthbar(ecs, ctx);
    draw_message_log(ecs, ctx);
}

fn draw_message_log(ecs: &World, ctx: &mut Rltk) {
    let log = ecs.fetch::<GameLog>();
    let mut y = 44;
    let l = max((log.entries.len() as i32) - 5, 0) as usize;
    // console::log(format!("abcd: {}", l));
    for s in log.entries[l..log.entries.len()].iter() {
        if y < 49 {
            ctx.print(2, y, s);
        }
        y += 1;
    }
}

fn draw_bottom_border(ctx: &mut Rltk) {
    ctx.draw_box(
        0,
        HEIGHT,
        WINDOW_WIDTH - 1,
        B_GUI_SIZE - 1,
        RGB::named(rltk::WHEAT),
        RGB::named(rltk::BLACK),
    )
}

fn draw_right_border(ctx: &mut Rltk) {
    ctx.draw_box(
        WIDTH,
        0,
        R_GUI_SIZE - 1,
        HEIGHT - 1,
        RGB::named(rltk::WHEAT),
        RGB::named(rltk::BLACK),
    )
}

fn draw_healthbar(ecs: &World, ctx: &mut Rltk) {
    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();

    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!("{:02}/{:02}", stats.hp, stats.max_hp);

        let mut color = RGB::named(rltk::GREEN1);
        if stats.hp < stats.max_hp / 3 {
            color = RGB::named(rltk::RED1);
        } else if stats.hp < stats.max_hp - stats.max_hp / 3 {
            color = RGB::named(rltk::YELLOW1);
        }

        ctx.print_color(
            WIDTH / 2 + WIDTH / 3 + 2,
            43,
            color,
            RGB::named(rltk::BLACK),
            &health,
        );
        ctx.draw_bar_horizontal(
            WIDTH,
            HEIGHT,
            WIDTH / 5,
            stats.hp,
            stats.max_hp,
            color,
            RGB::named(rltk::BLACK),
        );
    }
}

fn draw_player_stats(ecs: &World, ctx: &mut Rltk) {
    let combat_stats = ecs.read_storage::<CombatStats>();
    let magic_stats = ecs.read_storage::<MagicStats>();
    let players = ecs.read_storage::<Player>();
    let names = ecs.read_storage::<Name>();
    let map = ecs.fetch::<Map>();

    let start_value = 1;
    let y_spacing = 2;
    let x_start = 1;
    for (_player, stats, mstats, name) in (&players, &combat_stats, &magic_stats, &names).join() {
        let health = format!("hp:{:02}/{:02}", stats.hp, stats.max_hp);
        let mana = format!("mp:{:02}/{:02}", mstats.mana, mstats.max_mana);
        ctx.print_color(
            WIDTH + x_start,
            start_value,
            RGB::named(rltk::WHEAT),
            RGB::named(rltk::BLACK),
            &name.name,
        );
        ctx.print_color(
            WIDTH + x_start,
            start_value + y_spacing,
            RGB::named(rltk::WHEAT),
            RGB::named(rltk::BLACK),
            &health,
        );
        ctx.draw_bar_horizontal(
            WIDTH + x_start,
            start_value + y_spacing + 1,
            12,
            stats.hp,
            stats.max_hp,
            RGB::named(rltk::BROWN1),
            RGB::named(rltk::BLACK),
        );
        ctx.print_color(
            WIDTH + x_start,
            start_value + y_spacing * 2 + 1,
            RGB::named(rltk::WHEAT),
            RGB::named(rltk::BLACK),
            &mana,
        );

        ctx.draw_bar_horizontal(
            WIDTH + x_start,
            start_value + y_spacing * 3,
            12,
            mstats.mana,
            mstats.max_mana,
            RGB::named(rltk::BLUE_VIOLET),
            RGB::named(rltk::BLACK),
        );
        ctx.print_color(
            WIDTH + x_start,
            start_value + y_spacing * 4,
            RGB::named(rltk::WHEAT),
            RGB::named(rltk::BLACK),
            format!("depth:{}", map.depth),
        );
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

pub fn show_inventory(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    ctx.print_color(
        18,
        y - 2,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Inventory",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "ESCAPE to cancel",
    );

    let mut equippable: Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, name) in (&entities, &backpack, &names)
        .join()
        .filter(|item| item.1.owner == *player_entity)
    {
        ctx.set(
            17,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('('),
        );
        ctx.set(
            18,
            y,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            97 + j as rltk::FontCharType,
        );
        ctx.set(
            19,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437(')'),
        );

        ctx.print(21, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
        j += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = rltk::letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        ItemMenuResult::Selected,
                        Some(equippable[selection as usize]),
                    );
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}

pub fn drop_item_menu(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    ctx.print_color(
        18,
        y - 2,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Drop Which Item?",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "ESCAPE to cancel",
    );

    let mut equippable: Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, name) in (&entities, &backpack, &names)
        .join()
        .filter(|item| item.1.owner == *player_entity)
    {
        ctx.set(
            17,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('('),
        );
        ctx.set(
            18,
            y,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            97 + j as rltk::FontCharType,
        );
        ctx.set(
            19,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437(')'),
        );

        ctx.print(21, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
        j += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = rltk::letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        ItemMenuResult::Selected,
                        Some(equippable[selection as usize]),
                    );
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}

fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height {
        return;
    }
    let mut tooltip: Vec<String> = Vec::new();
    for (name, position) in (&names, &positions).join() {
        let idx = map.xy_idxi32(position.x, position.y);
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
            tooltip.push(name.name.to_string());
        }
    }

    if !tooltip.is_empty() {
        let mut width: i32 = 0;
        for s in tooltip.iter() {
            if width < s.len() as i32 {
                width = s.len() as i32;
            }
        }
        width += 3;

        if mouse_pos.0 > 40 {
            let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
            let left_x = mouse_pos.0 - width;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(
                    left_x,
                    y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::GREY),
                    s,
                );
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x - i,
                        y,
                        RGB::named(rltk::WHITE),
                        RGB::named(rltk::GREY),
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::GREY),
                &"->".to_string(),
            );
        } else {
            let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 + 3;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(
                    left_x + 1,
                    y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::GREY),
                    s,
                );
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x + 1 + i,
                        y,
                        RGB::named(rltk::WHITE),
                        RGB::named(rltk::GREY),
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::GREY),
                &"<-".to_string(),
            );
        }
    }
}

pub fn ranged_target(
    gs: &mut State,
    ctx: &mut Rltk,
    range: i32,
) -> (ItemMenuResult, Option<Point>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let player_pos = gs.ecs.fetch::<Point>();
    let viewsheds = gs.ecs.read_storage::<Viewshed>();

    ctx.print_color(
        5,
        0,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Select Target:",
    );

    // Highlight available target cells
    let mut available_cells = Vec::new();
    let visible = viewsheds.get(*player_entity);
    if let Some(visible) = visible {
        // We have a viewshed
        for idx in visible.visible_tiles.iter() {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, *idx);
            if distance <= range as f32 {
                ctx.set_bg(idx.x, idx.y, RGB::named(rltk::BLUE));
                available_cells.push(idx);
            }
        }
    } else {
        return (ItemMenuResult::Cancel, None);
    }

    // Draw mouse cursor
    let mouse_pos = ctx.mouse_pos();
    let mut valid_target = false;
    for idx in available_cells.iter() {
        if idx.x == mouse_pos.0 && idx.y == mouse_pos.1 {
            valid_target = true;
        }
    }
    if valid_target {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::CYAN));
        if ctx.left_click {
            return (
                ItemMenuResult::Selected,
                Some(Point::new(mouse_pos.0, mouse_pos.1)),
            );
        }
    } else {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::RED));
        if ctx.left_click {
            return (ItemMenuResult::Cancel, None);
        }
    }

    (ItemMenuResult::NoResponse, None)
}
