use std::cmp::max;

use rltk::{Rltk, RGB};
use specs::{Join, World, WorldExt};

use super::{CombatStats, MagicStats, Player};
use super::{GameLog, Name, B_GUI_SIZE, HEIGHT, R_GUI_SIZE, WIDTH, WINDOW_WIDTH};

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    draw_bottom_ui(ecs, ctx);
    draw_right_ui(ecs, ctx);

    draw_cursor(ctx);
}

fn draw_cursor(ctx: &mut Rltk) {
    // Draw mouse cursor
    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));
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
            RGB::named(rltk::BROWN1),
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
            RGB::named(rltk::BLUE_VIOLET),
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
    }
}
