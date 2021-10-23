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
    scale: GenerationScale,
    chunks: HashMap<Vector2<i32>, ChunkGeneration<T, CHUNK_WIDTH, CHUNK_HEIGHT>>,
}

impl<T> WorldGenerator<T> {
    pub fn new() -> Self {
        Self {
            generator: Generator::new(),
            scale: GenerationScale::TileSize { x: 1.0, y: 1.0 },
            chunks: HashMap::new(),
        }
    }

    fn tile_to_chunk_pos(tile_position: Vector2<f32>, tile_size: Vector2<f32>) -> Vector2<i32> {
        Vector2::new(
            (tile_position.x / (CHUNK_WIDTH as f32 * tile_size.x)).floor() as i32,
            (tile_position.y / (CHUNK_HEIGHT as f32 * tile_size.y)).floor() as i32,
        )
    }

    /// Change the generation scale. Clears all previous generations
    /// if scale is different from previous.
    pub fn set_scale(&mut self, new_scale: GenerationScale) {
        if self.scale == new_scale {
            return;
        }

        self.chunks.clear();
        self.scale = new_scale;
    }
}

impl<T: Copy> WorldGenerator<T> {
    /// Generate a rectangular area and return its view. The generation might be bigger
    /// (but not smaller) than requested because it generates chunks.
    pub fn generate_area(&mut self, area: Area<f32>) -> GenerationView<T> {
        let tile_size = self.scale.tile_size();
        let start = Self::tile_to_chunk_pos(area.start, tile_size);
        let end = Self::tile_to_chunk_pos(area.end, tile_size);

        for y in start.y..=end.y {
            for x in start.x..=end.x {
                let chunk_pos = Vector2::new(x, y);
                self.chunks
                    .entry(chunk_pos)
                    .or_insert_with(|| self.generator.generate_chunk(chunk_pos, tile_size));
            }
        }

        self.view(area)
    }

    /// View the generated area.
    pub fn view(&self, area: Area<f32>) -> GenerationView<T> {
        let tile_size = self.scale.tile_size();
        let start = Self::tile_to_chunk_pos(area.start, tile_size);
        let end = Self::tile_to_chunk_pos(area.end, tile_size);

        let dx = end.x - start.x + 1;
        let dy = end.y - start.y + 1;
        if dx <= 0 || dy <= 0 {
            // Negative area
            return GenerationView {
                chunk_size: CHUNK_SIZE,
                tile_size,
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
            tile_size,
            chunks: visible_chunks,
        }
    }
}

/// Describes the scale of the generation.
#[derive(Clone, Copy, PartialEq)]
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
