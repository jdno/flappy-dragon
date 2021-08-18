use bracket_lib::prelude::*;

use crate::player::Player;
use crate::SCREEN_HEIGHT;

pub struct Obstacle {
    pub x: i32,
    gap: i32,
    size: i32,
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();

        Self {
            x,
            gap: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, player: &Player) {
        let screen_x = self.x - player.x;
        let half_size = self.size / 2;

        for y in 0..(self.gap - half_size) {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
        }

        for y in (self.gap + half_size)..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
        }
    }

    pub fn has_hit_player(&self, player: &Player) -> bool {
        let half_size = self.size / 2;

        let does_x_match = self.x == player.x;
        let player_above_gap = player.y < self.gap - half_size;
        let player_below_gap = player.y > self.gap + half_size;

        does_x_match && (player_above_gap || player_below_gap)
    }
}
