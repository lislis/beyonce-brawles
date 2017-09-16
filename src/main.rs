extern crate ggez;
extern crate rand;

use rand::Rng;

use ggez::conf;
use ggez::event;
use ggez::event::*;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Point};
use std::time::Duration;

enum SmashableType {
    Car { w: f32, h: f32, s: graphics::Image },
    Hydrant { w: f32, h: f32,  s: graphics::Image },
    CCTV { w: f32, h: f32, s: graphics::Image  }
}

struct Smashable {
    x: f32,
    y: f32,
    t: i32
}

impl Smashable {
    fn new(ctx: &mut Context) -> Smashable {
        let mut rng = rand::thread_rng();
        let y = rng.gen::<f32>() * 100.0 + rng.gen::<f32>() * 500.0;
        let x:f32;
        let ltr = rng.gen();
        match ltr {
            true => { x = 320.0 } // magic number
            false => { x = 470.0 } //magic number
        }
        let t:SmashableType;
        let t = rand::thread_rng().gen_range(1, 4);

        Smashable {
            x: x,
            y: y,
            t: t
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let point = graphics::Point::new(self.x, self.y);
        let car = graphics::Image::new(ctx, "/car.png").unwrap();
        let cctv = graphics::Image::new(ctx, "/cctv.png").unwrap();
        let hydrant = graphics::Image::new(ctx, "/hydrant.png").unwrap();
        match self.t {
            2 => {
                graphics::draw(ctx, &car, point, 0.0);
            }
            3 => {
                graphics::draw(ctx, &cctv, point, 0.0);
            }
            _ => {
                graphics::draw(ctx, &hydrant, point, 0.0);
            }
        }
    }
}

struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    sprite1: graphics::Image,
    sprite2: graphics::Image,
    hitarea: graphics::Image,
    holding: f32
}

impl Player {
    fn new(ctx: &mut Context) -> Player {
        Player {
            x: 400.0,
            y: 0.0,
            w: 64.0,
            h: 64.0,
            sprite1: graphics::Image::new(ctx, "/b1.png").unwrap(),
            sprite2: graphics::Image::new(ctx, "/b2.png").unwrap(),
            hitarea: graphics::Image::new(ctx, "/dangerzone.png").unwrap(),
            holding: 0.0
        }
    }

    pub fn update(&mut self) {
        if self.holding == 0.0 {
            self.y = self.y % 400.0 + 1.0;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let dest_point = graphics::Point::new(self.x, self.y);

        if self.holding > 4.0 { // magic number
            let dest_hitarea = graphics::Point::new(self.x, self.y + 32.0);
            graphics::draw(ctx, &self.sprite2, dest_point, 0.0);
            graphics::draw(ctx, &self.hitarea, dest_hitarea, 0.0);
        } else if self.holding > 0.0 {
            graphics::draw(ctx, &self.sprite2, dest_point, 0.0);
        } else {
            graphics::draw(ctx, &self.sprite1, dest_point, 0.0);
        }
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
}


struct MainState {
    player: Player,
    text: graphics::Text,
    smashables: Vec<Smashable>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/leaguespartan-bold.ttf", 28)?;
        let text = graphics::Text::new(ctx, "Beyonce Brawls", &font)?;
        let thing_image = graphics::Image::new(ctx, "/thing.png").unwrap();

        let mut smashables = vec![];

        for x in 0..20 {
            smashables.push(Smashable::new(ctx));
        }

        let s = MainState {
            player: Player::new(ctx),
            text: text,
            smashables: smashables
        };
        Ok(s)
    }

    pub fn collision(&mut self) {
        if self.player.holding > 4.0 { //magic number
            /*if self.player.x < self.t_x + 128.0 &&
                self.player.x + self.player.w > self.t_x &&
                self.player.y < self.t_y + 32.0 &&
                self.player.y + self.player.h > self.t_y {
                    println!("HIT EM");
                }*/
        }

    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        self.player.update();
        self.collision();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::draw(ctx, &self.text, Point { x: self.text.width() as f32, y: self.text.height() as f32 }, 0.0)?;

        for s in self.smashables.iter_mut() {
            s.draw(ctx);
        }

        self.player.draw(ctx);

        if self.player.holding > 4.0 {
            graphics::draw(ctx, &self.text, Point { x: 200.0,  y: 500.0}, 0.0)?;
        }
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
/*        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode,
            keymod,
            repeat
        );
*/
        match keycode {
            Keycode::Space => {
                self.player.hold();
            }
            _ => {}
        }
    }
    fn key_up_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
/*        println!(
            "Key released: {:?}, modifier {:?}, repeat: {}",
            keycode,
            keymod,
            repeat
        );
*/
        match keycode {
            Keycode::Space => {
                self.player.unhold();
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
