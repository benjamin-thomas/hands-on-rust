#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    GameOver,
}
struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play game");
        ctx.print_centered(9, "(Q) Quit game");

        match ctx.key {
            Some(VirtualKeyCode::P) => self.restart(),
            Some(VirtualKeyCode::Q) => ctx.quitting = true,
            _ => (),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play game");
        ctx.print_centered(9, "(Q) Quit game");

        match ctx.key {
            Some(VirtualKeyCode::P) => self.restart(),
            Some(VirtualKeyCode::Q) => ctx.quitting = true,
            _ => (),
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::GameOver;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::GameOver => self.game_over(ctx),
        }
    }
}

fn main() -> BError {
    let bterm = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(bterm, State::new())
}
