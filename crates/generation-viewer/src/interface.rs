use geng::ui;
use geng::ui::*;

use crate::property::{Property, SliderProperty};

use super::*;

pub struct UIState {
    theme: Rc<Theme>,

    resolution: u32,
    noises: Vec<GenerationNoise>,

    events: Vec<UIEvent>,
    panel: ColorBox,
    generate_button: Button,
    resolution_slider: Slider,
    noises_button: Button,
    show_noises: bool,
}

pub enum UIEvent {
    Generate,
}

impl UIState {
    pub fn new(geng: &Geng) -> Self {
        let theme = Rc::new(Theme::default(geng));
        Self {
            resolution: 64,
            noises: vec![GenerationNoise::new(
                "Height",
                MultiNoiseProperties {
                    min_value: -7.0,
                    max_value: 13.0,
                    scale: 100.0,
                    octaves: 3,
                    lacunarity: 2.0,
                    persistance: 0.7,
                },
                &theme,
            )],

            events: Vec::new(),
            panel: ColorBox::new(geng, Color::rgba(0.0, 0.0, 0.0, 0.5)),
            generate_button: Button::new(),
            resolution_slider: Slider::new(&theme),
            noises_button: Button::new(),
            show_noises: false,
            theme,
        }
    }

    pub fn resolution(&self) -> u32 {
        self.resolution
    }

    pub fn events(&mut self) -> Vec<UIEvent> {
        if self.noises_button.clicked() {
            self.show_noises = !self.show_noises;
        }

        for noise in &mut self.noises {
            if noise.button.clicked() {
                noise.show_properties = !noise.show_properties;
            }
        }

        if self.generate_button.clicked() {
            self.events.push(UIEvent::Generate);
        }

        std::mem::take(&mut self.events)
    }

    pub fn noises(&self) -> &Vec<GenerationNoise> {
        &self.noises
    }

    pub fn ui<'a>(&'a mut self) -> impl Widget + 'a {
        ui::stack![
            &mut self.panel,
            ui::column![
                Button::text(&mut self.generate_button, "Generate", &self.theme),
                ui::column![
                    Text::new(
                        "Resolution",
                        &self.theme.font,
                        self.theme.text_size,
                        self.theme.usable_color,
                    )
                    .padding_top(5.0),
                    self.resolution_slider
                        .ui(
                            self.resolution as f64,
                            RESOLUTION_MIN as f64..=RESOLUTION_MAX as f64,
                            Box::new(|new_resolution| {
                                self.resolution = new_resolution as u32;
                            })
                        )
                        .fixed_size(vec2(100.0, 24.0)),
                ],
                ui::column![
                    Button::text(&mut self.noises_button, "Noises", &self.theme),
                    ui::column(if self.show_noises {
                        std::iter::once(Box::new(Text::new(
                            "<New Noise Name>",
                            &self.theme.font,
                            self.theme.text_size,
                            self.theme.usable_color,
                        )) as Box<dyn Widget>)
                        .chain(
                            self.noises
                                .iter_mut()
                                .map(|noise| Box::new(noise.ui(&self.theme)) as Box<dyn Widget>),
                        )
                        .collect()
                    } else {
                        vec![]
                    }),
                ],
            ]
            .padding_top(10.0)
            .padding_right(50.0)
            .align(vec2(0.5, 1.0))
        ]
        .align(vec2(0.0, 1.0))
    }
}

impl GenerationNoise {
    pub fn ui<'a>(&'a mut self, theme: &'a Rc<Theme>) -> impl Widget + 'a {
        let mut widgets =
            vec![Box::new(Button::text(&mut self.button, &self.name, theme)) as Box<dyn Widget>];

        if self.show_properties {
            fn slider<'a, T: Property>(
                slider: &'a mut Slider,
                name: &'a str,
                value: &'a mut T,
                range: RangeInclusive<f64>,
                theme: &'a Rc<Theme>,
            ) -> Box<dyn Widget + 'a> {
                Box::new(SliderProperty::slider(slider, name, value, range, theme))
                    as Box<dyn Widget>
            }

            let properties = vec![
                slider(
                    &mut self.sliders.value_min,
                    "Value Min: ",
                    &mut self.properties.min_value,
                    -100.0..=100.0,
                    theme,
                ),
                slider(
                    &mut self.sliders.value_max,
                    "Value Max: ",
                    &mut self.properties.max_value,
                    -100.0..=100.0,
                    theme,
                ),
                slider(
                    &mut self.sliders.scale,
                    "Scale: ",
                    &mut self.properties.scale,
                    1.0..=500.0,
                    theme,
                ),
                slider(
                    &mut self.sliders.octaves,
                    "Octaves: ",
                    &mut self.properties.octaves,
                    1.0..=10.0,
                    theme,
                ),
                slider(
                    &mut self.sliders.lacunarity,
                    "Lacunarity: ",
                    &mut self.properties.lacunarity,
                    0.0..=10.0,
                    theme,
                ),
                slider(
                    &mut self.sliders.persistance,
                    "Persistance: ",
                    &mut self.properties.persistance,
                    0.0..=1.0,
                    theme,
                ),
            ];
            widgets.extend(properties.into_iter());
        }

        ui::column(widgets)
    }
}
