type PropertyMap<'a, T> = HashMap<&'a str, T>;

struct Game<'a> {
    name: &'a str,
    units: PropertyMap<'a, Unit>,
    terrain: PropertyMap<'a, Terrain>,
    dice: PropertyMap<'a, Dice>
}

struct Unit<'a> {
    battle_threshold: u8,
    effectiveness: &'a [u8],
    health: u8,
    movement: u8,
    range: u8,
}

struct Unit<'a> {
    battle_threshold: u8,
    effectiveness: &'a [u8],
    health: u8,
    movement: u8,
    range: u8,
}
