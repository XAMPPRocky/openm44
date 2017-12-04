use std::fs::File;
use std::time::Duration;

use merge;
use yaml;
use rand::{self, Rng};

use map::{Map, MapData};
use hex::OFFSET;
use yaml::Value;

use ggez::event::{self, MouseButton};
use ggez::{Context, GameResult};
use ggez::graphics;

pub struct Game {
    map: Map,
    src_position: Option<(i8, i8)>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Game {
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

        match button {
            MouseButton::Left => {
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

                    for dest in fringes {
                        let distance = Map::hex_distance(src, dest);
                        if distance != 0 {
                            self.map.get_mut(&dest).unwrap().distance = Some(distance);
                        }
                    }

                    let fire_fringes = self.map.can_fire_at(unit, &src);

                    for dest in fire_fringes {
                        let hex = self.map.get_mut(&dest).unwrap();

                        let distance = Map::hex_distance(src, dest) as usize;
                        let distance = distance.saturating_sub(1);

                        let dice = unit.range_effectiveness()[distance];
                        let dice = hex.reduce_dice(unit.unit_type, dice);

                        if dice != 0 && unit.faction != hex.unit.unwrap().faction {
                            hex.dice = Some(dice);
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
            _ => {}
        }
    }
}


