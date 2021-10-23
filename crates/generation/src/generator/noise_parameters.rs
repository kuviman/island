use super::*;

pub type Noise = (Box<dyn NoiseFn<[f64; 2]>>, NoiseParameters);

#[derive(Debug, Clone, Copy)]
pub struct NoiseParameters {
    pub scale: f32,
}

impl NoiseParameters {
    pub fn new(scale: f32) -> Self {
        assert!(scale > 0.0, "Noise scale must be positive");
        Self { scale }
    }
}
