use geng::ui;
use geng::ui::*;

use super::*;

pub trait Property: Display {
    fn get_value(&self) -> f64;
    fn set_value(&mut self, value: f64);
}

pub struct SliderProperty<'a, T: Property> {
    pub name: &'a str,
    pub value: &'a mut T,
    pub range: RangeInclusive<f64>,
}

impl<'a, T: Property> SliderProperty<'a, T> {
    pub fn slider(
        slider: &'a mut Slider,
        name: &'a str,
        value: &'a mut T,
        range: RangeInclusive<f64>,
        theme: &'a Rc<Theme>,
    ) -> impl Widget + 'a {
        ui::column![
            ui::row![
                Text::new(name, &theme.font, theme.text_size, theme.usable_color),
                Text::new(
                    format!("{}", value),
                    &theme.font,
                    theme.text_size,
                    theme.usable_color
                )
            ],
            slider
                .ui(
                    value.get_value(),
                    range,
                    Box::new(|new_value| value.set_value(new_value))
                )
                .fixed_size(vec2(100.0, 24.0)),
        ]
    }
}

macro_rules! impl_num {
    ($($t:ty),*) => {
        $(
            impl Property for $t {
                fn get_value(&self) -> f64 {
                    *self as f64
                }

                fn set_value(&mut self, value: f64) {
                    *self = value as $t;
                }
            }
        )*
    };
}

impl_num!(usize, u8, u16, u32, u64, u128);
impl_num!(isize, i8, i16, i32, i64, i128);
impl_num!(f32, f64);
