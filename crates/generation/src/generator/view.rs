use super::*;

pub struct GenerationView<'a, T> {
    pub(crate) tile_size: Vector2<f32>,
    pub(crate) chunks: Vec<(
        Vector2<i32>,
        &'a ChunkGeneration<T, CHUNK_WIDTH, CHUNK_HEIGHT>,
    )>,
}

impl<'a, T> GenerationView<'a, T> {
    /// Get an iterator over all tiles.
    /// The tile is returned as a tuple (tile_area, &tile)
    /// where tile_area is the **global** position and size of the tile
    /// (or the area that this tile covers),
    /// and tile is the generated tile.
    pub fn tiles(&'a self) -> impl Iterator<Item = (Area<f32>, &'a T)> + 'a {
        let chunk_size = CHUNK_SIZE.map(|x| x as f32);
        self.chunks()
            .map(move |(chunk_pos, tiles)| {
                let tile_start = chunk_size * chunk_pos.map(|x| x as f32);
                tiles.map(move |(position, tile)| {
                    let position = position.map(|x| x as f32) * self.tile_size;
                    (
                        Area {
                            start: tile_start,
                            end: tile_start + position,
                        },
                        tile,
                    )
                })
            })
            .flatten()
    }

    /// Get an iterator over all chunks with their positions.
    /// The tile is returned as a tuple (tile_pos, &tile)
    /// where tile_pos is the **local** position of the tile
    /// (or the area that this tile covers),
    /// and tile is the generated tile.
    pub fn chunks(
        &'a self,
    ) -> impl Iterator<Item = (Vector2<i32>, impl Iterator<Item = (Vector2<usize>, &'a T)>)> {
        self.chunks
            .iter()
            .map(|&(chunk_pos, chunk_gen)| (chunk_pos, chunk_gen.iter()))
    }
}
