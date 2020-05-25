use crate::color::{Color, GREY};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Glyph {
    pub ch: char,
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub render_order: i32,
}

impl Glyph {
    pub fn from(ch: char, foreground: Option<Color>, background: Option<Color>) -> Self {
        Glyph {
            ch,
            foreground,
            background,
            render_order: 0,
        }
    }

    pub fn greyscale(&self) -> Self {
        Glyph {
            ch: self.ch,
            foreground: self.foreground,
            background: None,
            render_order: self.render_order,
        }
    }
}
