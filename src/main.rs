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
    hitarea: graphics::Image,
    text: graphics::Text,
    holding: f32,
    t_x: f32,
    t_y: f32,
    t_image: graphics::Image
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, "/b1.png").unwrap();
        let image2 = graphics::Image::new(ctx, "/b2.png").unwrap();
        let hitarea = graphics::Image::new(ctx, "/dangerzone.png").unwrap();
        let font = graphics::Font::new(ctx, "/leaguespartan-bold.ttf", 28)?;
        let text = graphics::Text::new(ctx, "Beyonce Brawls", &font)?;

        let thing_image = graphics::Image::new(ctx, "/thing.png").unwrap();

        let s = MainState {
            pos_x: 400.0,
            pos_y: 0.0,
            image: image,
            image2: image2,
            hitarea: hitarea,
            text: text,
            holding: 0.0,
            t_x: 330.0,
            t_y: 300.0,
            t_image: thing_image
        };
        Ok(s)
    }
    pub fn hold(&mut self) {
        if self.holding > 0.0 {
            // count time
            self.holding += 0.1;
            println!("Holding {}", self.holding);

            if self.holding > 8.0 { // magic number
                println!("Holding too long, resetting");
                // some sort of punishing sleep?
                // otherwise pressing event fires right away again
                self.unhold();
            }
        } else {
            self.holding = 0.1;
        }
    }
    pub fn unhold(&mut self) -> bool {
        self.holding = 0.0;
        true
    }
    pub fn collision(&mut self) {
        if self.holding > 4.0 { //magic number
            if self.pos_x < self.t_x + 128.0 &&
                self.pos_x + 64.0 > self.t_x &&
                self.pos_y < self.t_y + 32.0 &&
                self.pos_y + 64.0 > self.t_y {
                    println!("HIT EM");
                }
        }

    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        if self.holding == 0.0 {
            self.pos_y = self.pos_y % 400.0 + 1.0;
        }
        self.collision();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let dest_point = graphics::Point::new(self.pos_x, self.pos_y);
        graphics::draw(ctx, &self.text, Point { x: self.text.width() as f32, y: self.text.height() as f32 }, 0.0)?;

        graphics::draw(ctx, &self.t_image, Point { x: self.t_x, y: self.t_y }, 0.0)?;

        if self.holding > 4.0 { // magic number
            let dest_hitarea = graphics::Point::new(self.pos_x, self.pos_y + 32.0);
            graphics::draw(ctx, &self.text, Point { x: 200.0,  y: 500.0}, 0.0)?;
            graphics::draw(ctx, &self.image2, dest_point, 0.0)?;
            graphics::draw(ctx, &self.hitarea, dest_hitarea, 0.0)?;
        } else if self.holding > 0.0 {
            graphics::draw(ctx, &self.image2, dest_point, 0.0)?;
        } else {
            graphics::draw(ctx, &self.image, dest_point, 0.0)?;
        }

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
        match keycode {
            Keycode::Space => {
                self.hold();
            }
            _ => {}
        }
    }
    fn key_up_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key released: {:?}, modifier {:?}, repeat: {}",
            keycode,
            keymod,
            repeat
        );

        match keycode {
            Keycode::Space => {
                self.unhold();
            }
            _ => {}
        }
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
