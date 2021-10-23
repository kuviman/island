use super::*;

pub type GenerationParameter = String;

pub struct Generator<T> {
    noises: HashMap<GenerationParameter, MultiNoise>,
    generations: Vec<(T, TileGeneration)>,
}

impl<T> Generator<T> {
    pub(crate) fn new() -> Self {
        Self {
            noises: HashMap::new(),
            generations: vec![],
        }
    }

    pub fn add_noise(
        &mut self,
        name: &str,
        noise_seed: u32,
        noise_parameters: MultiNoiseProperties,
    ) {
        self.noises.insert(
            name.to_owned(),
            MultiNoise::new(noise_seed, noise_parameters),
        );
    }

    pub fn add_generation(
        &mut self,
        generation: T,
        gen_parameters: TileGeneration,
    ) -> Result<(), GenerationError> {
        let parameters = gen_parameters.parameter_values.len();
        let noises = self.noises.len();
        if parameters > noises {
            return Err(GenerationError::TooManyParameters { parameters, noises });
        }

        for parameter in gen_parameters.parameter_values.keys() {
            if !self.noises.contains_key(parameter) {
                return Err(GenerationError::NoiseNotFound {
                    name: parameter.to_owned(),
                });
            }
        }

        self.generations.push((generation, gen_parameters));
        Ok(())
    }
}

impl<T: Copy> Generator<T> {
    pub(crate) fn generate_chunk(
        &self,
        chunk_pos: Vector2<i32>,
        tile_size: Vector2<f32>,
    ) -> ChunkGeneration<T, CHUNK_WIDTH, CHUNK_HEIGHT> {
        let chunk_size = CHUNK_SIZE.map(|x| x as f32);
        let tile_start = chunk_pos.map(|x| x as f32) * chunk_size * tile_size;

        let mut generation = Vec::with_capacity(CHUNK_WIDTH * CHUNK_HEIGHT);
        for y in 0..CHUNK_HEIGHT {
            for x in 0..CHUNK_WIDTH {
                let position = Vector2::new(x, y).map(|x| x as f32);
                let position = tile_start + position * tile_size;
                let gen = self.generate(position);
                generation.push(gen);
            }
        }

        ChunkGeneration::new(generation)
    }

    fn generate(&self, position: Vector2<f32>) -> Option<T> {
        let noise_values: HashMap<GenerationParameter, f32> = self
            .noises
            .iter()
            .map(|(parameter, noise)| (parameter.to_owned(), noise.get(position)))
            .collect();

        self.generations
            .iter()
            .filter_map(|(gen, generation)| {
                let mut total_score = 0.0;
                for (parameter, range) in &generation.parameter_values {
                    let value = noise_values[parameter];
                    let score = (value - range.min).min(range.max - value);
                    if score < 0.0 {
                        return None;
                    }
                    total_score += score;
                }
                Some((gen, total_score))
            })
            .min_by(|(_, score1), (_, score2)| score1.partial_cmp(score2).unwrap())
            .map(|(gen, _)| *gen)
    }
}

#[derive(Debug)]
pub enum GenerationError {
    TooManyParameters { parameters: usize, noises: usize },
    NoiseNotFound { name: GenerationParameter },
}

impl std::fmt::Display for GenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerationError::TooManyParameters { parameters, noises } => write!(
                f,
                "Too many generation parameters ({}) or not enough noises ({})",
                parameters, noises
            ),
            GenerationError::NoiseNotFound { name } => {
                write!(f, "Parameter named {} not found", name)
            }
        }
    }
}
