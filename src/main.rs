#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_yaml as yaml;
extern crate yaml_merge_keys as merge;
extern crate ggez;

pub mod faction;
pub mod feature;
pub mod hex;
pub mod map;
pub mod terrain;
pub mod unit;
pub mod hsl;

use std::fs::File;
use std::time::Duration;

use map::{Map, MapData};
use hex::{OFFSET, SIZE};
use yaml::Value;
use feature::Feature;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::{Context, GameResult};
use ggez::graphics;

const HEIGHT: u32 = SIZE * 14;
const WIDTH: u32 = (SIZE * 24);

struct MainState {
    map: Map,
    src_position: Option<(i8, i8)>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(MainState {
            map: {
                let file = File::open("./maps/plains/pegasus_bridge.yml")?;
                let value: Value = yaml::from_reader(file).unwrap();
                let merged = merge::merge_keys_serde(value).unwrap();
                let data: MapData = yaml::from_value(merged).unwrap();

                Map::from_data(data)
            },
            src_position: None,
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.map.draw(ctx)?;
        graphics::present(ctx);

        Ok(())
    }

    fn mouse_button_down_event(&mut self,
                               button: MouseButton,
                               x: i32,
                               y: i32) {
        let x = x as f32;
        let y = y as f32;

        let q = (x * f32::sqrt(3.)/3. - y / 3.) / OFFSET;
        let r = y * 2./3. / OFFSET;
        let y = -q - r;

        let mut rq = f32::round(q);
        let mut rr = f32::round(r);
        let ry = f32::round(y);

        let qdiff = f32::abs(rq - q);
        let rdiff = f32::abs(rr - r);
        let ydiff = f32::abs(ry - y);

        if qdiff > ydiff && qdiff > rdiff {
            rq = -ry - rr;
        } else {
            rr = -rq - ry;
        }

        let position = (rq as i8, rr as i8 -1);
        self.map.reset_view();

        match button {
            MouseButton::Left => {

                let mut has_unit = false;

                if let Some(hex) = self.map.get_mut(&position) {
                    hex.selected = true;
                    if hex.selected {
                        self.src_position = Some(hex.position);
                        has_unit = hex.unit.is_some();
                    }
                }

                if let (Some(src), true) = (self.src_position, has_unit) {

                    let fringes = {
                        let hex = self.map.get(&src).unwrap();
                        self.map.can_reach(hex.unit.unwrap().movement(), &hex.position)
                    };

                    for dest in fringes {
                        let distance = Map::hex_distance(src, dest);
                        if distance != 0 {
                            self.map.get_mut(&dest).unwrap().distance = Some(distance);
                        }
                    }
                }

            }

            MouseButton::Right => {
                if self.src_position.is_none() {
                    return
                }

                let src_position = self.src_position.unwrap().clone();

                self.map.change_hexs((src_position, position), |this, src, dest| {
                    src.selected = false;

                    if let Some(unit) = src.unit {
                        let fringes = this.can_reach(unit.movement(), &src.position);
                        if !fringes.contains(&dest.position) {
                            return;
                        }

                        if let Some(mut other) = dest.unit {
                            other.current_health -= 1;
                            if other.current_health == 0 {
                                dest.remove_unit();
                            } else {
                                dest.unit = Some(other);
                            }
                        } else {
                            if dest.terrain.is_walkable() {
                                dest.unit = src.remove_unit();
                            }
                        }
                    }

                    this.reset_view();
                });
            }
            _ => {}
        }
    }
}

fn main() {
    run().unwrap();
}

fn run() -> GameResult<()> {
    let mut c = conf::Conf::new();
    c.window_width = WIDTH;
    c.window_height = HEIGHT;
    c.vsync = true;

    let mut ctx = Context::load_from_conf("helloworld", "ggez", c).unwrap();
    let mut state = MainState::new(&mut ctx).unwrap();

    event::run(&mut ctx, &mut state)?;

    Ok(())
}
