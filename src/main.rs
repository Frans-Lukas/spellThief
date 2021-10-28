extern crate rltk;

use rltk::{GameState, Rltk};

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // CLear Screen
        ctx.cls();
        ctx.print(1, 1, "Hello, Rust world!");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("Roguelike Tutorial").build()?;
    let gs = State {};
    rltk::main_loop(context, gs)
}
