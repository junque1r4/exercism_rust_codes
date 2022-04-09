use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[derive(Debug, PartialEq, IntoEnumIterator, IntEnum, Copy, Clone, Ord, PartialOrd, Eq)]
#[repr(usize)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl ResistorColor {
    pub fn sort() -> Vec<ResistorColor> {
        let mut colors = ResistorColor::into_enum_iter().collect::<Vec<ResistorColor>>();
        colors.sort();
        colors
    }


    pub fn show_value(color: ResistorColor) -> usize {
        color as usize
    }

    pub fn show_string(color: usize) -> String {
        match ResistorColor::from_int(color) {
            Ok(color) => format!("{:?}", color),
            Err(_) => "value out of range".to_string(),
        }
    }

    pub fn ordered() -> Vec<ResistorColor> {
        ResistorColor::sort()
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    ResistorColor::show_value(_color)
}

pub fn value_to_color_string(value: usize) -> String {
    ResistorColor::show_string(value)
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::sort()
}
