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
    Car { w: f32, h: f32 },
    Hydrant { w: f32, h: f32 },
    CCTV { w: f32, h: f32 }
}

struct Smashable {
    x: f32,
    y: f32,
    t: i32,
    active: bool,
    car: graphics::Image,
    cctv: graphics::Image,
    hydrant: graphics::Image
}

impl Smashable {
    fn new(ctx: &mut Context) -> Smashable {
        let mut rng = rand::thread_rng();
        let y = rng.gen::<f32>() * 600.0 + 150.0;
        let x:f32;
        let ltr = rng.gen();
        match ltr {
            true => { x = 130.0 } // magic number
            false => { x = 260.0 } //magic number
        }
        let t = rand::thread_rng().gen_range(1, 4);

        let car = graphics::Image::new(ctx, "/car.png").unwrap();
        let cctv = graphics::Image::new(ctx, "/cctv.png").unwrap();
        let hydrant = graphics::Image::new(ctx, "/hydrant.png").unwrap();

        Smashable {
            x: x,
            y: y,
            t: t,
            active: true,
            car: car,
            cctv: cctv,
            hydrant: hydrant
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let point = graphics::Point::new(self.x, self.y);

        if self.active {
            match self.t {
                2 => {
                    graphics::draw(ctx, &self.car, point, 0.0);
                }
                3 => {
                    graphics::draw(ctx, &self.cctv, point, 0.0);
                }
                _ => {
                    graphics::draw(ctx, &self.hydrant, point, 0.0);
                }
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
            x: 190.0,
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
    score: u32
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/leaguespartan-bold.ttf", 20)?;
        let text = graphics::Text::new(ctx, "Beyonce Brawls", &font)?;

        let mut smashables = vec![];

        for x in 0..20 {
            smashables.push(Smashable::new(ctx));
        }

        let s = MainState {
            player: Player::new(ctx),
            text: text,
            smashables: smashables,
            score: 0
        };
        Ok(s)
    }

    pub fn collision(&mut self) {
        if self.player.holding > 4.0 { //magic number

            for s in self.smashables.iter_mut() {

                if self.player.x < s.x + 32.0 &&
                    self.player.x + self.player.w > s.x &&
                    self.player.y < s.y + 64.0 &&
                    self.player.y + self.player.h > s.y {
                        s.active = false;
                        self.score += 1;
                    }
            }
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

        graphics::draw(ctx, &self.text, Point { x: 200.0, y: self.text.height() as f32 }, 0.0)?;
        //graphics::draw(ctx, );

        for s in self.smashables.iter_mut() {
            s.draw(ctx);
        }

        self.player.draw(ctx);

        if self.player.holding > 4.0 {
            graphics::draw(ctx, &self.text, Point { x: 100.0,  y: 300.0}, 0.0)?;
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
    let mut c = conf::Conf::new();
    c.window_title = "Beyonce Brawles!".to_string();
    c.window_width = 400;
    c.window_height = 600;
    c.window_icon = "/b1.png".to_string();

    let ctx = &mut Context::load_from_conf("beyonce_brawles", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
