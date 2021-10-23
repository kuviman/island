use super::*;

pub struct ChunkGeneration<B, const W: usize, const H: usize> {
    generation: Vec<B>,
}

impl<B, const W: usize, const H: usize> ChunkGeneration<B, W, H> {
    pub(crate) fn new(generation: Vec<B>) -> Self {
        assert!(
            generation.len() == W * H,
            "Generation does not fit in the chunk. Chunk size = ({}, {}), generation length = {}",
            W,
            H,
            generation.len()
        );
        Self { generation }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vector2<usize>, &B)> {
        self.generation
            .iter()
            .enumerate()
            .map(|(index, gen)| (Vector2::new(index % W, index / W), gen))
    }
}
