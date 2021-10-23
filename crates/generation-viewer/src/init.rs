use super::*;

impl GenerationState {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 100.0,
            },
            framebuffer_size: vec2(1.0, 1.0),
            texture: None,
            generator: {
                let mut generator = WorldGenerator::new();
                let gen = &mut generator.generator;

                // Noises
                gen.add_noise("Height", 0, NoiseParameters::new(100.0));
                gen.add_noise("Temperature", 1, NoiseParameters::new(20.0));
                gen.add_noise("Humidity", 2, NoiseParameters::new(20.0));

                // Biomes
                gen.add_generation(
                    Biome::Ocean,
                    TileGeneration::new(0.0, vec![("Height", -0.5)]),
                )
                .unwrap();
                gen.add_generation(
                    Biome::Beach,
                    TileGeneration::new(0.2, vec![("Height", 0.1)]),
                )
                .unwrap();
                gen.add_generation(
                    Biome::Lake,
                    TileGeneration::new(
                        0.1,
                        vec![("Height", 0.3), ("Temperature", 0.1), ("Humidity", 0.5)],
                    ),
                )
                .unwrap();
                gen.add_generation(
                    Biome::Forest,
                    TileGeneration::new(
                        0.0,
                        vec![("Height", 0.3), ("Temperature", -0.1), ("Humidity", -0.1)],
                    ),
                )
                .unwrap();
                gen.add_generation(
                    Biome::Hills,
                    TileGeneration::new(
                        0.0,
                        vec![("Height", 0.6), ("Temperature", -0.2), ("Humidity", -0.2)],
                    ),
                )
                .unwrap();
                gen.add_generation(
                    Biome::MagicForest,
                    TileGeneration::new(
                        0.1,
                        vec![("Height", 0.3), ("Temperature", 0.4), ("Humidity", 0.3)],
                    ),
                )
                .unwrap();

                generator.generate_area(Area {
                    start: Vector2::new(-50.0, -50.0),
                    end: Vector2::new(50.0, 50.0),
                });
                generator
            },
        }
    }
}
