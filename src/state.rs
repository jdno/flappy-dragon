use bracket_lib::prelude::*;

use crate::obstacle::Obstacle;
use crate::player::Player;
use crate::{FRAME_DURATION, SCREEN_HEIGHT, SCREEN_WIDTH};

enum GameMode {
    Menu,
    Game,
    Score,
}

pub struct State {
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    player: Player,
    score: i32,
}

impl State {
    fn menu_mode(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        self.process_input(ctx);
    }

    fn game_mode(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);

        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.fly();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, format!("Score: {}", self.score));

        self.obstacle.render(ctx, &self.player);

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.obstacle.has_hit_player(&self.player) || self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::Score;
        }
    }

    fn score_mode(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You crashed!");
        ctx.print_centered(6, format!("You earned {} points", self.score));

        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        self.process_input(ctx);
    }

    fn restart_game(&mut self) {
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.player = Player::new(5, 25);
        self.mode = GameMode::Game;
        self.score = 0;
    }

    fn process_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart_game(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            player: Player::new(5, 25),
            mode: GameMode::Menu,
            score: 0,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.menu_mode(ctx),
            GameMode::Game => self.game_mode(ctx),
            GameMode::Score => self.score_mode(ctx),
        }
    }
}
