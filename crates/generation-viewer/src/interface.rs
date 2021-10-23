use geng::ui;
use geng::ui::*;

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
            noises: vec![GenerationNoise {
                name: "Height".to_owned(),
                properties: MultiNoiseProperties {
                    min_value: -7.0,
                    max_value: 13.0,
                    scale: 100.0,
                    octaves: 3,
                    lacunarity: 2.0,
                    persistance: 0.7,
                },
                button: Button::new(),
                show_properties: false,
            }],

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
        .align(vec2(1.0, 1.0))
    }
}

impl GenerationNoise {
    fn ui<'a>(&'a mut self, theme: &'a Rc<Theme>) -> impl Widget + 'a {
        let properties = vec![
            ("Value Min: ", self.properties.min_value.to_string()),
            ("Value Max: ", self.properties.max_value.to_string()),
            ("Scale: ", self.properties.scale.to_string()),
            ("Octaves: ", self.properties.octaves.to_string()),
            ("Lacunarity: ", self.properties.lacunarity.to_string()),
            ("Persistance: ", self.properties.persistance.to_string()),
        ];

        let mut widgets =
            vec![Box::new(Button::text(&mut self.button, &self.name, theme)) as Box<dyn Widget>];

        if self.show_properties {
            widgets.push(Box::new(ui::column(
                properties
                    .into_iter()
                    .map(|(name, value)| {
                        Box::new(ui::row![
                            Text::new(name, &theme.font, theme.text_size, theme.usable_color),
                            Text::new(value, &theme.font, theme.text_size, theme.usable_color)
                        ]) as Box<dyn Widget>
                    })
                    .collect(),
            )) as Box<dyn Widget>);
        }

        ui::column(widgets)
    }
}
