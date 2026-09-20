use crossterm::style::Color;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub name: &'static str,
    pub color: Color,
}

pub const THEMES: &[Theme] = &[
    Theme {
        name: "Pastel Pink",
        color: Color::Rgb { r: 235, g: 180, b: 205 }, // Matches reference image pink
    },
    Theme {
        name: "Lavender",
        color: Color::Rgb { r: 200, g: 175, b: 240 },
    },
    Theme {
        name: "Cyan",
        color: Color::Rgb { r: 120, g: 220, b: 245 },
    },
    Theme {
        name: "Emerald Green",
        color: Color::Rgb { r: 130, g: 230, b: 165 },
    },
    Theme {
        name: "Amber",
        color: Color::Rgb { r: 255, g: 185, b: 90 },
    },
    Theme {
        name: "Pure White",
        color: Color::Rgb { r: 240, g: 240, b: 245 },
    },
];
