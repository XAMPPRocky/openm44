extern crate openm44;
extern crate ggez;

use openm44::{
    HEIGHT,
    WIDTH,
    game::Game,
};

use ggez::{conf, Context, event, GameResult};

fn main() {
    run().unwrap();
}

fn run() -> GameResult<()> {
    let c = conf::Conf {
        window_mode: conf::WindowMode {
            width: WIDTH - 1,
            height: HEIGHT - 1,
            ..conf::WindowMode::default()
        },
        ..conf::Conf::new()
    };

    let mut ctx = Context::load_from_conf("helloworld", "ggez", c)?;
    let mut state = Game::new(&mut ctx)?;

    event::run(&mut ctx, &mut state)?;

    Ok(())
}
