use super::*;

pub(crate) type Tile<T> = Option<T>;

pub struct ChunkGeneration<T, const W: usize, const H: usize> {
    generation: Vec<Tile<T>>,
}

impl<T, const W: usize, const H: usize> ChunkGeneration<T, W, H> {
    pub(crate) fn new(generation: Vec<Tile<T>>) -> Self {
        assert!(
            generation.len() == W * H,
            "Generation does not fit in the chunk. Chunk size = ({}, {}), generation length = {}",
            W,
            H,
            generation.len()
        );
        Self { generation }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vector2<usize>, &Tile<T>)> {
        self.generation
            .iter()
            .enumerate()
            .map(|(index, gen)| (Vector2::new(index % W, index / W), gen))
    }
}
