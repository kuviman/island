use std::ops::RangeInclusive;

use super::*;

#[derive(Debug, Clone)]
pub struct TileGeneration {
    pub parameter_values: HashMap<GenerationParameter, ParameterRange>,
}

#[derive(Debug, Clone, Copy)]
pub struct ParameterRange {
    pub(crate) min: f32,
    pub(crate) max: f32,
}

impl From<RangeInclusive<f32>> for ParameterRange {
    fn from(range: RangeInclusive<f32>) -> Self {
        Self {
            min: *range.start(),
            max: *range.end(),
        }
    }
}

impl TileGeneration {
    pub fn new(parameter_values: Vec<(&str, impl Into<ParameterRange>)>) -> Self {
        Self {
            parameter_values: parameter_values
                .into_iter()
                .map(|(name, value)| (name.to_owned(), value.into()))
                .collect(),
        }
    }
}
