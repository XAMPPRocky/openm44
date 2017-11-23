#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Terrain {
    Plain,
    Forest,
    River,
    Town
}

impl Default for Terrain {
    fn default() -> Self {
        Terrain::Plain
    }
}
