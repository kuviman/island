use super::*;

impl GenerationState {
    pub fn new(geng: &Geng) -> Self {
        let mut state = Self {
            framebuffer_size: vec2(1.0, 1.0),
            renderer: Renderer::new(geng),
            ui_state: UIState::new(geng),
            ui_controller: geng::ui::Controller::new(),
            dragging: None,
            generator: {
                let mut generator = WorldGenerator::new();
                let gen = &mut generator.generator;

                // Noises
                gen.add_noise(
                    "Height",
                    0,
                    MultiNoiseProperties {
                        min_value: -7.0,
                        max_value: 13.0,
                        scale: 100.0,
                        octaves: 3,
                        lacunarity: 2.0,
                        persistance: 0.5,
                    },
                );
                gen.add_noise(
                    "Humidity",
                    1,
                    MultiNoiseProperties {
                        min_value: 0.0,
                        max_value: 1.0,
                        scale: 50.0,
                        octaves: 1,
                        lacunarity: 1.0,
                        persistance: 1.0,
                    },
                );
                gen.add_noise(
                    "Magic",
                    2,
                    MultiNoiseProperties {
                        min_value: 0.0,
                        max_value: 1.0,
                        scale: 50.0,
                        octaves: 1,
                        lacunarity: 1.0,
                        persistance: 1.0,
                    },
                );

                // Biomes
                gen.add_generation(
                    Biome::Ocean,
                    TileGeneration::new(vec![("Height", -7.0..=0.0)]),
                )
                .unwrap();
                gen.add_generation(
                    Biome::Beach,
                    TileGeneration::new(vec![("Height", 0.0..=1.0)]),
                )
                .unwrap();
                gen.add_generation(
                    Biome::Forest,
                    TileGeneration::new(vec![("Height", 1.0..=9.0)]),
                )
                .unwrap();
                gen.add_generation(
                    Biome::Lake,
                    TileGeneration::new(vec![("Height", 2.0..=8.0), ("Humidity", 0.9..=1.0)]),
                )
                .unwrap();
                gen.add_generation(
                    Biome::Hills,
                    TileGeneration::new(vec![("Height", 9.0..=13.0)]),
                )
                .unwrap();
                gen.add_generation(
                    Biome::MagicForest,
                    TileGeneration::new(vec![("Height", 2.0..=9.0), ("Magic", 0.8..=1.0)]),
                )
                .unwrap();

                generator
            },
        };
        state.generate_view();
        state
    }
}
