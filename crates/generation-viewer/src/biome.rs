use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Biome {
    Ocean,
    Beach,
    Lake,
    Forest,
    Hills,
    MagicForest,
}

impl Biome {
    pub fn color(&self) -> Color<f32> {
        match self {
            Biome::Ocean => Color::rgb(0.0, 0.0, 0.7),
            Biome::Beach => Color::rgb(0.8, 0.8, 0.0),
            Biome::Lake => Color::rgb(0.1, 0.1, 0.8),
            Biome::Forest => Color::rgb(0.0, 0.7, 0.0),
            Biome::Hills => Color::rgb(0.7, 0.7, 0.7),
            Biome::MagicForest => Color::rgb(0.3, 0.0, 0.5),
        }
    }
}
