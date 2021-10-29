use geng::ui::*;

use super::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "geng::prelude::serde")]
pub struct GenerationConfig {
    pub resolution: u32,
    pub noises: Vec<GenerationNoiseSer>,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            resolution: 64,
            noises: Vec::new(),
        }
    }
}

pub struct GenerationNoise {
    pub name: String,
    pub properties: MultiNoiseProperties,
    pub sliders: Sliders,
    pub button: Button,
    pub show_properties: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "geng::prelude::serde")]
pub struct GenerationNoiseSer {
    pub name: String,
    pub properties: MultiNoiseProperties,
}

impl GenerationNoiseSer {
    pub fn to_noise(self, theme: &Rc<Theme>) -> GenerationNoise {
        GenerationNoise {
            name: self.name,
            properties: self.properties,
            sliders: Sliders::new(theme),
            button: Button::new(),
            show_properties: false,
        }
    }
}

pub struct Sliders {
    pub value_min: Slider,
    pub value_max: Slider,
    pub scale: Slider,
    pub octaves: Slider,
    pub lacunarity: Slider,
    pub persistance: Slider,
}

impl Sliders {
    fn new(theme: &Rc<Theme>) -> Self {
        Self {
            value_min: Slider::new(theme),
            value_max: Slider::new(theme),
            scale: Slider::new(theme),
            octaves: Slider::new(theme),
            lacunarity: Slider::new(theme),
            persistance: Slider::new(theme),
        }
    }
}

impl GenerationNoise {
    pub fn new(name: &str, properties: MultiNoiseProperties, theme: &Rc<Theme>) -> Self {
        Self {
            name: name.to_owned(),
            properties,
            sliders: Sliders::new(theme),
            button: Button::new(),
            show_properties: false,
        }
    }
}
