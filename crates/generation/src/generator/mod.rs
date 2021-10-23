use std::collections::HashMap;

use super::*;

mod chunk;
mod generator;
mod multi_noise;
mod tile_generation;
mod view;

use chunk::*;
use generator::*;
pub use multi_noise::*;
pub use tile_generation::*;
pub use view::*;

const CHUNK_WIDTH: usize = 50;
const CHUNK_HEIGHT: usize = 50;
const CHUNK_SIZE: Vector2<usize> = Vector2::new(CHUNK_WIDTH, CHUNK_HEIGHT);

pub struct WorldGenerator<T> {
    pub generator: Generator<T>,
    tile_size: Vector2<f32>,
    chunks: HashMap<Vector2<i32>, ChunkGeneration<T, CHUNK_WIDTH, CHUNK_HEIGHT>>,
}

impl<T> WorldGenerator<T> {
    pub fn new() -> Self {
        Self {
            generator: Generator::new(),
            tile_size: Vector2::new(2.0, 2.0),
            chunks: HashMap::new(),
        }
    }

    pub fn tile_size(&self) -> Vector2<f32> {
        self.tile_size
    }

    fn tile_to_chunk_pos(&self, tile_position: Vector2<f32>) -> Vector2<i32> {
        Vector2::new(
            (tile_position.x / (CHUNK_WIDTH as f32 * self.tile_size.x)).floor() as i32,
            (tile_position.y / (CHUNK_HEIGHT as f32 * self.tile_size.y)).floor() as i32,
        )
    }

    /// Change the generation scale. Clears all previous generations.
    pub fn set_scale(&mut self, new_scale: GenerationScale) {
        self.chunks.clear();
        self.tile_size = new_scale.tile_size();
    }
}

impl<T: Copy> WorldGenerator<T> {
    /// Generate a rectangular area and return its view. The generation might be bigger
    /// (but not smaller) than requested because it generates chunks.
    pub fn generate_area(&mut self, area: Area<f32>) -> GenerationView<T> {
        let start = self.tile_to_chunk_pos(area.start);
        let end = self.tile_to_chunk_pos(area.end);

        for y in start.y..=end.y {
            for x in start.x..=end.x {
                let chunk_pos = Vector2::new(x, y);
                self.chunks
                    .entry(chunk_pos)
                    .or_insert_with(|| self.generator.generate_chunk(chunk_pos, self.tile_size));
            }
        }

        self.view(area)
    }

    /// View the generated area.
    pub fn view(&self, area: Area<f32>) -> GenerationView<T> {
        let start = self.tile_to_chunk_pos(area.start);
        let end = self.tile_to_chunk_pos(area.end);

        let dx = end.x - start.x + 1;
        let dy = end.y - start.y + 1;
        if dx <= 0 || dy <= 0 {
            // Negative area
            return GenerationView {
                chunk_size: CHUNK_SIZE,
                tile_size: self.tile_size,
                chunks: Vec::new(),
            };
        }

        let mut visible_chunks = Vec::with_capacity((dx * dy) as usize);

        for y in start.y..=end.y {
            for x in start.x..=end.x {
                let chunk_pos = Vector2::new(x, y);
                if let Some(chunk) = self.chunks.get(&chunk_pos) {
                    visible_chunks.push((chunk_pos, chunk));
                }
            }
        }

        GenerationView {
            chunk_size: CHUNK_SIZE,
            tile_size: self.tile_size,
            chunks: visible_chunks,
        }
    }
}

/// Describes the scale of the generation.
pub enum GenerationScale {
    /// Bigger tile size -> faster generation
    TileSize { x: f32, y: f32 },
}

impl GenerationScale {
    fn tile_size(self) -> Vector2<f32> {
        match self {
            GenerationScale::TileSize { x, y } => Vector2::new(x, y),
        }
    }
}
