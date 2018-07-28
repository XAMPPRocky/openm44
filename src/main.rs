extern crate openm44;
extern crate ggez;

use openm44::game::Game;
use openm44::hex::SIZE;

use ggez::{conf, Context, event, GameResult};

const HEIGHT: u32 = SIZE * 14;
const WIDTH: u32 = (SIZE * 24);

fn main() {
    run().unwrap();
}

fn run() -> GameResult<()> {
    let c = conf::Conf {
        window_width: WIDTH,
        window_height: HEIGHT,
        vsync: true,
        ..conf::Conf::new()
    };

    let mut ctx = Context::load_from_conf("helloworld", "ggez", c)?;
    let mut state = Game::new(&mut ctx)?;

    event::run(&mut ctx, &mut state)?;

    Ok(())
}
