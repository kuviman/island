use super::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "geng::prelude::serde")]
pub struct GenerationConfig {
    pub resolution: u32,
    pub noises: Vec<GenerationNoise>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "geng::prelude::serde")]
pub struct GenerationNoise {
    pub name: String,
    pub properties: MultiNoiseProperties,
    #[serde(skip, default = "geng::ui::Button::new")]
    pub button: geng::ui::Button,
    #[serde(skip)]
    pub show_properties: bool,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            resolution: 64,
            noises: Vec::new(),
        }
    }
}
