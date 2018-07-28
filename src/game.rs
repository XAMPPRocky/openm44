use std::fs::File;
use std::time::Duration;

use ggez::event::{self, MouseButton};
use ggez::graphics;
use ggez::{Context, GameResult};
use merge::merge_keys_serde as merge_keys;
use rand::{self, Rng};
use yaml;

use map::Map;
use hex::{Coordinate, OFFSET, X_OFFSET};
use card::{Card, Deck};
use turn::TurnPhase;

#[derive(Default)]
pub struct Game {
    map: Map,
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
                let f = File::open("./maps/plains/pegasus_bridge.yml")?;
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
        let q = (x * 3f32.sqrt() / 3. - y / 3.) / OFFSET;
        let r = y * 2. / 3. / OFFSET;

        let position = (q as i8, r as i8);
        println!("{:?}", position);

        match button {
            MouseButton::Left => self.handle_map_left_click(position),
            MouseButton::Right => self.handle_map_right_click(position),
            _ => {}
        }
    }
}
