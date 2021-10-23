use super::*;

pub struct GenerationView<'a, T> {
    pub(crate) tile_size: Vector2<f32>,
    pub(crate) chunks: Vec<(
        Vector2<i32>,
        &'a ChunkGeneration<T, CHUNK_WIDTH, CHUNK_HEIGHT>,
    )>,
}

impl<'a, T> GenerationView<'a, T> {
    pub fn tiles(&'a self) -> impl Iterator<Item = (Area<f32>, &'a T)> + 'a {
        let chunk_size = CHUNK_SIZE.map(|x| x as f32);
        self.chunks
            .iter()
            .map(move |&(chunk_pos, chunk_gen)| {
                let tile_start = chunk_pos.map(|x| x as f32) * chunk_size;
                chunk_gen.iter().map(move |(tile_pos, tile)| {
                    let tile_pos = tile_start + tile_pos.map(|x| x as f32) * self.tile_size;
                    (
                        Area {
                            start: tile_pos,
                            end: tile_pos + self.tile_size,
                        },
                        tile,
                    )
                })
            })
            .flatten()
    }
}
