extern crate rltk;
extern crate specs;

use std::cmp::{max, min};

use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs::World;

use specs_derive::Component;

const WIDTH: usize = 80;
const HEIGHT: usize = 50;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct LeftMover {}

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

struct State {
    ecs: World,
}

struct LeftMoverImplementation {}

impl<'a> System<'a> for LeftMoverImplementation {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = (WIDTH - 1) as i32;
            }
        }
    }
}

pub fn xy_idx(x: usize, y: usize) -> usize {
    (y as usize * WIDTH) + x as usize
}

fn new_game_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; HEIGHT * WIDTH];
    for x in 0..WIDTH {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, HEIGHT - 1)] = TileType::Wall;
    }
    for y in 0..HEIGHT {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(WIDTH - 1, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();
    for _i in 0..400 {
        let x = rng.roll_dice(1, 79) as usize;
        let y = rng.roll_dice(1, 49) as usize;
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

fn draw_game_map(map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_u8(87, 134, 112),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();
    // for i in 0..10 {
    //     gs.ecs
    //         .create_entity()
    //         .with(Position { x: i * 7, y: 20 })
    //         .with(Renderable {
    //             glyph: rltk::to_cp437('@'),
    //             fg: RGB::named(rltk::YELLOW),
    //             bg: RGB::named(rltk::BLACK),
    //         })
    //         .with(LeftMover {})
    //         .build();
    // }

    gs.ecs.insert(new_game_map());

    rltk::main_loop(context, gs)
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // CLear Screen
        ctx.cls();

        player_input(self, ctx);

        self.run_systems();
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_game_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftMoverImplementation {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    //player movement
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();
    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx((pos.x + delta_x) as usize, (pos.y + delta_y) as usize);
        if map[destination_idx] != TileType::Wall {
            pos.x = min((WIDTH - 1) as i32, max(0, pos.x + delta_x));
            pos.y = min((HEIGHT - 1) as i32, max(0, pos.y + delta_y));
        }
    }
}
