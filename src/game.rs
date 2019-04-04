use std::fs::File;
use std::time::Duration;

use ggez::event::{self, MouseButton};
use ggez::graphics::{self, Text};
use ggez::{Context, GameResult};
use merge::merge_keys_serde as merge_keys;
use rand::{self, Rng};
use yaml;

use crate::{
    card::{Card, Deck},
    cube::Cube,
    hex::{Coordinate, OFFSET, X_OFFSET},
    map::Map,
    turn::TurnPhase,
    unit::FONT,
    layout::Layout,
    point::Point,
};


use super::{WIDTH, HEIGHT};

#[derive(Default)]
pub struct Game {
    map: Map,
    layout: Layout,
    src_position: Option<Coordinate>,
    allied_hand: Vec<Card>,
    axis_hand: Vec<Card>,
    deck: Deck,
    /// Whose turn it is, `false == Axis` / `true == Allies`
    turn: bool,
    phase: TurnPhase,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        Ok(Game {
            map: {
                let f = File::open("./resources/maps/plains/pegasus_bridge.yml")?;
                let value = merge_keys(yaml::from_reader(f).unwrap()).unwrap();

                Map::from_data(yaml::from_value(value).unwrap())
            },
            deck: {
                let f = File::open("./resources/deck/standard.yml")?;
                let value = merge_keys(yaml::from_reader(f).unwrap()).unwrap();

                yaml::from_value(value).unwrap()
            },
            ..Self::default()
        })
    }

    fn handle_map_right_click(&mut self, position: Coordinate) {
        if self.src_position.is_none() {
            return
        }

        let src_position = self.src_position.unwrap().clone();

        self.map.change_hexs((src_position, position), |this, src, dest| {
            src.selected = false;

            if let Some(unit) = src.unit {
                let fringes = this.can_fire_at(unit, &src.position);

                if !fringes.contains(&dest.position) {
                    return;
                }

                if dest.unit.is_some() {
                    let unit = dest.unit.as_mut().unwrap();
                    let mut rng = rand::thread_rng();
                    for _ in 0..dest.dice.unwrap() {
                        let dice = rng.gen();
                        if unit.takes_causality(dice) {
                            unit.current_health -= 1;

                            if unit.current_health == 0 {
                                unit.destroy = true;
                            }
                        }
                    }
                } else {
                    dest.unit = src.remove_unit();
                }
            }

        });

        self.map.reset_view();
    }

    fn handle_map_left_click(&mut self, position: Coordinate) {
        self.map.reset_view();
        let mut has_unit = false;

        if let Some(hex) = self.map.get_mut(&position) {
            hex.selected = true;
            if hex.selected {
                self.src_position = Some(hex.position);
                has_unit = hex.unit.is_some();
            }
        }

        if let (Some(src), true) = (self.src_position, has_unit) {
            let unit = self.map[&src].unit.unwrap();
            let fringes = self.map.can_move_to(unit.movement(), &src);
            let fire_fringes = self.map.can_fire_at(unit, &src);

            for dest in fringes {
                let distance = Map::hex_distance(src, dest);
                if distance != 0 {
                    self.map.get_mut(&dest).unwrap().distance = Some(distance);
                }
            }

            for dest in fire_fringes {
                let hex = self.map.get_mut(&dest).unwrap();
                let distance = Map::hex_distance(src, dest).saturating_sub(1);
                let dice = {
                    let max = unit.range_effectiveness()[distance as usize];
                    hex.reduce_dice(unit.unit_type, max)
                };

                if dice != 0 && unit.faction != hex.unit.unwrap().faction {
                    hex.dice = Some(dice);
                }
            }
        }
    }

}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_resolution(ctx, WIDTH, HEIGHT);
        graphics::clear(ctx);
        self.map.draw(ctx)?;
        graphics::present(ctx);

        Ok(())
    }

    fn mouse_motion_event(&mut self,
                          ctx: &mut Context,
                          _state: event::MouseState,
                          x: i32,
                          y: i32,
                          _xrel: i32,
                          _yrel: i32)
    {
        self.map.reset_view();
        let mut has_unit = false;
        let position = self.layout.pixel_to_hex(Point::new(x as f64, y as f64));
        let position = (position.q() as i8, position.r() as i8);


        graphics::set_color(ctx, ::hsl::Hsl::WHITE.into()).unwrap();
        let mut text = Text::new(ctx, &format!("({}, {})", x, y), &FONT).unwrap();
        text.set_filter(graphics::FilterMode::Nearest);
        graphics::draw(ctx, &text, graphics::Point2::new(x as f32, y as f32), 0.).unwrap();

        if let Some(hex) = self.map.get_mut(&position) {
            hex.selected = true;
            if hex.selected {
                self.src_position = Some(hex.position);
                has_unit = hex.unit.is_some();
            }
        }

    }

    fn mouse_button_down_event(&mut self,
                               _ctx: &mut Context,
                               button: MouseButton,
                               x: i32,
                               y: i32) {

        let position = pixel_to_hex(x as f32, y as f32);
        println!("{:?}", position);

        match button {
            MouseButton::Left => self.handle_map_left_click(position),
            MouseButton::Right => self.handle_map_right_click(position),
            _ => {}
        }
    }
}

fn pixel_to_hex(x: f32, y: f32) -> (i8, i8) {
    let x = x - X_OFFSET;
    let y = y - OFFSET;
    println!("{:?}", (x, y));
    let q = (3f32.sqrt()/3. * x  - 1./3. * y) / OFFSET;
    let r = (                      2./3. * y) / OFFSET;
    println!("{:?}", (q, r));
    unimplemented!()
    // Cube::new(q,  -q-r, r).round().to_oddr()

    /*
    let q = (f32::sqrt(3.)/3. * x  -  1./3. * y) / OFFSET;
    let r = (                        2./3. * y) / OFFSET;
    let mut rx = f32::round(q);
    let mut ry = f32::round(-q-r);
    let mut rz = f32::round(r);

    let x_diff = f32::abs(rx - q);
    let y_diff = f32::abs(ry - (-q-r));
    let z_diff = f32::abs(rz - r);

    if x_diff > y_diff && x_diff > z_diff {
        rx = -ry-rz;
    } else if y_diff > z_diff {
        ry = -rx-rz;
    } else {
        rz = -rx-ry;
    }

    let col = rx + (rz - (rz as i32 & 1) as f32) / 2.;
    let row = rz;

    (col as i8, row as i8)
    */
}
