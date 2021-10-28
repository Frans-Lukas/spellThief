extern crate rltk;
extern crate specs;

use rltk::{GameState, RGB, Rltk};
use specs::prelude::*;

pub use components::*;
pub use map::*;
use player::*;
pub use rect::Rect;
use specs_derive::Component;

mod components;
mod map;
mod player;
mod rect;

const WIDTH: usize = 80;
const HEIGHT: usize = 50;
const WORLD_SIZE: Position = Position {
    x: WIDTH as i32,
    y: HEIGHT as i32,
};

#[derive(Component)]
struct LeftMover {}

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

    let map =Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    gs.ecs
      .create_entity()
      .with(Position { x: player_x, y: player_y })
      .with(Renderable {
          glyph: rltk::to_cp437('@'),
          fg: RGB::named(rltk::YELLOW),
          bg: RGB::named(rltk::BLACK),
      })
      .with(Player {})
      .build();
    rltk::main_loop(context, gs)
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // CLear Screen
        ctx.cls();

        player_input(self, ctx, WORLD_SIZE);

        self.run_systems();
        draw_game_map(&self.ecs, ctx);

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
