use super::*;

#[derive(Debug, Clone)]
pub struct TileGeneration {
    pub offset: f32,
    pub parameter_values: HashMap<GenerationParameter, f32>,
}

impl TileGeneration {
    pub fn new(offset: f32, parameter_values: Vec<(&str, f32)>) -> Self {
        // TODO: check every parameter to be in range -1..1 and offset to be in range 0..1
        Self {
            offset,
            parameter_values: parameter_values
                .into_iter()
                .map(|(name, value)| (name.to_owned(), value))
                .collect(),
        }
    }
    pub fn calculate_score(
        &self,
        position: Vector2<f32>,
        noises: &HashMap<GenerationParameter, Noise>,
    ) -> f32 {
        let parameters_count = self.parameter_values.len();
        let noises_count = noises.len();
        assert!(parameters_count <= noises_count);

        let mut score = 0.0;
        for (parameter_name, &parameter_value) in &self.parameter_values {
            let (noise, noise_parameters) = &noises[parameter_name];
            let noise_value = noise.get([
                position.x as f64 / noise_parameters.scale as f64,
                position.y as f64 / noise_parameters.scale as f64,
            ]) as f32;
            score += 2.0 - (parameter_value - noise_value).abs();
        }
        score / parameters_count as f32 - self.offset
    }
}
