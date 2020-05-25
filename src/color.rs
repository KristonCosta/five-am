#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
pub const BLUE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
pub const GREY: Color = Color {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 1.0,
};
pub const RED: Color = Color {
    r: 0.9,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};

pub const TAN: Color = Color {
    r: 232.0 / 255.0,
    g: 166.0 / 255.0,
    b: 80.0 / 255.0,
    a: 1.0,
};
