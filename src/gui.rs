use rltk::{RGB, Rltk};
use specs::{Join, World, WorldExt};

use super::{CombatStats, MagicStats, Player};
use super::{B_GUI_SIZE, HEIGHT, Name, R_GUI_SIZE, WIDTH, WINDOW_WIDTH};

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    draw_bottom_ui(ecs, ctx);
    draw_right_ui(ecs, ctx);
}

fn draw_right_ui(ecs: &World, ctx: &mut Rltk) {
    draw_right_border(ctx);
    draw_player_stats(ecs, ctx);
}

fn draw_bottom_ui(ecs: &World, ctx: &mut Rltk) {
    draw_bottom_border(ctx);
    draw_healthbar(ecs, ctx);
}

fn draw_bottom_border(ctx: &mut Rltk) {
    ctx.draw_box(
        0,
        HEIGHT,
        WINDOW_WIDTH - 1,
        B_GUI_SIZE - 1,
        RGB::named(rltk::WHEAT),
        RGB::named((rltk::BLACK)),
    )
}

fn draw_right_border(ctx: &mut Rltk) {
    ctx.draw_box(
        WIDTH,
        0,
        R_GUI_SIZE - 1,
        HEIGHT - 1,
        RGB::named(rltk::WHEAT),
        RGB::named((rltk::BLACK)),
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

        ctx.print_color(53, 43, color, RGB::named(rltk::BLACK), &health);
        ctx.draw_bar_horizontal(
            1,
            HEIGHT,
            51,
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
        ctx.print_color(
            WIDTH + x_start,
            start_value + y_spacing * 2,
            RGB::named(rltk::BLUE_VIOLET),
            RGB::named(rltk::BLACK),
            &mana,
        );
    }
}
