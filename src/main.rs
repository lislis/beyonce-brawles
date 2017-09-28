extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::event::*;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Point};
use std::time::Duration;


const WINDOW_W: u32 = 400;
const WINDOW_H: u32 = 700;

const PLAYER_X: f32 = 195.0;
const PLAYER_Y: f32 = 20.0;
const PLAYER_WALKING_SPEED: f32 = 2.0;
const HITAREA_W: f32 = 128.0;
const HITAREA_H: f32 = 32.0;
const PLAYER_HOLDING_SPEED: f32 = 0.3;
const PLAYER_HOLDING_TIME_MIN: f32 = 4.0;
const PLAYER_HOLDING_TIME_MAX: f32 = 6.0;


struct Player {
    x: f32,
    y: f32,
    sprite: graphics::Image,
    hitarea: graphics::Image,
    h_x: f32,
    h_y: f32,
    h_w: f32,
    h_h: f32,
    holding: f32
}

impl Player {
    fn new(ctx: &mut Context) -> Player {
        Player {
            x: PLAYER_X,
            y: PLAYER_Y,
            sprite: graphics::Image::new(ctx, "/beyonce.png").unwrap(),
            hitarea: graphics::Image::new(ctx, "/hitarea.png").unwrap(),
            h_x: PLAYER_X,
            h_y: PLAYER_X + HITAREA_H,
            h_w: HITAREA_W,
            h_h: HITAREA_H,
            holding: 0.0
        }
    }

    pub fn update(&mut self) {
        if self.holding == 0.0 {
            self.y = self.y % WINDOW_H as f32 + PLAYER_WALKING_SPEED;
            self.h_y = self.y + HITAREA_H;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dest_point = graphics::Point::new(self.x, self.y);
        graphics::draw(ctx, &self.sprite, dest_point, 0.0)?;
        if self.holding > PLAYER_HOLDING_TIME_MIN {
            let dest_hitarea = graphics::Point::new(self.h_x, self.h_y);
            graphics::draw(ctx, &self.hitarea, dest_hitarea, 0.0)?;
        }
        Ok(())
    }

    pub fn hold(&mut self) {
        if self.holding > 0.0 {
            self.holding += PLAYER_HOLDING_SPEED;

            if self.holding > PLAYER_HOLDING_TIME_MAX {
                self.unhold();
            }
        } else {
            self.holding = 0.1;
        }
    }

    pub fn unhold(&mut self) {
        self.holding = 0.0;
    }
}

struct MainState {
    player: Player
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            player: Player::new(ctx)
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        self.player.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.player.draw(ctx)?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _: Mod, _: bool) {
        match keycode {
            Keycode::Space => {
                self.player.hold();
            }
            _ => {}
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, _: Mod, _: bool) {
        match keycode {
            Keycode::Space => {
                self.player.unhold();
            }
            _ => {}
        }
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_title = "Beyoncé Brawles".to_string();
    c.window_width = WINDOW_W;
    c.window_height = WINDOW_H;

    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
