extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::event::*;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Point};
use std::time::Duration;

struct MainState {
    pos_x: f32,
    pos_y: f32,
    image: graphics::Image,
    image2: graphics::Image,
    text: graphics::Text
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, "/b1.png").unwrap();
        let image2 = graphics::Image::new(ctx, "/b2.png").unwrap();
        let font = graphics::Font::new(ctx, "/leaguespartan-bold.ttf", 28)?;
        let text = graphics::Text::new(ctx, "Beyonce Brawls", &font)?;

        let s = MainState {
            pos_x: 400.0,
            pos_y: 0.0,
            image: image,
            image2: image2,
            text: text
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        self.pos_y = self.pos_y % 400.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let dest_point = graphics::Point::new(self.pos_x, self.pos_y);
        //graphics::circle(ctx, DrawMode::Fill, Point { x: self.pos_x, y: 380.0 }, 100.0, 32)?;
        graphics::draw(ctx, &self.image, dest_point, 0.0)?;

        graphics::draw(ctx, &self.text, Point { x: self.text.width() as f32, y: self.text.height() as f32 }, 0.0)?;
        graphics::present(ctx);
        Ok(())
    }
    
    fn key_down_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode,
            keymod,
            repeat
        );
    }
    fn key_up_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key released: {:?}, modifier {:?}, repeat: {}",
            keycode,
            keymod,
            repeat
        );
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
