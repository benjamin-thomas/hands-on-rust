#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;

impl Utils {
    fn pluralize(count: i32, singular: &str, plural: &str) -> String {
        if count == 1 {
            format!("{} {}", count, singular)
        } else {
            format!("{} {}", count, plural)
        }
    }
}
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const FRAME_DURATION: f32 = 30.0;
const DRAGON_FRAMES : [u16; 6] = [ 64, 1, 2, 3, 2, 1 ];

enum GameMode {
    Menu,
    Playing,
    GameOver,
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn gap_top(&self) -> i32 {
        self.gap_y - self.size / 2
    }

    fn gap_bottom(&self) -> i32 {
        self.gap_y + self.size / 2
    }

    fn is_hit(&self, player: &Player) -> bool {
        let does_x_match = player.x == self.x;
        let player_above_gap = (player.y as i32) < self.gap_top();
        let player_below_gap = (player.y as i32) > self.gap_bottom();
        does_x_match && (player_above_gap || player_below_gap)
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        ctx.set_active_console(1);
        let screen_x = self.x - player_x;

        let mut set_point = |y: i32| ctx.set_fancy(
            PointF::new(screen_x as f32, y as f32),
            1,
            Degrees::new(0.0),
            PointF::new(1.0, 1.0),
            RED,
            WHITE,
            4,
        );

        // Draw the top half of the obstacle
        for y in 0..self.gap_top() {
            // ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
            set_point(y);
        }

        // Draw the bottom half of the obstacle
        for y in self.gap_bottom()..SCREEN_HEIGHT {
            // ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
           set_point(y);
        }
        ctx.set_active_console(0);
    }
}

struct Player {
    x: i32,
    y: f32,
    velocity: f32,
    frame: usize,
}

impl Player {
    fn new(x: i32, y: f32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
            frame: 0,
        }
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2
        };

        self.y += self.velocity;

        if self.y < 0.0 {
            self.y = 0.0
        }

        self.x += 1;
        self.frame += 1;
        self.frame = self.frame % 6;
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        // ctx.set(0, self.y as i32, YELLOW, BLACK, to_cp437('@'));
        ctx.set_fancy(
            PointF::new(0.0, self.y),
            1,
            Degrees::new(0.0),
            PointF::new(2.0, 2.0),
            WHITE,
            NAVY,
            DRAGON_FRAMES[self.frame],
        );

        ctx.set_active_console(0);
    }
}

struct State {
    player: Player,
    obstacle: Obstacle,
    frame_time: f32,
    mode: GameMode,
    score: i32,
}

struct Utils(i32, &'static str, &'static str);

impl State {
    fn new() -> Self {
        State::init(GameMode::Menu)
    }

    fn restart(&mut self) {
        *self = State::init(GameMode::Playing);
    }

    fn init(mode: GameMode) -> Self {
        Self {
            player: Player::new(5, 25.0),
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            frame_time: 0.0,
            score: 0,
            mode,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play game");
        ctx.print_centered(9, "(Q) Quit game");

        self.play_or_quit(ctx);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(
            6,
            &format!(
                "You earned {}",
                Utils::pluralize(self.score, "point", "points")
            ),
        );
        ctx.print_centered(8, "(P) Play game");
        ctx.print_centered(9, "(Q) Quit game");

        self.play_or_quit(ctx);
    }

    fn play_or_quit(&mut self, ctx: &mut BTerm) {
        match ctx.key {
            Some(VirtualKeyCode::P) => self.restart(),
            Some(VirtualKeyCode::Q) => ctx.quitting = true,
            _ => (),
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);

        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score)
        }

        if self.player.y as i32 > SCREEN_HEIGHT || self.obstacle.is_hit(&self.player) {
            self.mode = GameMode::GameOver;
        }
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
    let font_path = "/home/benjamin/code/github.com/benjamin-thomas/hands-on-rust/flappy/assets/img/flappy32.png";
    let window = BTermBuilder::new()
        /*
           $ file assets/img/flappy32.png
           assets/img/flappy32.png: PNG image data, 512 x 512, 8-bit/color RGB, non-interlaced
           (32 px) * (16 items) = 512 (512 x 512 image)
        */
        .with_font(font_path, 32, 32)
        .with_tile_dimensions(16, 16)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, font_path)
        .with_fancy_console(  SCREEN_WIDTH, SCREEN_HEIGHT, font_path)
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(window, State::new())
}
