extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Point};
use std::time::Duration;


const WINDOW_W: u32 = 400;
const WINDOW_H: u32 = 700;



struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx, DrawMode::Fill, Point { x: self.pos_x, y: 380.0 }, 100.0, 32)?;
        graphics::present(ctx);
        Ok(())
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
